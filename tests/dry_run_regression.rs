//! Regression for the release-blocking sequence found by independent verification:
//! a clean dry run must not make a following real run believe target objects exist.

use forge_sync::{engine, state::State, Config};
use std::{
    io::{Read, Write},
    net::TcpListener,
    process::Command,
    sync::{Arc, Mutex},
    thread,
};

fn git(args: &[&str]) {
    let output = Command::new("git").args(args).output().unwrap();
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
}

fn mock(
    source_git_url: String,
    target_git_url: String,
) -> (String, Arc<Mutex<Vec<String>>>, thread::JoinHandle<()>) {
    let listener = TcpListener::bind("127.0.0.1:0").unwrap();
    let address = format!("http://{}", listener.local_addr().unwrap());
    let requests = Arc::new(Mutex::new(Vec::new()));
    let observed = requests.clone();
    let server = thread::spawn(move || {
        // Dry run: 5 GitHub reads + 2 target reads. Real run: 5 GitHub reads
        // plus target doctor, lookup, repo creation, label lookup/creation,
        // issue creation, and label assignment.
        for _ in 0..19 {
            let (mut stream, _) = listener.accept().unwrap();
            let mut bytes = [0_u8; 8192];
            let read = stream.read(&mut bytes).unwrap();
            let request = String::from_utf8_lossy(&bytes[..read]);
            let line = request.lines().next().unwrap().to_owned();
            observed.lock().unwrap().push(line.clone());
            let (status, body) = response(&line, &source_git_url, &target_git_url);
            write!(
                stream,
                "HTTP/1.1 {status}\r\ncontent-type: application/json\r\ncontent-length: {}\r\nconnection: close\r\n\r\n{body}",
                body.len()
            )
            .unwrap();
        }
    });
    (address, requests, server)
}

fn response(request: &str, source_git_url: &str, target_git_url: &str) -> (&'static str, String) {
    let github_repo = r#"[{"id":7,"name":"demo","full_name":"acme/demo","clone_url":"REPLACED","html_url":"https://github.test/acme/demo","archived":false,"private":true,"default_branch":"main","updated_at":"2026-08-27T00:00:00Z"}]"#;
    if request.starts_with("GET /orgs/acme/repos?") {
        return ("200 OK", github_repo.replace("REPLACED", source_git_url));
    }
    if request.starts_with("GET /repos/acme/demo/issues?state=") {
        return (
            "200 OK",
            r#"[{"id":101,"number":1,"title":"Move this issue","body":"body","state":"open","user":{"login":"octo","html_url":"https://github.test/octo"},"labels":[],"html_url":"https://github.test/acme/demo/issues/1","created_at":"2026-08-27T00:00:00Z","updated_at":"2026-08-27T00:00:00Z"}]"#.into(),
        );
    }
    if request.starts_with("GET /repos/acme/demo/labels?")
        || request.starts_with("GET /repos/acme/demo/milestones?")
        || request.starts_with("GET /repos/acme/demo/issues/comments?")
    {
        return ("200 OK", "[]".into());
    }
    if request.starts_with("GET /api/v1/user ") {
        return ("200 OK", r#"{"login":"mirror-bot"}"#.into());
    }
    if request.starts_with("GET /api/v1/repos/team/demo ") {
        return ("404 Not Found", r#"{"message":"missing"}"#.into());
    }
    if request.starts_with("POST /api/v1/orgs/team/repos ") {
        return (
            "200 OK",
            format!(
                r#"{{"id":9,"name":"demo","clone_url":"{target_git_url}","html_url":"https://forge.test/team/demo"}}"#
            ),
        );
    }
    if request.starts_with("GET /api/v1/repos/team/demo/labels?") {
        return ("200 OK", "[]".into());
    }
    if request.starts_with("POST /api/v1/repos/team/demo/labels ") {
        return ("200 OK", r#"{"id":31}"#.into());
    }
    if request.starts_with("POST /api/v1/repos/team/demo/issues ") {
        return ("200 OK", r#"{"number":41}"#.into());
    }
    if request.starts_with("PUT /api/v1/repos/team/demo/issues/41/labels ") {
        return ("200 OK", "{}".into());
    }
    panic!("unexpected mock request: {request}");
}

#[test]
fn dry_run_then_real_run_creates_every_reported_target_object_without_durable_dry_state() {
    // @claim:dry-run-read-only
    let temp = tempfile::tempdir().unwrap();
    let source = temp.path().join("source.git");
    let target = temp.path().join("target.git");
    let work = temp.path().join("work");
    git(&["init", "--bare", source.to_str().unwrap()]);
    git(&["init", "--bare", target.to_str().unwrap()]);
    git(&["clone", source.to_str().unwrap(), work.to_str().unwrap()]);
    git(&["-C", work.to_str().unwrap(), "config", "user.name", "Test"]);
    git(&[
        "-C",
        work.to_str().unwrap(),
        "config",
        "user.email",
        "test@example.test",
    ]);
    std::fs::write(work.join("README.md"), "mirror me\n").unwrap();
    git(&["-C", work.to_str().unwrap(), "add", "README.md"]);
    git(&["-C", work.to_str().unwrap(), "commit", "-m", "seed"]);
    git(&["-C", work.to_str().unwrap(), "push", "origin", "HEAD"]);
    let (base, requests, server) = mock(
        source.to_string_lossy().into_owned(),
        target.to_string_lossy().into_owned(),
    );

    let mut config = Config {
        source: forge_sync::config::SourceConfig {
            org: "acme".into(),
            token_env: "FORGE_SYNC_REGRESSION_GH_TOKEN".into(),
            api_url: base.clone(),
        },
        target: forge_sync::config::TargetConfig {
            kind: forge_sync::config::TargetKind::Forgejo,
            base_url: base,
            owner: "team".into(),
            token_env: "FORGE_SYNC_REGRESSION_TARGET_TOKEN".into(),
        },
        sync: forge_sync::config::SyncConfig {
            interval_seconds: 30,
            include_archived: true,
            private: true,
            state_dir: temp.path().join("state"),
            archive_dir: temp.path().join("archive"),
            git_archive: false,
            repos: vec![],
            exclude: vec![],
            dry_run: true,
            experimental_comment_relay: false,
        },
    };
    std::env::set_var("FORGE_SYNC_REGRESSION_GH_TOKEN", "source-token");
    std::env::set_var("FORGE_SYNC_REGRESSION_TARGET_TOKEN", "target-token");

    let plan = engine::run_once(&config).unwrap();
    assert_eq!((plan.discovered, plan.issues, plan.failed), (1, 1, 0));
    assert!(plan.dry_run);
    assert!(!config.sync.state_dir.exists(), "dry run created state");
    assert!(!config.sync.archive_dir.exists(), "dry run created archive");

    config.sync.dry_run = false;
    let actual = engine::run_once(&config).unwrap();
    assert_eq!(
        (actual.discovered, actual.issues, actual.failed),
        (1, 1, 0),
        "real-run errors: {:?}",
        actual.errors
    );
    assert!(!actual.dry_run);
    assert_eq!(
        State::open(&config.sync.state_dir)
            .unwrap()
            .status()
            .unwrap()
            .mappings,
        2
    );

    server.join().unwrap();
    let requests = requests.lock().unwrap();
    assert!(
        requests[..7]
            .iter()
            .all(|request| !request.starts_with("POST ") && !request.starts_with("PUT ")),
        "dry run made a target write: {:?}",
        &requests[..7]
    );
    assert!(
        requests
            .iter()
            .any(|request| request.starts_with("POST /api/v1/repos/team/demo/labels ")),
        "real run did not create the label: {requests:?}"
    );
    assert!(
        requests
            .iter()
            .any(|request| request.starts_with("POST /api/v1/repos/team/demo/issues ")),
        "real run did not create the reported issue: {requests:?}"
    );
}
