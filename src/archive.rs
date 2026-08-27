use crate::model::{ItemSnapshot, Label, Milestone, Repository};
use anyhow::{bail, Context, Result};
use chrono::Utc;
use serde::Serialize;
use std::{
    fs,
    path::{Path, PathBuf},
    process::Command,
};

pub struct Archive {
    root: PathBuf,
    git: bool,
}

impl Archive {
    pub fn new(root: &Path, git: bool) -> Result<Self> {
        fs::create_dir_all(root)?;
        Ok(Self {
            root: root.into(),
            git,
        })
    }
    pub fn write_repository(
        &self,
        repo: &Repository,
        labels: &[Label],
        milestones: &[Milestone],
        items: &[ItemSnapshot],
    ) -> Result<()> {
        let dir = self.root.join("repositories").join(&repo.name);
        let item_dir = dir.join("items");
        fs::create_dir_all(&item_dir)?;
        write_json(&dir.join("repository.json"), repo)?;
        write_json(&dir.join("labels.json"), labels)?;
        write_json(&dir.join("milestones.json"), milestones)?;
        for item in items {
            write_json(&item_dir.join(format!("{}.json", item.issue.number)), item)?;
        }
        write_json(
            &self.root.join("manifest.json"),
            &Manifest {
                schema: 1,
                generated_at: Utc::now().to_rfc3339(),
                format: "forge-sync portable archive",
            },
        )?;
        Ok(())
    }
    pub fn has_item(&self, repo: &str, number: i64) -> bool {
        self.root
            .join("repositories")
            .join(repo)
            .join("items")
            .join(format!("{number}.json"))
            .is_file()
    }
    pub fn commit(&self) -> Result<()> {
        if !self.git {
            return Ok(());
        }
        if !self.root.join(".git").exists() {
            git(&self.root, &["init", "--quiet"])?;
        }
        git(&self.root, &["add", "--all"])?;
        let changed = Command::new("git")
            .args(["diff", "--cached", "--quiet"])
            .current_dir(&self.root)
            .status()?;
        if !changed.success() {
            git(
                &self.root,
                &[
                    "-c",
                    "user.name=forge-sync",
                    "-c",
                    "user.email=archive@forge-sync.local",
                    "commit",
                    "--quiet",
                    "-m",
                    "archive: synchronize GitHub metadata",
                ],
            )?;
        }
        Ok(())
    }
}

#[derive(Serialize)]
struct Manifest<'a> {
    schema: u8,
    generated_at: String,
    format: &'a str,
}
fn write_json<T: Serialize + ?Sized>(path: &Path, value: &T) -> Result<()> {
    let temp = path.with_extension("json.tmp");
    let data = serde_json::to_vec_pretty(value)?;
    fs::write(&temp, data)?;
    fs::rename(temp, path)?;
    Ok(())
}
fn git(cwd: &Path, args: &[&str]) -> Result<()> {
    let out = Command::new("git")
        .args(args)
        .current_dir(cwd)
        .output()
        .context("execute archive git")?;
    if !out.status.success() {
        bail!(
            "archive git failed: {}",
            String::from_utf8_lossy(&out.stderr).trim()
        );
    }
    Ok(())
}
