mod common;

use assert_cmd::Command;
use predicates::prelude::predicate;
use tempfile::TempDir;

fn cmd() -> Command {
    Command::cargo_bin("{{project-name}}").expect("binary should exist")
}

#[test]
fn hello_default_name() {
    cmd()
        .arg("hello")
        .assert()
        .success()
        .stdout(predicate::str::contains(r#""greeting":"Hello, world!"#));
}

#[test]
fn hello_custom_name() {
    cmd()
        .args(["hello", "Alice"])
        .assert()
        .success()
        .stdout(predicate::str::contains(r#""greeting":"Hello, Alice!"#));
}

#[test]
fn config_list() {
    let tmp = TempDir::new().expect("failed to create temp dir");
    cmd()
        .env("APP_DATA_DIR", tmp.path())
        .args(["config", "list"])
        .assert()
        .success()
        .stdout(predicate::str::contains(r#""ok":true"#))
        .stdout(predicate::str::contains(r#""entries""#));
}

#[test]
fn config_set_then_get() {
    let tmp = TempDir::new().expect("failed to create temp dir");
    let dir = tmp.path();

    // Set a value
    cmd()
        .env("APP_DATA_DIR", dir)
        .args(["config", "set", "example.setting", "hello42"])
        .assert()
        .success()
        .stdout(predicate::str::contains(r#""ok":true"#));

    // Get the value back
    cmd()
        .env("APP_DATA_DIR", dir)
        .args(["config", "get", "example.setting"])
        .assert()
        .success()
        .stdout(predicate::str::contains(r#""value":"hello42"#));
}

#[test]
fn config_set_unknown_key_fails() {
    let tmp = TempDir::new().expect("failed to create temp dir");
    cmd()
        .env("APP_DATA_DIR", tmp.path())
        .args(["config", "set", "no.such.key", "val"])
        .assert()
        .failure()
        .stdout(predicate::str::contains(r#""ok":false"#));
}

#[test]
fn config_get_unknown_key_fails() {
    let tmp = TempDir::new().expect("failed to create temp dir");
    cmd()
        .env("APP_DATA_DIR", tmp.path())
        .args(["config", "get", "no.such.key"])
        .assert()
        .failure()
        .stdout(predicate::str::contains(r#""ok":false"#));
}

#[test]
fn agent_describe_outputs_valid_schema() {
    let assert = cmd()
        .arg("--agent-describe")
        .assert()
        .success();

    let output = assert.get_output();
    let stdout = String::from_utf8_lossy(&output.stdout);
    let schema: serde_json::Value = serde_json::from_str(&stdout)
        .expect("--agent-describe should output valid JSON");

    assert_eq!(schema["protocol"], "agent-cli/1");
    assert!(schema["commands"].as_array().unwrap().len() > 0);
}
