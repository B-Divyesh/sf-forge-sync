use std::process::Command;

fn command(config: &std::path::Path, subcommand: &str) -> std::process::Output {
    Command::new(env!("CARGO_BIN_EXE_forge-sync"))
        .args([subcommand, "--config", config.to_str().unwrap(), "--json"])
        .output()
        .unwrap()
}

#[test]
fn all_documented_configuration_failures_use_exit_code_two() {
    let temp = tempfile::tempdir().unwrap();
    let invalid = temp.path().join("invalid.toml");
    std::fs::write(
        &invalid,
        r#"
[source]
org = ""

[target]
kind = "forgejo"
base_url = "not-a-url"
owner = "team"
"#,
    )
    .unwrap();
    let invalid_output = command(&invalid, "status");
    assert_eq!(invalid_output.status.code(), Some(2));
    assert!(String::from_utf8_lossy(&invalid_output.stderr).contains("configuration error"));

    let missing_token = temp.path().join("missing-token.toml");
    std::fs::write(
        &missing_token,
        r#"
[source]
org = "acme"
api_url = "https://github.example"
token_env = "FORGE_SYNC_TEST_MISSING_SOURCE_TOKEN"

[target]
kind = "forgejo"
base_url = "https://forge.example"
owner = "team"
token_env = "FORGE_SYNC_TEST_MISSING_TARGET_TOKEN"
"#,
    )
    .unwrap();
    let token_output = command(&missing_token, "sync");
    assert_eq!(token_output.status.code(), Some(2));
    assert!(String::from_utf8_lossy(&token_output.stderr).contains("configuration error"));
}
