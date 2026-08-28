use std::{fs, process::Command};

#[test]
fn claim_demo_completed_mirror_is_isolated_and_inspectable() {
    // @claim:demo-completed-mirror
    let before = std::env::current_dir().unwrap();
    let sentinel = tempfile::tempdir().unwrap();
    fs::write(sentinel.path().join("forge-sync.toml"), "not valid toml").unwrap();
    let output = Command::new(env!("CARGO_BIN_EXE_forge-sync"))
        .current_dir(sentinel.path())
        .env("GITHUB_TOKEN", "claim-source-canary")
        .env("FORGE_TOKEN", "claim-target-canary")
        .args(["demo", "--json"])
        .output()
        .unwrap();
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    let report: serde_json::Value = serde_json::from_slice(&output.stdout).unwrap();
    let root = std::path::PathBuf::from(report["path"].as_str().unwrap());
    assert!(root.starts_with(std::env::temp_dir()));
    assert_ne!(root, before);
    assert_ne!(root, sentinel.path());
    assert_eq!(
        fs::read_to_string(sentinel.path().join("forge-sync.toml")).unwrap(),
        "not valid toml"
    );
    assert_eq!(report["repositories"], 1);
    assert_eq!(report["issues"], 1);
    assert_eq!(report["pull_request_records"], 1);
    assert!(report["archive_commit"].as_bool().unwrap());
    assert!(root.join("target/harbor-tools/branches.txt").exists());
    assert!(root.join("target/harbor-tools/tags.txt").exists());
    assert!(root.join("target/harbor-tools/issues/41.md").exists());
    let item =
        fs::read_to_string(root.join("archive/repositories/harbor-tools/items/41.json")).unwrap();
    assert!(item.contains("\"comments\"") && item.contains("inline review comment"));
    assert!(item.contains("review (approved)") && item.contains("Updated the copy for mobile."));
    let rendered = fs::read_to_string(root.join("target/harbor-tools/issues/41.md")).unwrap();
    assert!(rendered.contains("[GitHub PR #41]") && rendered.contains("src/alerts.rs:18"));
    assert!(rendered.contains("https://github.example/harbor-coop/harbor-tools/pull/41"));
    let all_output = walk_text(&root);
    assert!(!all_output.contains("claim-source-canary"));
    assert!(!all_output.contains("claim-target-canary"));
    let state = forge_sync::state::State::open(&root.join("state")).unwrap();
    let status = state.status().unwrap();
    assert_eq!(status.repositories, 1);
    assert_eq!(status.mappings, 1);
    assert_eq!(status.audit_events, 3);
    assert_eq!(
        state
            .mapping("item", "harbor-tools", 4100)
            .unwrap()
            .unwrap()
            .0,
        41
    );
    assert_eq!(
        Command::new("git")
            .args([
                "-C",
                root.join("archive").to_str().unwrap(),
                "log",
                "-1",
                "--format=%s"
            ])
            .output()
            .unwrap()
            .status
            .code(),
        Some(0)
    );
    fs::remove_dir_all(root).unwrap();
}

fn walk_text(root: &std::path::Path) -> String {
    let mut text = String::new();
    for entry in fs::read_dir(root).unwrap().flatten() {
        let path = entry.path();
        if path.file_name().is_some_and(|name| name == ".git") {
            continue;
        }
        if path.is_dir() {
            text.push_str(&walk_text(&path));
        } else if let Ok(value) = fs::read_to_string(path) {
            text.push_str(&value);
        }
    }
    text
}
