use std::{
    fs,
    io::{Read, Write},
    net::TcpListener,
    process::Command,
    thread,
};

#[test]
fn claim_doctor_checks_both_identities_with_gets_and_writes_nothing() {
    // @claim:doctor-read-only
    let listener = TcpListener::bind("127.0.0.1:0").unwrap();
    let base = format!("http://{}", listener.local_addr().unwrap());
    let server = thread::spawn(move || {
        for (expected_path, expected_auth) in [
            ("GET /user ", "authorization: bearer source-canary"),
            ("GET /api/v1/user ", "authorization: token target-canary"),
        ] {
            let (mut stream, _) = listener.accept().unwrap();
            let mut bytes = [0_u8; 4096];
            let read = stream.read(&mut bytes).unwrap();
            let request = String::from_utf8_lossy(&bytes[..read]);
            assert!(request.starts_with(expected_path), "{request}");
            assert!(
                request.to_ascii_lowercase().contains(expected_auth),
                "{request}"
            );
            let body = r#"{"login":"mirror-check"}"#;
            write!(stream, "HTTP/1.1 200 OK\r\ncontent-type: application/json\r\ncontent-length: {}\r\nconnection: close\r\n\r\n{body}", body.len()).unwrap();
        }
    });
    let temp = tempfile::tempdir().unwrap();
    let state = temp.path().join("state-that-must-not-exist");
    let archive = temp.path().join("archive-that-must-not-exist");
    let config = temp.path().join("forge-sync.toml");
    fs::write(
        &config,
        format!(
            "[source]\norg='harbor-coop'\napi_url='{base}'\ntoken_env='CLAIM_DOCTOR_SOURCE'\n[target]\nkind='forgejo'\nbase_url='{base}'\nowner='harbor-coop'\ntoken_env='CLAIM_DOCTOR_TARGET'\n[sync]\nstate_dir='{}'\narchive_dir='{}'\n",
            state.display(),
            archive.display()
        ),
    )
    .unwrap();
    let output = Command::new(env!("CARGO_BIN_EXE_forge-sync"))
        .args(["doctor", "--config", config.to_str().unwrap(), "--json"])
        .env("CLAIM_DOCTOR_SOURCE", "source-canary")
        .env("CLAIM_DOCTOR_TARGET", "target-canary")
        .output()
        .unwrap();
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    assert_eq!(
        serde_json::from_slice::<serde_json::Value>(&output.stdout).unwrap()["ok"],
        true
    );
    server.join().unwrap();
    assert!(!state.exists());
    assert!(!archive.exists());
}
