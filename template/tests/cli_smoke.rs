//! Smoke tests for CLI commands.

mod common;

#[test]
fn help_flag_succeeds() {
    common::cmd().arg("--help").assert().success();
}

#[test]
fn version_flag_succeeds() {
    common::cmd().arg("--version").assert().success();
}

#[test]
fn agent_describe_outputs_valid_json() {
    let output = common::cmd()
        .arg("--agent-describe")
        .output()
        .expect("command should execute");

    assert!(output.status.success());

    let json: serde_json::Value =
        serde_json::from_slice(&output.stdout).expect("output should be valid JSON");

    assert_eq!(json["protocol"], "agent-cli/1");
    assert!(json["commands"].is_array());
}

#[test]
fn hello_command_returns_agent_response() {
    let output = common::cmd()
        .args(["hello", "Alice"])
        .output()
        .expect("command should execute");

    assert!(output.status.success());

    let json: serde_json::Value =
        serde_json::from_slice(&output.stdout).expect("output should be valid JSON");

    assert_eq!(json["ok"], true);
    assert!(json["data"]["greeting"].as_str().unwrap().contains("Alice"));
}

#[test]
fn unknown_command_fails() {
    common::cmd()
        .arg("nonexistent")
        .assert()
        .failure();
}
