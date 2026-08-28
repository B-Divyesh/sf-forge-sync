use anyhow::{bail, Context, Result};
use serde::{Deserialize, Serialize};
use std::{
    env, fs,
    path::{Path, PathBuf},
};

pub const EXAMPLE: &str = r#"# forge-sync configuration
[source]
org = "acme"
token_env = "GITHUB_TOKEN"
api_url = "https://api.github.com"

[target]
kind = "forgejo" # forgejo, codeberg, or gitlab
base_url = "https://codeberg.org"
owner = "acme-mirror"
token_env = "FORGE_TOKEN"

[sync]
interval_seconds = 300
include_archived = true
private = true
state_dir = ".forge-sync"
archive_dir = "forge-archive"
git_archive = true
repos = []
exclude = []
dry_run = false
experimental_comment_relay = false
"#;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub source: SourceConfig,
    pub target: TargetConfig,
    #[serde(default)]
    pub sync: SyncConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SourceConfig {
    pub org: String,
    #[serde(default = "github_token_env")]
    pub token_env: String,
    #[serde(default = "github_api")]
    pub api_url: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TargetConfig {
    pub kind: TargetKind,
    pub base_url: String,
    pub owner: String,
    #[serde(default = "forge_token_env")]
    pub token_env: String,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum TargetKind {
    Forgejo,
    Codeberg,
    Gitlab,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct SyncConfig {
    pub interval_seconds: u64,
    pub include_archived: bool,
    pub private: bool,
    pub state_dir: PathBuf,
    pub archive_dir: PathBuf,
    pub git_archive: bool,
    pub repos: Vec<String>,
    pub exclude: Vec<String>,
    pub dry_run: bool,
    pub experimental_comment_relay: bool,
}

impl Default for SyncConfig {
    fn default() -> Self {
        Self {
            interval_seconds: 300,
            include_archived: true,
            private: true,
            state_dir: PathBuf::from(".forge-sync"),
            archive_dir: PathBuf::from("forge-archive"),
            git_archive: true,
            repos: vec![],
            exclude: vec![],
            dry_run: false,
            experimental_comment_relay: false,
        }
    }
}

impl Config {
    pub fn from_path(path: &Path) -> Result<Self> {
        let raw = fs::read_to_string(path)
            .with_context(|| format!("configuration error: read {}", path.display()))?;
        let config: Self = toml::from_str(&raw).context("configuration error: parse TOML")?;
        config.validate().context("configuration error")?;
        Ok(config)
    }

    pub fn validate(&self) -> Result<()> {
        if self.source.org.trim().is_empty() {
            bail!("source.org may not be empty");
        }
        if self.target.owner.trim().is_empty() {
            bail!("target.owner may not be empty");
        }
        if !self.source.api_url.starts_with("http://")
            && !self.source.api_url.starts_with("https://")
        {
            bail!("source.api_url must be an http(s) URL");
        }
        if !self.target.base_url.starts_with("http://")
            && !self.target.base_url.starts_with("https://")
        {
            bail!("target.base_url must be an http(s) URL");
        }
        if self.sync.interval_seconds == 0 {
            bail!("sync.interval_seconds must be at least 1");
        }
        Ok(())
    }

    pub fn source_token(&self) -> Result<String> {
        required_env(&self.source.token_env).context("configuration error")
    }
    pub fn target_token(&self) -> Result<String> {
        required_env(&self.target.token_env).context("configuration error")
    }

    pub fn includes_repo(&self, name: &str, archived: bool) -> bool {
        if archived && !self.sync.include_archived {
            return false;
        }
        if !self.sync.repos.is_empty() && !self.sync.repos.iter().any(|r| r == name) {
            return false;
        }
        !self
            .sync
            .exclude
            .iter()
            .any(|pattern| wildcard(pattern, name))
    }
}

fn required_env(name: &str) -> Result<String> {
    env::var(name).with_context(|| format!("required environment variable {name} is not set"))
}
fn github_api() -> String {
    "https://api.github.com".into()
}
fn github_token_env() -> String {
    "GITHUB_TOKEN".into()
}
fn forge_token_env() -> String {
    "FORGE_TOKEN".into()
}

fn wildcard(pattern: &str, value: &str) -> bool {
    let (mut p, mut v, mut star, mut retry) = (0, 0, None, 0);
    let pb = pattern.as_bytes();
    let vb = value.as_bytes();
    while v < vb.len() {
        if p < pb.len() && (pb[p] == b'?' || pb[p] == vb[v]) {
            p += 1;
            v += 1;
        } else if p < pb.len() && pb[p] == b'*' {
            star = Some(p);
            p += 1;
            retry = v;
        } else if let Some(s) = star {
            p = s + 1;
            retry += 1;
            v = retry;
        } else {
            return false;
        }
    }
    while p < pb.len() && pb[p] == b'*' {
        p += 1;
    }
    p == pb.len()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn wildcard_patterns_work() {
        assert!(wildcard("scratch-*", "scratch-one"));
        assert!(wildcard("a?c", "abc"));
        assert!(!wildcard("docs-*", "api"));
    }
    #[test]
    fn example_parses() {
        let c: Config = toml::from_str(EXAMPLE).unwrap();
        c.validate().unwrap();
        assert_eq!(c.target.kind, TargetKind::Forgejo);
    }
}
