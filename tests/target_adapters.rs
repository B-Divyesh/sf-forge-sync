use forge_sync::{
    config::{TargetConfig, TargetKind},
    forge::Forge,
    model::Repository,
};
use std::{
    io::{Read, Write},
    net::TcpListener,
    thread,
};

fn request(stream: &mut std::net::TcpStream) -> String {
    let mut bytes = [0_u8; 8192];
    let read = stream.read(&mut bytes).unwrap();
    String::from_utf8_lossy(&bytes[..read]).into_owned()
}

fn respond(stream: &mut std::net::TcpStream, status: &str, body: &str) {
    write!(
        stream,
        "HTTP/1.1 {status}\r\ncontent-type: application/json\r\ncontent-length: {}\r\nconnection: close\r\n\r\n{body}",
        body.len()
    )
    .unwrap();
}

fn check_adapter(kind: TargetKind) {
    let listener = TcpListener::bind("127.0.0.1:0").unwrap();
    let base = format!("http://{}", listener.local_addr().unwrap());
    let adapter = kind;
    let server = thread::spawn(move || {
        let (mut first, _) = listener.accept().unwrap();
        let lookup = request(&mut first);
        match adapter {
            TargetKind::Forgejo | TargetKind::Codeberg => {
                assert!(lookup.starts_with("GET /api/v1/repos/harbor-coop/harbor-tools "));
                assert!(lookup
                    .to_ascii_lowercase()
                    .contains("authorization: token target-canary"));
            }
            TargetKind::Gitlab => {
                assert!(lookup.starts_with("GET /api/v4/projects/harbor%2Dcoop%2Fharbor%2Dtools "));
                assert!(lookup
                    .to_ascii_lowercase()
                    .contains("private-token: target-canary"));
            }
        }
        respond(&mut first, "404 Not Found", r#"{"message":"missing"}"#);

        if adapter == TargetKind::Gitlab {
            let (mut namespace, _) = listener.accept().unwrap();
            let namespace_request = request(&mut namespace);
            assert!(namespace_request.starts_with("GET /api/v4/namespaces/harbor%2Dcoop "));
            respond(&mut namespace, "200 OK", r#"{"id":72}"#);
        }

        let (mut create, _) = listener.accept().unwrap();
        let create_request = request(&mut create);
        match adapter {
            TargetKind::Forgejo | TargetKind::Codeberg => {
                assert!(create_request.starts_with("POST /api/v1/orgs/harbor-coop/repos "));
                assert!(create_request.contains(r#""private":true"#));
                respond(
                    &mut create,
                    "200 OK",
                    r#"{"id":41,"name":"harbor-tools","clone_url":"https://forge.test/harbor-coop/harbor-tools.git","html_url":"https://forge.test/harbor-coop/harbor-tools"}"#,
                );
            }
            TargetKind::Gitlab => {
                assert!(create_request.starts_with("POST /api/v4/projects "));
                assert!(create_request.contains(r#""visibility":"private""#));
                assert!(create_request.contains(r#""namespace_id":72"#));
                respond(
                    &mut create,
                    "200 OK",
                    r#"{"id":41,"name":"harbor-tools","http_url_to_repo":"https://forge.test/harbor-coop/harbor-tools.git","web_url":"https://forge.test/harbor-coop/harbor-tools"}"#,
                );
            }
        }
    });

    let fixture: Repository =
        serde_json::from_str(include_str!("../examples/sample-mirror/repository.json")).unwrap();
    let result = Forge::new(
        TargetConfig {
            kind,
            base_url: base,
            owner: "harbor-coop".into(),
            token_env: "FORGE_TOKEN".into(),
        },
        "target-canary".into(),
    )
    .unwrap()
    .ensure_repo(&fixture, true, false)
    .unwrap();
    assert_eq!(result.name, "harbor-tools");
    assert_eq!(result.id, "41");
    server.join().unwrap();
}

#[test]
fn claim_supported_target_adapters_use_their_real_api_contracts() {
    // @claim:supported-targets
    for kind in [
        TargetKind::Forgejo,
        TargetKind::Codeberg,
        TargetKind::Gitlab,
    ] {
        check_adapter(kind);
    }
}
