//! Configured, local end-to-end proof for the production capabilities named in
//! the README.  The fixture serves GitHub and Forgejo-compatible HTTP APIs and
//! uses real local Git repositories, SQLite state, and archive directories.

use forge_sync::{engine, state::State, Config};
use serde_json::{json, Value};
use std::{
    collections::HashSet,
    fs,
    io::{Read, Write},
    net::{TcpListener, TcpStream},
    path::{Path, PathBuf},
    process::Command,
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc, Mutex,
    },
    thread,
    time::{Duration, Instant},
};

#[derive(Clone, Default)]
struct Observed {
    requests: Vec<String>,
    created_repositories: HashSet<String>,
    next_id: i64,
}

struct Fixture {
    temp: tempfile::TempDir,
    base: String,
    config_path: PathBuf,
    state_dir: PathBuf,
    archive_dir: PathBuf,
    observed: Arc<Mutex<Observed>>,
    stop: Arc<AtomicBool>,
    server: Option<thread::JoinHandle<()>>,
}

impl Fixture {
    fn new(git_archive: bool, interval_seconds: u64) -> Self {
        let temp = tempfile::tempdir().unwrap();
        let source_root = temp.path().join("source");
        let target_root = temp.path().join("target");
        fs::create_dir_all(&source_root).unwrap();
        fs::create_dir_all(&target_root).unwrap();
        for name in ["harbor-tools", "dock-log"] {
            seed_bare_repository(&source_root.join(format!("{name}.git")), name);
            git(&[
                "init",
                "--bare",
                target_root.join(format!("{name}.git")).to_str().unwrap(),
            ]);
        }
        let listener = TcpListener::bind("127.0.0.1:0").unwrap();
        listener.set_nonblocking(true).unwrap();
        let base = format!("http://{}", listener.local_addr().unwrap());
        let observed = Arc::new(Mutex::new(Observed {
            next_id: 100,
            ..Default::default()
        }));
        let stop = Arc::new(AtomicBool::new(false));
        let server_observed = observed.clone();
        let server_stop = stop.clone();
        let source_root_for_server = source_root.clone();
        let target_root_for_server = target_root.clone();
        let server = thread::spawn(move || {
            while !server_stop.load(Ordering::SeqCst) {
                match listener.accept() {
                    Ok((mut stream, _)) => {
                        let request = read_request(&mut stream);
                        let (status, body) = fixture_response(
                            &request,
                            &server_observed,
                            &source_root_for_server,
                            &target_root_for_server,
                        );
                        write_response(&mut stream, status, &body);
                    }
                    Err(error) if error.kind() == std::io::ErrorKind::WouldBlock => {
                        thread::sleep(Duration::from_millis(5))
                    }
                    Err(error) => panic!("fixture listener failed: {error}"),
                }
            }
        });
        let state_dir = temp.path().join("configured-state");
        let archive_dir = temp.path().join("configured-archive");
        let config_path = temp.path().join("forge-sync.toml");
        fs::write(
            &config_path,
            format!(
                "[source]\norg = 'harbor-coop'\napi_url = '{base}'\ntoken_env = 'FORGE_SYNC_PRODUCTION_SOURCE'\n\n[target]\nkind = 'forgejo'\nbase_url = '{base}'\nowner = 'mirror-team'\ntoken_env = 'FORGE_SYNC_PRODUCTION_TARGET'\n\n[sync]\ninterval_seconds = {interval_seconds}\nprivate = true\nstate_dir = '{}'\narchive_dir = '{}'\ngit_archive = {git_archive}\n",
                state_dir.display(), archive_dir.display()
            ),
        ).unwrap();
        std::env::set_var("FORGE_SYNC_PRODUCTION_SOURCE", "source-canary");
        std::env::set_var("FORGE_SYNC_PRODUCTION_TARGET", "target-canary");
        Self {
            temp,
            base,
            config_path,
            state_dir,
            archive_dir,
            observed,
            stop,
            server: Some(server),
        }
    }

    fn config(&self) -> Config {
        Config::from_path(&self.config_path).unwrap()
    }
    fn run(&self) -> engine::RunReport {
        engine::run_once(&self.config()).unwrap()
    }
    fn requests(&self) -> Vec<String> {
        self.observed.lock().unwrap().requests.clone()
    }
}

impl Drop for Fixture {
    fn drop(&mut self) {
        self.stop.store(true, Ordering::SeqCst);
        // Wake the nonblocking accept loop so the thread can finish promptly.
        let _ = TcpStream::connect(self.base.trim_start_matches("http://"));
        if let Some(server) = self.server.take() {
            server.join().unwrap();
        }
    }
}

fn git(args: &[&str]) {
    let output = Command::new("git").args(args).output().unwrap();
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
}

fn seed_bare_repository(bare: &Path, name: &str) {
    git(&["init", "--bare", bare.to_str().unwrap()]);
    let work = bare.parent().unwrap().join(format!("{name}-work"));
    git(&["clone", bare.to_str().unwrap(), work.to_str().unwrap()]);
    git(&[
        "-C",
        work.to_str().unwrap(),
        "config",
        "user.name",
        "Fixture",
    ]);
    git(&[
        "-C",
        work.to_str().unwrap(),
        "config",
        "user.email",
        "fixture@example.test",
    ]);
    fs::write(work.join("README.md"), format!("# {name}\n")).unwrap();
    git(&["-C", work.to_str().unwrap(), "add", "README.md"]);
    git(&["-C", work.to_str().unwrap(), "commit", "-m", "seed"]);
    git(&["-C", work.to_str().unwrap(), "branch", "release/2026"]);
    git(&["-C", work.to_str().unwrap(), "tag", "v2.4.0"]);
    git(&["-C", work.to_str().unwrap(), "push", "--all", "origin"]);
    git(&["-C", work.to_str().unwrap(), "push", "--tags", "origin"]);
}

fn read_request(stream: &mut TcpStream) -> String {
    stream
        .set_read_timeout(Some(Duration::from_secs(2)))
        .unwrap();
    let mut bytes = Vec::new();
    let mut chunk = [0_u8; 4096];
    let mut expected = None;
    loop {
        let read = stream.read(&mut chunk).unwrap();
        if read == 0 {
            break;
        }
        bytes.extend_from_slice(&chunk[..read]);
        if expected.is_none() {
            if let Some(end) = bytes.windows(4).position(|part| part == b"\r\n\r\n") {
                let headers = String::from_utf8_lossy(&bytes[..end]);
                let length = headers
                    .lines()
                    .find_map(|line| {
                        line.strip_prefix("content-length:")
                            .or_else(|| line.strip_prefix("Content-Length:"))
                    })
                    .and_then(|v| v.trim().parse::<usize>().ok())
                    .unwrap_or(0);
                expected = Some(end + 4 + length);
            }
        }
        if expected.is_some_and(|length| bytes.len() >= length) {
            break;
        }
    }
    String::from_utf8(bytes).unwrap()
}

fn write_response(stream: &mut TcpStream, status: &str, body: &str) {
    write!(stream, "HTTP/1.1 {status}\r\ncontent-type: application/json\r\ncontent-length: {}\r\nconnection: close\r\n\r\n{body}", body.len()).unwrap();
}

fn fixture_response(
    request: &str,
    observed: &Arc<Mutex<Observed>>,
    source_root: &Path,
    target_root: &Path,
) -> (&'static str, String) {
    let line = request.lines().next().unwrap_or_default();
    observed.lock().unwrap().requests.push(request.to_owned());
    let source_repo = |name: &str| {
        json!({
            "id": if name == "harbor-tools" { 7 } else { 8 }, "name": name,
            "full_name": format!("harbor-coop/{name}"), "description": format!("{name} record"),
            "clone_url": source_root.join(format!("{name}.git")).to_string_lossy(),
            "html_url": format!("https://github.example/harbor-coop/{name}"), "archived": false,
            "private": true, "default_branch": "master", "updated_at": "2026-08-28T08:00:00Z"
        })
    };
    if line.starts_with("GET /orgs/harbor-coop/repos?") {
        return (
            "200 OK",
            json!([source_repo("harbor-tools"), source_repo("dock-log")]).to_string(),
        );
    }
    if line.starts_with("GET /repos/harbor-coop/") && line.contains("/labels?") {
        return ("200 OK", json!([{"id": 51, "name":"migration", "color":"174d67", "description":"fixture label"}]).to_string());
    }
    if line.starts_with("GET /repos/harbor-coop/") && line.contains("/milestones?") {
        return ("200 OK", json!([{"id":61,"number":4,"title":"Harbor release","description":"fixture milestone","state":"open","due_on":"2026-09-01T00:00:00Z","html_url":"https://github.example/milestone/4"}]).to_string());
    }
    if line.starts_with("GET /repos/harbor-coop/harbor-tools/issues?") {
        return ("200 OK", json!([{"id":4100,"number":41,"title":"Tide alerts","body":"Make warning thresholds readable.","state":"open","user":{"login":"marina","html_url":"https://github.example/marina"},"labels":[{"id":51,"name":"migration","color":"174d67"}],"milestone":{"id":61,"number":4,"title":"Harbor release","state":"open","html_url":"https://github.example/milestone/4"},"pull_request":{"url":"https://github.example/harbor-coop/harbor-tools/pulls/41"},"html_url":"https://github.example/harbor-coop/harbor-tools/pull/41","created_at":"2026-08-27T09:00:00Z","updated_at":"2026-08-28T08:00:00Z"}]).to_string());
    }
    if line.starts_with("GET /repos/harbor-coop/dock-log/issues?") {
        return ("200 OK", json!([{"id":4200,"number":8,"title":"Add berth log","body":"Record berth changes.","state":"open","user":{"login":"niko","html_url":"https://github.example/niko"},"labels":[{"id":51,"name":"migration","color":"174d67"}],"milestone":{"id":61,"number":4,"title":"Harbor release","state":"open","html_url":"https://github.example/milestone/4"},"html_url":"https://github.example/harbor-coop/dock-log/issues/8","created_at":"2026-08-27T11:00:00Z","updated_at":"2026-08-28T08:00:00Z"}]).to_string());
    }
    if line.starts_with("GET /repos/harbor-coop/harbor-tools/issues/comments?") {
        return ("200 OK", json!([{"id":4103,"body":"Please keep the warning copy.","user":{"login":"marina","html_url":"https://github.example/marina"},"html_url":"https://github.example/harbor-coop/harbor-tools/issues/41#issuecomment-4103","created_at":"2026-08-27T10:10:00Z","updated_at":"2026-08-27T10:10:00Z","issue_url":"https://github.example/harbor-coop/harbor-tools/issues/41"}]).to_string());
    }
    if line.starts_with("GET /repos/harbor-coop/dock-log/issues/comments?") {
        return ("200 OK", "[]".into());
    }
    if line.starts_with("GET /repos/harbor-coop/harbor-tools/pulls/41/reviews?") {
        return ("200 OK", json!([{"id":4101,"body":"Approved after mobile review.","user":{"login":"keon","html_url":"https://github.example/keon"},"html_url":"https://github.example/harbor-coop/harbor-tools/pull/41#pullrequestreview-4101","submitted_at":"2026-08-27T10:00:00Z","state":"APPROVED"}]).to_string());
    }
    if line.starts_with("GET /repos/harbor-coop/harbor-tools/pulls/41/comments?") {
        return ("200 OK", json!([{"id":4102,"body":"Name this threshold.","user":{"login":"keon","html_url":"https://github.example/keon"},"html_url":"https://github.example/harbor-coop/harbor-tools/pull/41#discussion_r4102","created_at":"2026-08-27T10:05:00Z","updated_at":"2026-08-27T10:05:00Z","path":"src/alerts.rs","line":18}]).to_string());
    }
    if line.starts_with("GET /api/v1/user ") {
        return ("200 OK", r#"{"login":"mirror-bot"}"#.into());
    }
    if line.starts_with("GET /api/v1/repos/mirror-team/") && line.contains("/labels?") {
        return ("200 OK", "[]".into());
    }
    if let Some(name) = line
        .strip_prefix("GET /api/v1/repos/mirror-team/")
        .and_then(|value| value.split_whitespace().next())
    {
        let exists = observed.lock().unwrap().created_repositories.contains(name);
        if !exists {
            return ("404 Not Found", r#"{"message":"missing"}"#.into());
        }
        return ("200 OK", target_repo(name, target_root).to_string());
    }
    if line.starts_with("POST /api/v1/orgs/mirror-team/repos ") {
        let name = request_body(request)
            .get("name")
            .and_then(Value::as_str)
            .unwrap()
            .to_owned();
        observed
            .lock()
            .unwrap()
            .created_repositories
            .insert(name.clone());
        return ("200 OK", target_repo(&name, target_root).to_string());
    }
    if line.starts_with("POST /api/v1/repos/mirror-team/") && line.contains("/labels ") {
        return ("200 OK", next_id(observed, "id").to_string());
    }
    if line.starts_with("POST /api/v1/repos/mirror-team/") && line.contains("/milestones ") {
        return ("200 OK", next_id(observed, "id").to_string());
    }
    if line.starts_with("POST /api/v1/repos/mirror-team/") && line.contains("/issues ") {
        return ("200 OK", next_id(observed, "number").to_string());
    }
    if line.starts_with("PUT /api/v1/repos/mirror-team/") && line.contains("/labels ") {
        return ("200 OK", "{}".into());
    }
    if line.starts_with("POST /api/v1/repos/mirror-team/") && line.contains("/comments ") {
        return ("200 OK", next_id(observed, "id").to_string());
    }
    panic!("unexpected fixture request: {line}\n{request}");
}

fn request_body(request: &str) -> Value {
    serde_json::from_str(request.split("\r\n\r\n").nth(1).unwrap_or("{}")).unwrap()
}
fn next_id(observed: &Arc<Mutex<Observed>>, field: &str) -> Value {
    let mut state = observed.lock().unwrap();
    state.next_id += 1;
    json!({field: state.next_id})
}
fn target_repo(name: &str, target_root: &Path) -> Value {
    json!({"id": if name == "harbor-tools" { 91 } else { 92 }, "name":name, "clone_url":target_root.join(format!("{name}.git")).to_string_lossy(), "html_url":format!("https://forge.example/mirror-team/{name}")})
}

fn run_and_requests() -> (Fixture, engine::RunReport, Vec<String>) {
    let fixture = Fixture::new(true, 1);
    let report = fixture.run();
    let requests = fixture.requests();
    (fixture, report, requests)
}

#[test]
fn claim_configured_run_writes_selected_state_and_archive_paths() {
    // @claim:configured-run-state-and-archive
    let (fixture, report, _) = run_and_requests();
    assert_eq!(
        (report.discovered, report.synchronized, report.failed),
        (2, 2, 0)
    );
    let status = State::open(&fixture.state_dir).unwrap().status().unwrap();
    assert_eq!(status.repositories, 2);
    assert!(status.mappings >= 8);
    assert!(fixture.archive_dir.join("manifest.json").is_file());
    assert!(fixture
        .archive_dir
        .join("repositories/harbor-tools/items/41.json")
        .is_file());
    assert!(fixture.archive_dir.join(".git").is_dir());
}

#[test]
fn claim_daemon_makes_a_later_configured_pass() {
    // @claim:continuous-daemon-passes
    let fixture = Fixture::new(true, 1);
    let mut child = Command::new(env!("CARGO_BIN_EXE_forge-sync"))
        .args([
            "daemon",
            "--config",
            fixture.config_path.to_str().unwrap(),
            "--json",
        ])
        .env("FORGE_SYNC_PRODUCTION_SOURCE", "source-canary")
        .env("FORGE_SYNC_PRODUCTION_TARGET", "target-canary")
        .spawn()
        .unwrap();
    let deadline = Instant::now() + Duration::from_secs(12);
    while fixture
        .requests()
        .iter()
        .filter(|request| request.starts_with("GET /orgs/harbor-coop/repos?"))
        .count()
        < 2
        && Instant::now() < deadline
    {
        thread::sleep(Duration::from_millis(50));
    }
    let passes = fixture
        .requests()
        .iter()
        .filter(|request| request.starts_with("GET /orgs/harbor-coop/repos?"))
        .count();
    let _ = child.kill();
    let _ = child.wait();
    assert!(passes >= 2, "daemon did not make a later pass: {passes}");
    assert!(fixture.state_dir.join("state.sqlite3").is_file());
}

#[test]
fn claim_configured_run_records_discovery_refs_labels_milestones_and_issues() {
    // @claim:configured-records-metadata
    let (fixture, report, requests) = run_and_requests();
    assert_eq!(report.discovered, 2);
    for endpoint in ["/labels?", "/milestones?", "/issues?"] {
        assert!(
            requests.iter().any(|request| request.contains(endpoint)),
            "missing {endpoint}"
        );
    }
    assert!(requests
        .iter()
        .any(|request| request.starts_with("POST /api/v1/orgs/mirror-team/repos ")));
    assert!(requests.iter().any(|request| request.contains("/labels ")));
    assert!(requests
        .iter()
        .any(|request| request.contains("/milestones ")));
    assert!(requests.iter().any(|request| request.contains("/issues ")));
    assert!(fixture
        .archive_dir
        .join("repositories/harbor-tools/labels.json")
        .is_file());
    assert!(fixture
        .archive_dir
        .join("repositories/harbor-tools/milestones.json")
        .is_file());
    for name in ["harbor-tools.git", "dock-log.git"] {
        let refs = Command::new("git")
            .args([
                "--git-dir",
                fixture
                    .temp
                    .path()
                    .join("target")
                    .join(name)
                    .to_str()
                    .unwrap(),
                "show-ref",
            ])
            .output()
            .unwrap();
        assert!(refs.status.success());
        assert!(String::from_utf8_lossy(&refs.stdout).contains("refs/tags/v2.4.0"));
    }
}

#[test]
fn claim_configured_run_renders_every_pull_request_history_field() {
    // @claim:configured-renders-pull-request-history
    let (_, _, requests) = run_and_requests();
    let text = requests.join("\n");
    assert!(text.contains("forge-sync:pull-request"));
    for field in [
        "Make warning thresholds readable.",
        "Approved after mobile review.",
        "Name this threshold.",
        "Please keep the warning copy.",
        "src/alerts.rs:18",
    ] {
        assert!(text.contains(field), "missing pull-request field {field}");
    }
}

#[test]
fn claim_configured_run_copies_author_time_and_github_links() {
    // @claim:configured-copied-body-attribution
    let (_, _, requests) = run_and_requests();
    let text = requests.join("\n");
    for field in [
        "[@marina](https://github.example/marina)",
        "Originally created 2026-08-27T09:00:00Z",
        "https://github.example/harbor-coop/harbor-tools/pull/41",
        "[@keon](https://github.example/keon)",
        "on 2026-08-27T10:05:00Z",
    ] {
        assert!(text.contains(field), "missing copied attribution {field}");
    }
}

#[test]
fn claim_configured_run_optionally_commits_the_json_archive() {
    // @claim:configured-optional-git-archive
    let with_git = Fixture::new(true, 1);
    with_git.run();
    assert!(with_git.archive_dir.join("manifest.json").is_file());
    assert!(with_git.archive_dir.join(".git").is_dir());
    let without_git = Fixture::new(false, 1);
    without_git.run();
    assert!(without_git.archive_dir.join("manifest.json").is_file());
    assert!(!without_git.archive_dir.join(".git").exists());
}
