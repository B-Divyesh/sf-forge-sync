use crate::{
    config::{TargetConfig, TargetKind},
    model::{Issue, Label, Milestone, Repository},
};
use anyhow::{bail, Context, Result};
use percent_encoding::{utf8_percent_encode, NON_ALPHANUMERIC};
use reqwest::{
    blocking::{Client, Response},
    Method, StatusCode,
};
use serde::{de::DeserializeOwned, Deserialize};
use serde_json::{json, Value};
use std::time::Duration;

#[derive(Debug, Clone)]
pub struct TargetRepo {
    pub id: String,
    pub name: String,
    pub http_url: String,
    pub web_url: String,
}

pub struct Forge {
    client: Client,
    config: TargetConfig,
    token: String,
}

#[derive(Deserialize)]
struct ForgeRepo {
    id: i64,
    name: String,
    clone_url: String,
    html_url: String,
}
#[derive(Deserialize)]
struct GitLabRepo {
    id: i64,
    name: String,
    http_url_to_repo: String,
    web_url: String,
}
#[derive(Deserialize)]
struct ForgeLabel {
    id: i64,
}
#[derive(Deserialize)]
struct GitLabLabel {
    id: i64,
}
#[derive(Deserialize)]
struct ForgeMilestone {
    id: i64,
}
#[derive(Deserialize)]
struct GitLabMilestone {
    id: i64,
}
#[derive(Deserialize)]
struct ForgeIssue {
    number: i64,
}
#[derive(Deserialize)]
struct GitLabIssue {
    iid: i64,
}
#[derive(Deserialize)]
struct ForgeComment {
    id: i64,
}
#[derive(Deserialize)]
struct GitLabComment {
    id: i64,
}

impl Forge {
    pub fn new(config: TargetConfig, token: String) -> Result<Self> {
        Ok(Self {
            client: Client::builder().timeout(Duration::from_secs(60)).build()?,
            config,
            token,
        })
    }
    pub fn kind(&self) -> TargetKind {
        self.config.kind
    }
    fn api(&self) -> String {
        let base = self.config.base_url.trim_end_matches('/');
        match self.config.kind {
            TargetKind::Forgejo | TargetKind::Codeberg => format!("{base}/api/v1"),
            TargetKind::Gitlab => format!("{base}/api/v4"),
        }
    }
    fn request(&self, method: Method, path: &str) -> reqwest::blocking::RequestBuilder {
        let r = self
            .client
            .request(method, format!("{}{}", self.api(), path))
            .header(
                "User-Agent",
                concat!("forge-sync/", env!("CARGO_PKG_VERSION")),
            );
        match self.config.kind {
            TargetKind::Forgejo | TargetKind::Codeberg => {
                r.header("Authorization", format!("token {}", self.token))
            }
            TargetKind::Gitlab => r.header("PRIVATE-TOKEN", &self.token),
        }
    }
    fn checked(response: Response) -> Result<Response> {
        if response.status().is_success() {
            return Ok(response);
        }
        let status = response.status();
        let body = response.text().unwrap_or_default();
        bail!(
            "target API returned {status}: {}",
            body.chars().take(300).collect::<String>()
        )
    }
    fn json<T: DeserializeOwned>(
        &self,
        method: Method,
        path: &str,
        body: Option<Value>,
    ) -> Result<T> {
        let mut req = self.request(method, path);
        if let Some(v) = body {
            req = req.json(&v);
        }
        Self::checked(req.send()?)?
            .json()
            .with_context(|| format!("decode target response for {path}"))
    }
    pub fn doctor(&self) -> Result<String> {
        let v: Value = self.json(Method::GET, "/user", None)?;
        Ok(v.get("login")
            .or_else(|| v.get("username"))
            .and_then(Value::as_str)
            .unwrap_or("authenticated user")
            .into())
    }
    pub fn ensure_repo(&self, source: &Repository, private: bool, dry: bool) -> Result<TargetRepo> {
        match self.config.kind {
            TargetKind::Forgejo | TargetKind::Codeberg => {
                let path = format!("/repos/{}/{}", self.config.owner, source.name);
                let found = self.request(Method::GET, &path).send()?;
                let repo: ForgeRepo = if found.status() == StatusCode::NOT_FOUND {
                    if dry {
                        return Ok(TargetRepo {
                            id: format!("dry:{}", source.id),
                            name: source.name.clone(),
                            http_url: target_git_url(&self.config, &source.name),
                            web_url: format!(
                                "{}/{}/{}",
                                self.config.base_url.trim_end_matches('/'),
                                self.config.owner,
                                source.name
                            ),
                        });
                    }
                    self.json(Method::POST, &format!("/orgs/{}/repos", self.config.owner), Some(json!({"name":source.name,"description":source.description,"private":private,"auto_init":false})))?
                } else {
                    Self::checked(found)?.json()?
                };
                Ok(TargetRepo {
                    id: repo.id.to_string(),
                    name: repo.name,
                    http_url: repo.clone_url,
                    web_url: repo.html_url,
                })
            }
            TargetKind::Gitlab => {
                let full = format!("{}/{}", self.config.owner, source.name);
                let encoded = utf8_percent_encode(&full, NON_ALPHANUMERIC);
                let found = self
                    .request(Method::GET, &format!("/projects/{encoded}"))
                    .send()?;
                let repo: GitLabRepo = if found.status() == StatusCode::NOT_FOUND {
                    if dry {
                        return Ok(TargetRepo {
                            id: format!("dry:{}", source.id),
                            name: source.name.clone(),
                            http_url: target_git_url(&self.config, &source.name),
                            web_url: format!(
                                "{}/{}/{}",
                                self.config.base_url.trim_end_matches('/'),
                                self.config.owner,
                                source.name
                            ),
                        });
                    }
                    let ns: Value = self.json(
                        Method::GET,
                        &format!(
                            "/namespaces/{}",
                            utf8_percent_encode(&self.config.owner, NON_ALPHANUMERIC)
                        ),
                        None,
                    )?;
                    let ns_id = ns
                        .get("id")
                        .and_then(Value::as_i64)
                        .context("target GitLab namespace has no id")?;
                    self.json(Method::POST, "/projects", Some(json!({"name":source.name,"namespace_id":ns_id,"visibility":if private{"private"}else{"public"},"description":source.description,"initialize_with_readme":false})))?
                } else {
                    Self::checked(found)?.json()?
                };
                Ok(TargetRepo {
                    id: repo.id.to_string(),
                    name: repo.name,
                    http_url: repo.http_url_to_repo,
                    web_url: repo.web_url,
                })
            }
        }
    }
    pub fn ensure_label(
        &self,
        repo: &TargetRepo,
        label: &Label,
        mapped: Option<i64>,
        dry: bool,
    ) -> Result<i64> {
        if dry {
            return Ok(mapped.unwrap_or(label.id));
        }
        match self.config.kind {
            TargetKind::Forgejo | TargetKind::Codeberg => {
                let color = label.color.trim_start_matches('#');
                if let Some(id) = mapped {
                    let _: Value = self.json(Method::PATCH, &format!("/repos/{}/{}/labels/{id}", self.config.owner, repo.name), Some(json!({"name":label.name,"color":color,"description":label.description})))?;
                    return Ok(id);
                }
                let labels: Vec<Value> = self.json(
                    Method::GET,
                    &format!(
                        "/repos/{}/{}/labels?limit=100",
                        self.config.owner, repo.name
                    ),
                    None,
                )?;
                if let Some(id) = find_named_id(&labels, &label.name) {
                    return Ok(id);
                }
                let out: ForgeLabel = self.json(
                    Method::POST,
                    &format!("/repos/{}/{}/labels", self.config.owner, repo.name),
                    Some(json!({"name":label.name,"color":color,"description":label.description})),
                )?;
                Ok(out.id)
            }
            TargetKind::Gitlab => {
                if let Some(id) = mapped {
                    let _: Value = self.json(Method::PUT, &format!("/projects/{}/labels/{id}", repo.id), Some(json!({"new_name":label.name,"color":format!("#{}", label.color.trim_start_matches('#')),"description":label.description})))?;
                    return Ok(id);
                }
                let labels: Vec<Value> = self.json(
                    Method::GET,
                    &format!("/projects/{}/labels?per_page=100", repo.id),
                    None,
                )?;
                if let Some(id) = find_named_id(&labels, &label.name) {
                    return Ok(id);
                }
                let out: GitLabLabel = self.json(Method::POST, &format!("/projects/{}/labels", repo.id), Some(json!({"name":label.name,"color":format!("#{}", label.color.trim_start_matches('#')),"description":label.description})))?;
                Ok(out.id)
            }
        }
    }
    pub fn ensure_milestone(
        &self,
        repo: &TargetRepo,
        milestone: &Milestone,
        mapped: Option<i64>,
        dry: bool,
    ) -> Result<i64> {
        if dry {
            return Ok(mapped.unwrap_or(milestone.id));
        }
        let body = json!({"title":milestone.title,"description":milestone.description,"state":milestone.state,"due_on":milestone.due_on.as_deref().map(|s| s.get(..10).unwrap_or(s))});
        match (self.config.kind, mapped) {
            (TargetKind::Forgejo | TargetKind::Codeberg, Some(id)) => {
                let _: Value = self.json(
                    Method::PATCH,
                    &format!("/repos/{}/{}/milestones/{id}", self.config.owner, repo.name),
                    Some(body),
                )?;
                Ok(id)
            }
            (TargetKind::Forgejo | TargetKind::Codeberg, None) => {
                let v: ForgeMilestone = self.json(
                    Method::POST,
                    &format!("/repos/{}/{}/milestones", self.config.owner, repo.name),
                    Some(body),
                )?;
                Ok(v.id)
            }
            (TargetKind::Gitlab, Some(id)) => {
                let _: Value = self.json(
                    Method::PUT,
                    &format!("/projects/{}/milestones/{id}", repo.id),
                    Some(body),
                )?;
                Ok(id)
            }
            (TargetKind::Gitlab, None) => {
                let v: GitLabMilestone = self.json(
                    Method::POST,
                    &format!("/projects/{}/milestones", repo.id),
                    Some(body),
                )?;
                Ok(v.id)
            }
        }
    }
    pub fn upsert_issue(
        &self,
        repo: &TargetRepo,
        issue: &Issue,
        mapped: Option<i64>,
        label_ids: &[i64],
        milestone: Option<i64>,
        dry: bool,
    ) -> Result<i64> {
        if dry {
            return Ok(mapped.unwrap_or(issue.id));
        }
        match self.config.kind {
            TargetKind::Forgejo | TargetKind::Codeberg => {
                let update = json!({"title":issue.mirrored_title(),"body":issue.mirrored_body(),"state":issue.state,"milestone":milestone});
                let id = if let Some(id) = mapped {
                    let _: Value = self.json(
                        Method::PATCH,
                        &format!("/repos/{}/{}/issues/{id}", self.config.owner, repo.name),
                        Some(update),
                    )?;
                    id
                } else {
                    let out: ForgeIssue = self.json(
                        Method::POST,
                        &format!("/repos/{}/{}/issues", self.config.owner, repo.name),
                        Some(json!({"title":issue.mirrored_title(),"body":issue.mirrored_body(),"closed":issue.state=="closed","labels":label_ids,"milestone":milestone})),
                    )?;
                    out.number
                };
                Self::checked(
                    self.request(
                        Method::PUT,
                        &format!(
                            "/repos/{}/{}/issues/{id}/labels",
                            self.config.owner, repo.name
                        ),
                    )
                    .json(&json!({"labels":label_ids}))
                    .send()?,
                )?;
                Ok(id)
            }
            TargetKind::Gitlab => {
                let names = issue
                    .labels
                    .iter()
                    .map(|l| l.name.as_str())
                    .collect::<Vec<_>>()
                    .join(",");
                let body = json!({"title":issue.mirrored_title(),"description":issue.mirrored_body(),"labels":names,"milestone_id":milestone,"state_event":if issue.state=="closed"{"close"}else{"reopen"}});
                if let Some(id) = mapped {
                    let _: Value = self.json(
                        Method::PUT,
                        &format!("/projects/{}/issues/{id}", repo.id),
                        Some(body),
                    )?;
                    Ok(id)
                } else {
                    let out: GitLabIssue = self.json(
                        Method::POST,
                        &format!("/projects/{}/issues", repo.id),
                        Some(json!({"title":issue.mirrored_title(),"description":issue.mirrored_body(),"labels":names,"milestone_id":milestone})),
                    )?;
                    if issue.state == "closed" {
                        let _: Value = self.json(
                            Method::PUT,
                            &format!("/projects/{}/issues/{}", repo.id, out.iid),
                            Some(json!({"state_event":"close"})),
                        )?;
                    }
                    Ok(out.iid)
                }
            }
        }
    }
    pub fn upsert_comment(
        &self,
        repo: &TargetRepo,
        target_issue: i64,
        body: &str,
        mapped: Option<i64>,
        source_id: i64,
        dry: bool,
    ) -> Result<i64> {
        if dry {
            return Ok(mapped.unwrap_or(source_id));
        }
        match (self.config.kind, mapped) {
            (TargetKind::Forgejo | TargetKind::Codeberg, Some(id)) => {
                let _: Value = self.json(
                    Method::PATCH,
                    &format!(
                        "/repos/{}/{}/issues/comments/{id}",
                        self.config.owner, repo.name
                    ),
                    Some(json!({"body":body})),
                )?;
                Ok(id)
            }
            (TargetKind::Forgejo | TargetKind::Codeberg, None) => {
                let v: ForgeComment = self.json(
                    Method::POST,
                    &format!(
                        "/repos/{}/{}/issues/{target_issue}/comments",
                        self.config.owner, repo.name
                    ),
                    Some(json!({"body":body})),
                )?;
                Ok(v.id)
            }
            (TargetKind::Gitlab, Some(id)) => {
                let _: Value = self.json(
                    Method::PUT,
                    &format!("/projects/{}/issues/{target_issue}/notes/{id}", repo.id),
                    Some(json!({"body":body})),
                )?;
                Ok(id)
            }
            (TargetKind::Gitlab, None) => {
                let v: GitLabComment = self.json(
                    Method::POST,
                    &format!("/projects/{}/issues/{target_issue}/notes", repo.id),
                    Some(json!({"body":body})),
                )?;
                Ok(v.id)
            }
        }
    }
}

fn find_named_id(values: &[Value], name: &str) -> Option<i64> {
    values
        .iter()
        .find(|v| v.get("name").and_then(Value::as_str) == Some(name))
        .and_then(|v| v.get("id"))
        .and_then(Value::as_i64)
}
fn target_git_url(config: &TargetConfig, name: &str) -> String {
    format!(
        "{}/{}/{}.git",
        config.base_url.trim_end_matches('/'),
        config.owner,
        name
    )
}
