use forge_sync::{gitmirror, model::Repository};
use std::{path::Path, process::Command};

fn git(cwd: Option<&Path>, args: &[&str]) {
    let mut command = Command::new("git");
    command.args(args);
    if let Some(path) = cwd {
        command.current_dir(path);
    }
    let output = command.output().unwrap();
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
}

#[test]
fn mirrors_branches_and_tags_between_real_local_repositories() {
    let temp = tempfile::tempdir().unwrap();
    let source = temp.path().join("source.git");
    let target = temp.path().join("target.git");
    let work = temp.path().join("work");
    git(None, &["init", "--bare", source.to_str().unwrap()]);
    git(None, &["init", "--bare", target.to_str().unwrap()]);
    git(
        None,
        &["clone", source.to_str().unwrap(), work.to_str().unwrap()],
    );
    git(Some(&work), &["config", "user.name", "Test"]);
    git(Some(&work), &["config", "user.email", "test@example.test"]);
    std::fs::write(work.join("README.md"), "portable\n").unwrap();
    git(Some(&work), &["add", "README.md"]);
    git(Some(&work), &["commit", "-m", "initial"]);
    git(Some(&work), &["tag", "v0.1.0"]);
    git(Some(&work), &["push", "origin", "HEAD", "--tags"]);
    let repo = Repository {
        id: 1,
        name: "demo".into(),
        full_name: "acme/demo".into(),
        description: None,
        clone_url: source.to_string_lossy().into(),
        html_url: "https://example.test/acme/demo".into(),
        archived: false,
        private: true,
        default_branch: "master".into(),
        updated_at: "2026-08-27T00:00:00Z".into(),
    };
    gitmirror::mirror(
        &repo,
        target.to_str().unwrap(),
        "token",
        "",
        "",
        &temp.path().join("state"),
    )
    .unwrap();
    let refs = Command::new("git")
        .args(["show-ref"])
        .current_dir(&target)
        .output()
        .unwrap();
    let refs = String::from_utf8(refs.stdout).unwrap();
    assert!(refs.contains("refs/heads/master"));
    assert!(refs.contains("refs/tags/v0.1.0"));
}
