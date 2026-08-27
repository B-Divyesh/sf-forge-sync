use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Repository {
    pub id: i64,
    pub name: String,
    pub full_name: String,
    #[serde(default)]
    pub description: Option<String>,
    pub clone_url: String,
    pub html_url: String,
    #[serde(default)]
    pub archived: bool,
    #[serde(default)]
    pub private: bool,
    #[serde(default)]
    pub default_branch: String,
    #[serde(default)]
    pub updated_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub login: String,
    #[serde(default)]
    pub html_url: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Label {
    pub id: i64,
    pub name: String,
    #[serde(default)]
    pub color: String,
    #[serde(default)]
    pub description: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Milestone {
    pub id: i64,
    pub number: i64,
    pub title: String,
    #[serde(default)]
    pub description: Option<String>,
    pub state: String,
    #[serde(default)]
    pub due_on: Option<String>,
    #[serde(default)]
    pub html_url: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PullRef {
    pub url: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Issue {
    pub id: i64,
    pub number: i64,
    pub title: String,
    #[serde(default)]
    pub body: Option<String>,
    pub state: String,
    pub user: User,
    #[serde(default)]
    pub labels: Vec<Label>,
    #[serde(default)]
    pub milestone: Option<Milestone>,
    #[serde(default)]
    pub pull_request: Option<PullRef>,
    pub html_url: String,
    #[serde(default)]
    pub created_at: String,
    #[serde(default)]
    pub updated_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Comment {
    pub id: i64,
    #[serde(default)]
    pub body: String,
    pub user: User,
    pub html_url: String,
    #[serde(default)]
    pub created_at: String,
    #[serde(default)]
    pub updated_at: String,
    #[serde(default)]
    pub kind: String,
    #[serde(default)]
    pub path: Option<String>,
    #[serde(default)]
    pub line: Option<i64>,
    #[serde(default)]
    pub issue_url: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Review {
    pub id: i64,
    #[serde(default)]
    pub body: Option<String>,
    pub user: User,
    pub html_url: String,
    #[serde(default)]
    pub submitted_at: Option<String>,
    #[serde(default)]
    pub state: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ItemSnapshot {
    pub issue: Issue,
    pub comments: Vec<Comment>,
}

impl Issue {
    pub fn is_pull_request(&self) -> bool {
        self.pull_request.is_some()
    }
    pub fn mirrored_title(&self) -> String {
        if self.is_pull_request() {
            format!("[GitHub PR #{}] {}", self.number, self.title)
        } else {
            self.title.clone()
        }
    }
    pub fn mirrored_body(&self) -> String {
        let kind = if self.is_pull_request() {
            "pull request"
        } else {
            "issue"
        };
        format!("{}\n\n---\n_Mirrored from GitHub {kind} [#{}]({}) by [@{}]({}). Originally created {}._\n\n<!-- forge-sync:github:item:{} -->",
            self.body.as_deref().unwrap_or(""), self.number, self.html_url, self.user.login, self.user.html_url,
            self.created_at, self.id)
    }
}

impl Comment {
    pub fn mirrored_body(&self) -> String {
        let anchor = match (&self.path, self.line) {
            (Some(p), Some(l)) => format!(" on `{p}:{l}`"),
            (Some(p), None) => format!(" on `{p}`"),
            _ => String::new(),
        };
        format!("{}\n\n---\n_Mirrored GitHub {}{} by [@{}]({}) on {}. [Open original]({})._\n\n<!-- forge-sync:github:comment:{} -->",
            self.body, if self.kind.is_empty() { "comment" } else { &self.kind }, anchor, self.user.login, self.user.html_url,
            self.created_at, self.html_url, self.id)
    }
}
