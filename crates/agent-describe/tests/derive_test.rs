use agent_describe::AgentDescribe;
use clap::{Parser, Subcommand};
use schemars::JsonSchema;
use serde::Serialize;

#[derive(Parser)]
#[command(name = "testcli", version = "1.0.0", about = "A test CLI")]
struct TestCli {
    #[command(subcommand)]
    command: TestCommand,
}

#[derive(Subcommand, AgentDescribe)]
#[agent(cli = TestCli)]
enum TestCommand {
    /// Deploy to an environment
    Deploy {
        /// Target environment
        env: String,
        /// Skip actual deployment
        #[arg(long)]
        dry_run: bool,
    },

    /// Show greeting
    #[agent(skip)]
    Hello {
        name: String,
    },

    /// Check system status
    #[agent(output = CustomStatusOutput)]
    Status {
        /// Component to check
        component: String,
    },

    /// Manage config
    Config {
        #[command(subcommand)]
        action: ConfigAction,
    },
}

#[derive(Subcommand)]
enum ConfigAction {
    /// Set a value
    Set {
        /// Config key
        key: String,
        /// Config value
        value: String,
    },
    /// Get a value
    Get {
        /// Config key
        key: String,
    },
}

#[derive(Serialize, JsonSchema)]
struct DeployResult {
    url: String,
    took_secs: f64,
}

#[derive(Serialize, JsonSchema)]
struct CustomStatusOutput {
    healthy: bool,
    uptime_secs: u64,
}

#[test]
fn schema_includes_deploy_but_not_hello() {
    let schema = TestCommand::agent_schema();

    assert_eq!(schema["protocol"], "agent-cli/1");
    assert_eq!(schema["name"], "testcli");

    let commands = schema["commands"].as_array().unwrap();
    let names: Vec<&str> = commands.iter().map(|c| c["name"].as_str().unwrap()).collect();

    assert!(names.contains(&"deploy"), "deploy should be in schema");
    assert!(!names.contains(&"hello"), "hello should be skipped");
}

#[test]
fn deploy_has_args_and_output() {
    let schema = TestCommand::agent_schema();

    let commands = schema["commands"].as_array().unwrap();
    let deploy = commands.iter().find(|c| c["name"] == "deploy").unwrap();

    assert_eq!(deploy["description"], "Deploy to an environment");

    let args = deploy["args"].as_array().unwrap();
    assert!(args.iter().any(|a| a["name"] == "env"), "should have env arg");
    assert!(
        args.iter()
            .any(|a| a["name"] == "--dry-run" && a["type"] == "bool"),
        "should have --dry-run flag"
    );

    // Output schema should exist and have DeployResult properties
    let output = &deploy["output"];
    assert!(output.is_object(), "output schema should be present");
}

#[test]
fn schema_has_error_format() {
    let schema = TestCommand::agent_schema();
    assert!(schema["error_format"].is_object());
    assert!(schema["error_format"]["properties"]["error"].is_object());
}

#[test]
fn output_override_uses_custom_type() {
    let schema = TestCommand::agent_schema();
    let commands = schema["commands"].as_array().unwrap();
    let status = commands.iter().find(|c| c["name"] == "status").unwrap();

    assert_eq!(status["description"], "Check system status");

    // Output schema should reflect CustomStatusOutput, not StatusResult
    let output = &status["output"];
    assert!(output.is_object(), "output schema should be present");

    // Verify the schema title matches the custom type, not the default StatusResult
    assert_eq!(
        output["title"], "CustomStatusOutput",
        "output type should be CustomStatusOutput, not StatusResult"
    );

    let props = &output["properties"];
    assert!(
        props["healthy"].is_object(),
        "should have 'healthy' from CustomStatusOutput"
    );
    assert!(
        props["uptime_secs"].is_object(),
        "should have 'uptime_secs' from CustomStatusOutput"
    );
}

#[test]
fn subcommand_flattening_produces_separate_commands() {
    let schema = TestCommand::agent_schema();
    let commands = schema["commands"].as_array().unwrap();
    let names: Vec<&str> = commands.iter().map(|c| c["name"].as_str().unwrap()).collect();

    // Flattened subcommands should appear as "config set" and "config get"
    assert!(
        names.contains(&"config set"),
        "should have flattened 'config set', got: {:?}",
        names
    );
    assert!(
        names.contains(&"config get"),
        "should have flattened 'config get', got: {:?}",
        names
    );

    // "config" alone should NOT appear
    assert!(
        !names.contains(&"config"),
        "'config' should not appear as standalone command"
    );

    // Verify args on "config set"
    let config_set = commands
        .iter()
        .find(|c| c["name"] == "config set")
        .unwrap();
    let args = config_set["args"].as_array().unwrap();
    let arg_names: Vec<&str> = args.iter().map(|a| a["name"].as_str().unwrap()).collect();
    assert!(
        arg_names.contains(&"key"),
        "config set should have 'key' arg"
    );
    assert!(
        arg_names.contains(&"value"),
        "config set should have 'value' arg"
    );

    // Verify args on "config get"
    let config_get = commands
        .iter()
        .find(|c| c["name"] == "config get")
        .unwrap();
    let args = config_get["args"].as_array().unwrap();
    let arg_names: Vec<&str> = args.iter().map(|a| a["name"].as_str().unwrap()).collect();
    assert!(
        arg_names.contains(&"key"),
        "config get should have 'key' arg"
    );
    assert_eq!(args.len(), 1, "config get should have exactly 1 arg");
}
