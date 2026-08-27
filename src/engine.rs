use crate::{
    archive::Archive,
    config::Config,
    forge::Forge,
    github::{cached_repositories, GitHub, RepoResponse},
    gitmirror,
    model::{ItemSnapshot, Label},
    state::State,
};
use anyhow::{anyhow, Context, Result};
use chrono::Utc;
use serde::Serialize;
use std::{collections::HashMap, time::Instant};

#[derive(Debug, Default, Serialize)]
pub struct RunReport {
    pub discovered: usize,
    pub synchronized: usize,
    pub failed: usize,
    pub issues: usize,
    pub comments: usize,
    pub dry_run: bool,
    pub started_at: String,
    pub duration_ms: u128,
    pub errors: Vec<RepoError>,
}
#[derive(Debug, Serialize)]
pub struct RepoError {
    pub repository: String,
    pub error: String,
}

struct SyncResources<'a> {
    config: &'a Config,
    github: &'a GitHub,
    forge: &'a Forge,
    state: &'a State,
    archive: &'a Archive,
    source_token: &'a str,
    target_token: &'a str,
    target_git_user: &'a str,
}

pub fn run_once(config: &Config) -> Result<RunReport> {
    let started = Instant::now();
    let mut report = RunReport {
        started_at: Utc::now().to_rfc3339(),
        dry_run: config.sync.dry_run,
        ..Default::default()
    };
    // A plan must be observational: it must not create a state directory,
    // poison mappings with placeholder IDs, update discovery cache/audit data,
    // or write an archive.
    let state = if config.sync.dry_run {
        State::open_read_only(&config.sync.state_dir)?
    } else {
        State::open(&config.sync.state_dir)?
    };
    let source_token = config.source_token()?;
    let target_token = config.target_token()?;
    let github = GitHub::new(
        &config.source.api_url,
        &config.source.org,
        source_token.clone(),
    )?;
    let forge = Forge::new(config.target.clone(), target_token.clone())?;
    let target_git_user = if config.target.kind == crate::config::TargetKind::Gitlab {
        "oauth2".to_owned()
    } else {
        forge.doctor().context("authenticate with target forge")?
    };
    let cache_key = format!("github:{}:repositories", config.source.org);
    let etag = state.cache_get(&(cache_key.clone() + ":etag"))?;
    let repos = match github.repositories(etag.as_deref())? {
        RepoResponse::Fresh { repos, etag } => {
            state.cache_set(&cache_key, &serde_json::to_string(&repos)?)?;
            if let Some(tag) = etag {
                state.cache_set(&(cache_key.clone() + ":etag"), &tag)?;
            }
            repos
        }
        RepoResponse::NotModified => cached_repositories(
            &state
                .cache_get(&cache_key)?
                .context("GitHub returned not modified but no repository cache exists")?,
        )?,
    };
    let repos: Vec<_> = repos
        .into_iter()
        .filter(|r| config.includes_repo(&r.name, r.archived))
        .collect();
    report.discovered = repos.len();
    if config.sync.experimental_comment_relay {
        state.audit(
            "warning",
            None,
            Some("relay"),
            None,
            "experimental comment relay requested; sending target comments remains disabled in v1",
        )?;
    }
    let archive = if config.sync.dry_run {
        Archive::open_read_only(&config.sync.archive_dir, config.sync.git_archive)
    } else {
        Archive::new(&config.sync.archive_dir, config.sync.git_archive)?
    };
    let resources = SyncResources {
        config,
        github: &github,
        forge: &forge,
        state: &state,
        archive: &archive,
        source_token: &source_token,
        target_token: &target_token,
        target_git_user: &target_git_user,
    };
    for repo in repos {
        match sync_repository(&resources, &repo, &mut report) {
            Ok(()) => report.synchronized += 1,
            Err(error) => {
                report.failed += 1;
                let safe = redact(&format!("{error:#}"), &[&source_token, &target_token]);
                state.repository_error(&repo.name, repo.id, &safe)?;
                state.audit(
                    "failed",
                    Some(&repo.name),
                    Some("repository"),
                    Some(repo.id),
                    &safe,
                )?;
                report.errors.push(RepoError {
                    repository: repo.name,
                    error: safe,
                });
            }
        }
    }
    archive.commit()?;
    report.duration_ms = started.elapsed().as_millis();
    Ok(report)
}

fn sync_repository(
    resources: &SyncResources<'_>,
    repo: &crate::model::Repository,
    report: &mut RunReport,
) -> Result<()> {
    let SyncResources {
        config,
        github,
        forge,
        state,
        archive,
        source_token,
        target_token,
        target_git_user,
    } = resources;
    state.audit(
        "start",
        Some(&repo.name),
        Some("repository"),
        Some(repo.id),
        "synchronization started",
    )?;
    let target = forge
        .ensure_repo(repo, config.sync.private, config.sync.dry_run)
        .context("ensure target repository")?;
    if !config.sync.dry_run {
        gitmirror::mirror(
            repo,
            &target.http_url,
            target_git_user,
            source_token,
            target_token,
            &config.sync.state_dir,
        )
        .context("mirror Git refs")?;
    }
    let mut labels = github.labels(&repo.name).context("list labels")?;
    if !labels.iter().any(|l| l.name == "forge-sync:pull-request") {
        labels.push(Label {
            id: -repo.id,
            name: "forge-sync:pull-request".into(),
            color: "174d67".into(),
            description: Some("Mirrored GitHub pull request discussion".into()),
        });
    }
    let mut label_map = HashMap::new();
    for label in &labels {
        let fingerprint = serde_json::to_string(label)?;
        let existing = state.mapping("label", &repo.name, label.id)?;
        let target_id = if existing
            .as_ref()
            .is_some_and(|(_, seen)| seen == &fingerprint)
        {
            existing.expect("mapping checked").0
        } else {
            forge.ensure_label(
                &target,
                label,
                existing.map(|mapping| mapping.0),
                config.sync.dry_run,
            )?
        };
        label_map.insert(label.name.clone(), target_id);
        state.map("label", &repo.name, label.id, target_id, &fingerprint)?;
    }
    let milestones = github.milestones(&repo.name).context("list milestones")?;
    let mut milestone_map = HashMap::new();
    for milestone in &milestones {
        let fingerprint = serde_json::to_string(milestone)?;
        let existing = state.mapping("milestone", &repo.name, milestone.id)?;
        let id = if existing
            .as_ref()
            .is_some_and(|(_, seen)| seen == &fingerprint)
        {
            existing.expect("mapping checked").0
        } else {
            forge.ensure_milestone(
                &target,
                milestone,
                existing.map(|mapping| mapping.0),
                config.sync.dry_run,
            )?
        };
        state.map("milestone", &repo.name, milestone.id, id, &fingerprint)?;
        milestone_map.insert(milestone.id, id);
    }
    let issues = github
        .issues(&repo.name)
        .context("list issues and pull requests")?;
    let mut repository_comments = HashMap::<i64, Vec<_>>::new();
    for comment in github.repository_comments(&repo.name)? {
        if let Some(number) = comment
            .issue_url
            .as_deref()
            .and_then(|url| url.rsplit('/').next())
            .and_then(|value| value.parse().ok())
        {
            repository_comments.entry(number).or_default().push(comment);
        }
    }
    let mut snapshots = Vec::with_capacity(issues.len());
    for issue in issues {
        let mut effective = issue.clone();
        if issue.is_pull_request() {
            effective.labels.push(
                labels
                    .iter()
                    .find(|l| l.name == "forge-sync:pull-request")
                    .expect("synthetic label")
                    .clone(),
            );
        }
        let label_ids = effective
            .labels
            .iter()
            .filter_map(|l| label_map.get(&l.name).copied())
            .collect::<Vec<_>>();
        let milestone_id = issue
            .milestone
            .as_ref()
            .and_then(|m| milestone_map.get(&m.id).copied());
        let existing = state.mapping("item", &repo.name, issue.id)?;
        let unchanged = existing
            .as_ref()
            .is_some_and(|(_, seen)| seen == &issue.updated_at);
        let target_issue = if unchanged {
            existing.expect("mapping checked").0
        } else {
            forge
                .upsert_issue(
                    &target,
                    &effective,
                    existing.map(|mapping| mapping.0),
                    &label_ids,
                    milestone_id,
                    config.sync.dry_run,
                )
                .with_context(|| format!("sync item #{}", issue.number))?
        };
        state.map(
            "item",
            &repo.name,
            issue.id,
            target_issue,
            &issue.updated_at,
        )?;
        report.issues += 1;
        if unchanged && archive.has_item(&repo.name, issue.number) {
            continue;
        }
        let mut comments = repository_comments
            .remove(&issue.number)
            .unwrap_or_default();
        if issue.is_pull_request() {
            comments.extend(github.reviews(&repo.name, issue.number)?);
            comments.extend(github.pull_comments(&repo.name, issue.number)?);
            comments.sort_by(|a, b| a.created_at.cmp(&b.created_at));
        }
        for comment in &comments {
            let kind = if comment.kind.starts_with("review (") {
                "review"
            } else if comment.kind == "inline review comment" {
                "review-comment"
            } else {
                "comment"
            };
            let existing = state.mapping(kind, &repo.name, comment.id)?;
            let id = if existing
                .as_ref()
                .is_some_and(|(_, seen)| seen == &comment.updated_at)
            {
                existing.expect("mapping checked").0
            } else {
                forge.upsert_comment(
                    &target,
                    target_issue,
                    &comment.mirrored_body(),
                    existing.map(|mapping| mapping.0),
                    comment.id,
                    config.sync.dry_run,
                )?
            };
            state.map(kind, &repo.name, comment.id, id, &comment.updated_at)?;
            report.comments += 1;
        }
        snapshots.push(ItemSnapshot { issue, comments });
    }
    archive.write_repository(repo, &labels, &milestones, &snapshots)?;
    state.repository_ok(&repo.name, repo.id, &target.id)?;
    state.audit(
        "complete",
        Some(&repo.name),
        Some("repository"),
        Some(repo.id),
        &format!("target {}", target.web_url),
    )?;
    Ok(())
}

fn redact(input: &str, values: &[&str]) -> String {
    values
        .iter()
        .filter(|v| !v.is_empty())
        .fold(input.to_owned(), |s, v| s.replace(v, "[REDACTED]"))
}

pub fn partial_failure(report: &RunReport) -> Result<()> {
    if report.failed > 0 {
        Err(anyhow!("{} repositories failed", report.failed))
    } else {
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn redacts_secrets() {
        assert_eq!(redact("token abc", &["abc"]), "token [REDACTED]");
    }
}
