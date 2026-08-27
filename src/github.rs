use crate::model::{Comment, Issue, Label, Milestone, Repository, Review, User};
use anyhow::{anyhow, bail, Context, Result};
use reqwest::blocking::{Client, Response};
use reqwest::header::{ACCEPT, AUTHORIZATION, ETAG, IF_NONE_MATCH, USER_AGENT};
use serde::de::DeserializeOwned;
use serde_json::Value;
use std::time::Duration;

pub struct GitHub {
    client: Client,
    base: String,
    token: String,
    org: String,
}

pub enum RepoResponse {
    NotModified,
    Fresh {
        repos: Vec<Repository>,
        etag: Option<String>,
    },
}

impl GitHub {
    pub fn new(base: &str, org: &str, token: String) -> Result<Self> {
        let client = Client::builder().timeout(Duration::from_secs(60)).build()?;
        Ok(Self {
            client,
            base: base.trim_end_matches('/').into(),
            token,
            org: org.into(),
        })
    }
    fn get(&self, path: &str) -> reqwest::blocking::RequestBuilder {
        self.client
            .get(format!("{}{}", self.base, path))
            .header(
                USER_AGENT,
                concat!("forge-sync/", env!("CARGO_PKG_VERSION")),
            )
            .header(ACCEPT, "application/vnd.github+json")
            .header(AUTHORIZATION, format!("Bearer {}", self.token))
            .header("X-GitHub-Api-Version", "2022-11-28")
    }
    fn checked(response: Response) -> Result<Response> {
        let status = response.status();
        if status.is_success() {
            return Ok(response);
        }
        let remaining = response
            .headers()
            .get("x-ratelimit-remaining")
            .and_then(|v| v.to_str().ok())
            .unwrap_or("unknown")
            .to_owned();
        let body = response.text().unwrap_or_default();
        bail!(
            "GitHub API returned {status} (rate remaining: {remaining}): {}",
            truncate(&body)
        )
    }
    pub fn doctor(&self) -> Result<User> {
        Self::checked(self.get("/user").send()?)?
            .json()
            .context("decode GitHub identity")
    }

    pub fn repositories(&self, etag: Option<&str>) -> Result<RepoResponse> {
        let mut request = self.get(&format!(
            "/orgs/{}/repos?type=all&sort=full_name&per_page=100&page=1",
            self.org
        ));
        if let Some(tag) = etag {
            request = request.header(IF_NONE_MATCH, tag);
        }
        let response = request
            .send()
            .context("list GitHub organization repositories")?;
        if response.status().as_u16() == 304 {
            return Ok(RepoResponse::NotModified);
        }
        let response = Self::checked(response)?;
        let tag = response
            .headers()
            .get(ETAG)
            .and_then(|v| v.to_str().ok())
            .map(str::to_owned);
        let mut repos: Vec<Repository> = response.json().context("decode GitHub repositories")?;
        let mut page = 2;
        while repos.len() == (page - 1) * 100 {
            let batch: Vec<Repository> = self.page(&format!(
                "/orgs/{}/repos?type=all&sort=full_name&per_page=100&page={page}",
                self.org
            ))?;
            let count = batch.len();
            repos.extend(batch);
            if count < 100 {
                break;
            }
            page += 1;
        }
        Ok(RepoResponse::Fresh { repos, etag: tag })
    }
    pub fn labels(&self, repo: &str) -> Result<Vec<Label>> {
        self.pages(&format!("/repos/{}/{repo}/labels", self.org))
    }
    pub fn milestones(&self, repo: &str) -> Result<Vec<Milestone>> {
        self.pages(&format!("/repos/{}/{repo}/milestones?state=all", self.org))
    }
    pub fn issues(&self, repo: &str) -> Result<Vec<Issue>> {
        self.pages(&format!(
            "/repos/{}/{repo}/issues?state=all&sort=updated&direction=asc",
            self.org
        ))
    }
    pub fn repository_comments(&self, repo: &str) -> Result<Vec<Comment>> {
        let mut comments: Vec<Comment> = self.pages(&format!(
            "/repos/{}/{repo}/issues/comments?sort=updated&direction=asc",
            self.org
        ))?;
        for item in &mut comments {
            item.kind = "comment".into();
        }
        Ok(comments)
    }
    pub fn pull_comments(&self, repo: &str, number: i64) -> Result<Vec<Comment>> {
        let mut comments: Vec<Comment> = self.pages(&format!(
            "/repos/{}/{repo}/pulls/{number}/comments",
            self.org
        ))?;
        for item in &mut comments {
            item.kind = "inline review comment".into();
        }
        Ok(comments)
    }
    pub fn reviews(&self, repo: &str, number: i64) -> Result<Vec<Comment>> {
        let reviews: Vec<Review> = self.pages(&format!(
            "/repos/{}/{repo}/pulls/{number}/reviews",
            self.org
        ))?;
        Ok(reviews
            .into_iter()
            .filter(|r| {
                r.body
                    .as_deref()
                    .is_some_and(|body| !body.trim().is_empty())
            })
            .map(|r| Comment {
                id: r.id,
                body: r.body.unwrap_or_default(),
                user: r.user,
                html_url: r.html_url,
                created_at: r.submitted_at.clone().unwrap_or_default(),
                updated_at: r.submitted_at.unwrap_or_default(),
                kind: format!("review ({})", r.state.to_lowercase()),
                path: None,
                line: None,
                issue_url: None,
            })
            .collect())
    }
    fn page<T: DeserializeOwned>(&self, path: &str) -> Result<Vec<T>> {
        Self::checked(self.get(path).send()?)?
            .json()
            .with_context(|| format!("decode GitHub response for {path}"))
    }
    fn pages<T: DeserializeOwned>(&self, path: &str) -> Result<Vec<T>> {
        let separator = if path.contains('?') { '&' } else { '?' };
        let mut all = vec![];
        let mut page = 1;
        loop {
            let batch: Vec<T> = self.page(&format!("{path}{separator}per_page=100&page={page}"))?;
            let count = batch.len();
            all.extend(batch);
            if count < 100 {
                break;
            }
            page += 1;
        }
        Ok(all)
    }
}

fn truncate(body: &str) -> String {
    let parsed = serde_json::from_str::<Value>(body)
        .ok()
        .and_then(|v| v.get("message").and_then(Value::as_str).map(str::to_owned));
    parsed.unwrap_or_else(|| body.chars().take(300).collect())
}

pub fn cached_repositories(raw: &str) -> Result<Vec<Repository>> {
    serde_json::from_str(raw).map_err(|e| anyhow!(e))
}
