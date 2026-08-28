use std::{fs, process::Command};

#[test]
fn claim_demo_completed_mirror_is_isolated_and_inspectable() {
    // @claim:demo-completed-mirror
    let before = std::env::current_dir().unwrap();
    let output = Command::new(env!("CARGO_BIN_EXE_forge-sync"))
        .current_dir(&before)
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
    assert_eq!(report["repositories"], 1);
    assert_eq!(report["issues"], 1);
    assert_eq!(report["pull_request_records"], 1);
    assert!(report["archive_commit"].as_bool().unwrap());
    assert!(root.join("target/harbor-tools/branches.txt").exists());
    assert!(root.join("target/harbor-tools/tags.txt").exists());
    assert!(root.join("target/harbor-tools/issues/41.md").exists());
    let item =
        fs::read_to_string(root.join("archive/repositories/harbor-tools/items/41.json")).unwrap();
    assert!(item.contains("inline_comments") && item.contains("discussion_comments"));
    assert!(root.join("state/id-mappings.json").exists());
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
