//! Deterministic organization-scale acceptance benchmark.
//!
//! The fixture uses local HTTP and Git endpoints so the measurement covers
//! forge-sync itself rather than public-internet latency. Fixture creation is
//! intentionally outside the measured initial and incremental passes.

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

const REPOSITORY_COUNT: usize = 50;
const ISSUES_PER_REPOSITORY: usize = 100;
const ISSUE_COUNT: usize = REPOSITORY_COUNT * ISSUES_PER_REPOSITORY;
const INITIAL_LIMIT_WITH_MARGIN: Duration = Duration::from_secs(24 * 60);
const INCREMENTAL_LIMIT_WITH_MARGIN: Duration = Duration::from_secs(96);

#[derive(Default)]
struct Observed {
    created_repositories: HashSet<String>,
    next_id: i64,
    issue_writes: usize,
}

struct ScaleFixture {
    _temp: tempfile::TempDir,
    base: String,
    config_path: PathBuf,
    state_dir: PathBuf,
    observed: Arc<Mutex<Observed>>,
    stop: Arc<AtomicBool>,
    server: Option<thread::JoinHandle<()>>,
}

impl ScaleFixture {
    fn new() -> Self {
        let temp = tempfile::tempdir().unwrap();
        let source_root = temp.path().join("source");
        let target_root = temp.path().join("target");
        let seed = temp.path().join("seed");
        fs::create_dir_all(&source_root).unwrap();
        fs::create_dir_all(&target_root).unwrap();
        git(&["init", seed.to_str().unwrap()]);
        git(&[
            "-C",
            seed.to_str().unwrap(),
            "config",
            "user.name",
            "Scale fixture",
        ]);
        git(&[
            "-C",
            seed.to_str().unwrap(),
            "config",
            "user.email",
            "scale@example.test",
        ]);
        fs::write(seed.join("README.md"), "# Scale fixture\n").unwrap();
        git(&["-C", seed.to_str().unwrap(), "add", "README.md"]);
        git(&["-C", seed.to_str().unwrap(), "commit", "-m", "seed"]);
        git(&["-C", seed.to_str().unwrap(), "tag", "v1.0.0"]);
        for index in 0..REPOSITORY_COUNT {
            let name = repository_name(index);
            git(&[
                "clone",
                "--bare",
                seed.to_str().unwrap(),
                source_root.join(format!("{name}.git")).to_str().unwrap(),
            ]);
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
            next_id: 10_000,
            ..Default::default()
        }));
        let stop = Arc::new(AtomicBool::new(false));
        let server_observed = observed.clone();
        let server_stop = stop.clone();
        let source_for_server = source_root.clone();
        let target_for_server = target_root.clone();
        let server = thread::spawn(move || {
            while !server_stop.load(Ordering::SeqCst) {
                match listener.accept() {
                    Ok((mut stream, _)) => {
                        let request = read_request(&mut stream);
                        let (status, body) = scale_response(
                            &request,
                            &server_observed,
                            &source_for_server,
                            &target_for_server,
                        );
                        write_response(&mut stream, status, &body);
                    }
                    Err(error) if error.kind() == std::io::ErrorKind::WouldBlock => {
                        thread::sleep(Duration::from_millis(1));
                    }
                    Err(error) => panic!("scale fixture listener failed: {error}"),
                }
            }
        });

        let state_dir = temp.path().join("state");
        let archive_dir = temp.path().join("archive");
        let config_path = temp.path().join("forge-sync.toml");
        fs::write(
            &config_path,
            format!(
                "[source]\norg = 'scale-org'\napi_url = '{base}'\ntoken_env = 'FORGE_SYNC_SCALE_SOURCE'\n\n[target]\nkind = 'forgejo'\nbase_url = '{base}'\nowner = 'scale-mirror'\ntoken_env = 'FORGE_SYNC_SCALE_TARGET'\n\n[sync]\ninterval_seconds = 300\nprivate = true\nstate_dir = '{}'\narchive_dir = '{}'\ngit_archive = true\n",
                state_dir.display(),
                archive_dir.display()
            ),
        )
        .unwrap();
        std::env::set_var("FORGE_SYNC_SCALE_SOURCE", "scale-source-token");
        std::env::set_var("FORGE_SYNC_SCALE_TARGET", "scale-target-token");
        Self {
            _temp: temp,
            base,
            config_path,
            state_dir,
            observed,
            stop,
            server: Some(server),
        }
    }

    fn run(&self) -> engine::RunReport {
        engine::run_once(&Config::from_path(&self.config_path).unwrap()).unwrap()
    }

    fn issue_writes(&self) -> usize {
        self.observed.lock().unwrap().issue_writes
    }
}

impl Drop for ScaleFixture {
    fn drop(&mut self) {
        self.stop.store(true, Ordering::SeqCst);
        let _ = TcpStream::connect(self.base.trim_start_matches("http://"));
        if let Some(server) = self.server.take() {
            server.join().unwrap();
        }
    }
}

#[test]
#[ignore = "organization-scale acceptance benchmark; run through the claim command"]
fn claim_organization_scale_initial_and_incremental_times() {
    // @claim:organization-scale-performance
    let fixture = ScaleFixture::new();

    let initial_started = Instant::now();
    let initial = fixture.run();
    let initial_elapsed = initial_started.elapsed();
    assert_eq!(initial.discovered, REPOSITORY_COUNT);
    assert_eq!(initial.synchronized, REPOSITORY_COUNT);
    assert_eq!(initial.issues, ISSUE_COUNT);
    assert_eq!(initial.failed, 0);
    assert_eq!(fixture.issue_writes(), ISSUE_COUNT);
    assert!(
        initial_elapsed < INITIAL_LIMIT_WITH_MARGIN,
        "initial pass took {initial_elapsed:?}; 20% margin limit is {INITIAL_LIMIT_WITH_MARGIN:?}"
    );

    let incremental_started = Instant::now();
    let incremental = fixture.run();
    let incremental_elapsed = incremental_started.elapsed();
    assert_eq!(incremental.discovered, REPOSITORY_COUNT);
    assert_eq!(incremental.synchronized, REPOSITORY_COUNT);
    assert_eq!(incremental.issues, ISSUE_COUNT);
    assert_eq!(incremental.failed, 0);
    assert_eq!(
        fixture.issue_writes(),
        ISSUE_COUNT,
        "a no-change pass must not rewrite issues"
    );
    assert!(
        incremental_elapsed < INCREMENTAL_LIMIT_WITH_MARGIN,
        "incremental pass took {incremental_elapsed:?}; 20% margin limit is {INCREMENTAL_LIMIT_WITH_MARGIN:?}"
    );

    let status = State::open(&fixture.state_dir).unwrap().status().unwrap();
    assert_eq!(status.repositories, REPOSITORY_COUNT as i64);
    assert_eq!(status.mappings, (ISSUE_COUNT + REPOSITORY_COUNT) as i64);
    eprintln!(
        "scale benchmark: os={} arch={} parallelism={} repositories={} issues={} initial_ms={} incremental_ms={}",
        std::env::consts::OS,
        std::env::consts::ARCH,
        thread::available_parallelism().map_or(1, usize::from),
        REPOSITORY_COUNT,
        ISSUE_COUNT,
        initial_elapsed.as_millis(),
        incremental_elapsed.as_millis()
    );
}

fn scale_response(
    request: &str,
    observed: &Arc<Mutex<Observed>>,
    source_root: &Path,
    target_root: &Path,
) -> (&'static str, String) {
    let line = request.lines().next().unwrap_or_default();
    if line.starts_with("GET /orgs/scale-org/repos?") {
        let repositories = (0..REPOSITORY_COUNT)
            .map(|index| {
                let name = repository_name(index);
                json!({
                    "id": index + 1,
                    "name": name,
                    "full_name": format!("scale-org/{name}"),
                    "description": "Deterministic scale record",
                    "clone_url": source_root.join(format!("{name}.git")).to_string_lossy(),
                    "html_url": format!("https://github.example/scale-org/{name}"),
                    "private": true,
                    "default_branch": "master",
                    "updated_at": "2026-08-28T08:00:00Z"
                })
            })
            .collect::<Vec<_>>();
        return ("200 OK", Value::Array(repositories).to_string());
    }
    if line.starts_with("GET /repos/scale-org/") && line.contains("/labels?") {
        return ("200 OK", "[]".into());
    }
    if line.starts_with("GET /repos/scale-org/") && line.contains("/milestones?") {
        return ("200 OK", "[]".into());
    }
    if line.starts_with("GET /repos/scale-org/") && line.contains("/issues/comments?") {
        return ("200 OK", "[]".into());
    }
    if line.starts_with("GET /repos/scale-org/") && line.contains("/issues?") {
        if !line.contains("&page=1 ") {
            return ("200 OK", "[]".into());
        }
        let name = line
            .strip_prefix("GET /repos/scale-org/")
            .unwrap()
            .split('/')
            .next()
            .unwrap();
        let repository_index: usize = name.trim_start_matches("repo-").parse().unwrap();
        let issues = (0..ISSUES_PER_REPOSITORY)
            .map(|offset| {
                let number = offset + 1;
                let id = repository_index * 10_000 + number;
                json!({
                    "id": id,
                    "number": number,
                    "title": format!("Scale issue {number}"),
                    "body": "Deterministic organization-scale issue.",
                    "state": "open",
                    "user": {"login": "scale-user", "html_url": "https://github.example/scale-user"},
                    "html_url": format!("https://github.example/scale-org/{name}/issues/{number}"),
                    "created_at": "2026-08-27T09:00:00Z",
                    "updated_at": "2026-08-28T08:00:00Z"
                })
            })
            .collect::<Vec<_>>();
        return ("200 OK", Value::Array(issues).to_string());
    }
    if line.starts_with("GET /api/v1/user ") {
        return ("200 OK", r#"{"login":"scale-bot"}"#.into());
    }
    if line.starts_with("GET /api/v1/repos/scale-mirror/") && line.contains("/labels?") {
        return ("200 OK", "[]".into());
    }
    if let Some(name) = line
        .strip_prefix("GET /api/v1/repos/scale-mirror/")
        .and_then(|value| value.split_whitespace().next())
    {
        let exists = observed.lock().unwrap().created_repositories.contains(name);
        if !exists {
            return ("404 Not Found", r#"{"message":"missing"}"#.into());
        }
        return ("200 OK", target_repository(name, target_root).to_string());
    }
    if line.starts_with("POST /api/v1/orgs/scale-mirror/repos ") {
        let name = request_body(request)["name"].as_str().unwrap().to_owned();
        observed
            .lock()
            .unwrap()
            .created_repositories
            .insert(name.clone());
        return ("200 OK", target_repository(&name, target_root).to_string());
    }
    if line.starts_with("POST /api/v1/repos/scale-mirror/") && line.contains("/labels ") {
        let id = next_id(observed);
        return ("200 OK", json!({"id": id}).to_string());
    }
    if line.starts_with("POST /api/v1/repos/scale-mirror/") && line.contains("/issues ") {
        let mut state = observed.lock().unwrap();
        state.next_id += 1;
        state.issue_writes += 1;
        return ("200 OK", json!({"number": state.next_id}).to_string());
    }
    if line.starts_with("PUT /api/v1/repos/scale-mirror/") && line.contains("/labels ") {
        return ("200 OK", "{}".into());
    }
    panic!("unexpected scale fixture request: {line}");
}

fn repository_name(index: usize) -> String {
    format!("repo-{index:02}")
}

fn target_repository(name: &str, root: &Path) -> Value {
    let id: i64 = name.trim_start_matches("repo-").parse().unwrap();
    json!({
        "id": id + 1,
        "name": name,
        "clone_url": root.join(format!("{name}.git")).to_string_lossy(),
        "html_url": format!("https://forge.example/scale-mirror/{name}")
    })
}

fn next_id(observed: &Arc<Mutex<Observed>>) -> i64 {
    let mut state = observed.lock().unwrap();
    state.next_id += 1;
    state.next_id
}

fn request_body(request: &str) -> Value {
    serde_json::from_str(request.split("\r\n\r\n").nth(1).unwrap_or("{}")).unwrap()
}

fn read_request(stream: &mut TcpStream) -> String {
    stream
        .set_read_timeout(Some(Duration::from_secs(5)))
        .unwrap();
    let mut bytes = Vec::new();
    let mut chunk = [0_u8; 8192];
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
                    .and_then(|value| value.trim().parse::<usize>().ok())
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
    write!(
        stream,
        "HTTP/1.1 {status}\r\ncontent-type: application/json\r\ncontent-length: {}\r\nconnection: close\r\n\r\n{body}",
        body.len()
    )
    .unwrap();
}

fn git(args: &[&str]) {
    let output = Command::new("git").args(args).output().unwrap();
    assert!(
        output.status.success(),
        "git {:?} failed: {}",
        args,
        String::from_utf8_lossy(&output.stderr)
    );
}
