use crate::model::Repository;
use anyhow::{bail, Context, Result};
use std::{
    fs,
    path::{Path, PathBuf},
    process::{Command, Output},
};

pub fn mirror(
    repo: &Repository,
    target_url: &str,
    target_user: &str,
    source_token: &str,
    target_token: &str,
    state_dir: &Path,
) -> Result<()> {
    let root = state_dir.join("mirrors");
    fs::create_dir_all(&root)?;
    let local = root.join(format!("{}.git", safe_name(&repo.name)));
    let askpass = ensure_askpass(state_dir)?;
    if !local.exists() {
        run_git(
            None,
            &["clone", "--mirror", &repo.clone_url, path_str(&local)?],
            &askpass,
            "x-access-token",
            source_token,
        )
        .with_context(|| format!("clone source repository {}", repo.name))?;
    } else {
        run_git(
            Some(&local),
            &["remote", "set-url", "origin", &repo.clone_url],
            &askpass,
            "x-access-token",
            source_token,
        )?;
        run_git(
            Some(&local),
            &["fetch", "--prune", "origin", "+refs/*:refs/*"],
            &askpass,
            "x-access-token",
            source_token,
        )
        .with_context(|| format!("fetch source repository {}", repo.name))?;
    }
    let remotes = run_git_capture(Some(&local), &["remote"], &askpass, "token", target_token)?;
    if String::from_utf8_lossy(&remotes.stdout)
        .lines()
        .any(|r| r == "mirror-target")
    {
        run_git(
            Some(&local),
            &["remote", "set-url", "mirror-target", target_url],
            &askpass,
            "token",
            target_token,
        )?;
    } else {
        run_git(
            Some(&local),
            &["remote", "add", "mirror-target", target_url],
            &askpass,
            "token",
            target_token,
        )?;
    }
    run_git(
        Some(&local),
        &["push", "--mirror", "mirror-target"],
        &askpass,
        target_user,
        target_token,
    )
    .with_context(|| format!("push mirror for {}", repo.name))
}

fn run_git(
    cwd: Option<&Path>,
    args: &[&str],
    askpass: &Path,
    user: &str,
    password: &str,
) -> Result<()> {
    let out = run_git_capture(cwd, args, askpass, user, password)?;
    if !out.status.success() {
        bail!(
            "git exited {}: {}",
            out.status,
            String::from_utf8_lossy(&out.stderr).trim()
        );
    }
    Ok(())
}
fn run_git_capture(
    cwd: Option<&Path>,
    args: &[&str],
    askpass: &Path,
    user: &str,
    password: &str,
) -> Result<Output> {
    let mut command = Command::new("git");
    command
        .args(args)
        .env("GIT_TERMINAL_PROMPT", "0")
        .env("GIT_ASKPASS", askpass)
        .env("FORGE_SYNC_GIT_USER", user)
        .env("FORGE_SYNC_GIT_PASSWORD", password);
    if let Some(path) = cwd {
        command.current_dir(path);
    }
    command.output().context("execute git")
}
fn ensure_askpass(state_dir: &Path) -> Result<PathBuf> {
    let path = state_dir.join("git-askpass.sh");
    if !path.exists() {
        fs::write(&path, "#!/bin/sh\ncase \"$1\" in *sername*) printf '%s\\n' \"$FORGE_SYNC_GIT_USER\" ;; *) printf '%s\\n' \"$FORGE_SYNC_GIT_PASSWORD\" ;; esac\n")?;
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            fs::set_permissions(&path, fs::Permissions::from_mode(0o700))?;
        }
    }
    Ok(path)
}
fn safe_name(name: &str) -> String {
    name.chars()
        .map(|c| {
            if c.is_ascii_alphanumeric() || c == '-' || c == '_' || c == '.' {
                c
            } else {
                '_'
            }
        })
        .collect()
}
fn path_str(path: &Path) -> Result<&str> {
    path.to_str().context("state path is not valid UTF-8")
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn sanitizes_repo_names() {
        assert_eq!(safe_name("a/b weird"), "a_b_weird");
    }
}
