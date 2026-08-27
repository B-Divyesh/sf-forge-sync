use forge_sync::{
    config::{TargetConfig, TargetKind},
    forge::Forge,
    github::GitHub,
};
use std::{
    io::{Read, Write},
    net::TcpListener,
    thread,
};

fn one_response(
    body: &'static str,
    expected_auth: &'static str,
) -> (String, thread::JoinHandle<()>) {
    let listener = TcpListener::bind("127.0.0.1:0").unwrap();
    let address = format!("http://{}", listener.local_addr().unwrap());
    let server = thread::spawn(move || {
        let (mut stream, _) = listener.accept().unwrap();
        let mut request = [0_u8; 4096];
        let read = stream.read(&mut request).unwrap();
        let request = String::from_utf8_lossy(&request[..read]);
        assert!(request.to_ascii_lowercase().contains(expected_auth));
        write!(
            stream,
            "HTTP/1.1 200 OK\r\ncontent-type: application/json\r\ncontent-length: {}\r\nconnection: close\r\n\r\n{}",
            body.len(),
            body
        )
        .unwrap();
    });
    (address, server)
}

#[test]
fn github_doctor_uses_bearer_auth_and_decodes_identity() {
    let (base, server) = one_response(
        r#"{"login":"mirror-bot","html_url":"https://example.test/mirror-bot"}"#,
        "authorization: bearer source-secret",
    );
    let github = GitHub::new(&base, "acme", "source-secret".into()).unwrap();
    assert_eq!(github.doctor().unwrap().login, "mirror-bot");
    server.join().unwrap();
}

#[test]
fn forgejo_doctor_uses_token_auth() {
    let (base, server) = one_response(
        r#"{"login":"target-bot"}"#,
        "authorization: token target-secret",
    );
    let forge = Forge::new(
        TargetConfig {
            kind: TargetKind::Forgejo,
            base_url: base,
            owner: "acme-mirror".into(),
            token_env: "FORGE_TOKEN".into(),
        },
        "target-secret".into(),
    )
    .unwrap();
    assert_eq!(forge.doctor().unwrap(), "target-bot");
    server.join().unwrap();
}
