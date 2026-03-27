//! Typed response structs for each CLI command.
//!
//! Naming convention: command `Foo` → `FooResult`.
//! Each type derives `Serialize` + `JsonSchema` for agent-describe schema generation.

use schemars::JsonSchema;
use serde::Serialize;

/// Result of the `hello` command.
#[derive(Debug, Serialize, JsonSchema)]
pub struct HelloResult {
    /// The greeting message.
    pub greeting: String,
}

/// Result of `config set`.
#[derive(Debug, Serialize, JsonSchema)]
pub struct ConfigSetResult {
    /// The key that was set.
    pub key: String,
    /// The value that was set.
    pub value: String,
}

/// Result of `config get`.
#[derive(Debug, Serialize, JsonSchema)]
pub struct ConfigGetResult {
    /// The config key.
    pub key: String,
    /// The config value, or null if not set.
    pub value: Option<String>,
}

/// Result of `config list`.
#[derive(Debug, Serialize, JsonSchema)]
pub struct ConfigListResult {
    /// All config entries as key-value pairs.
    pub entries: std::collections::BTreeMap<String, String>,
}

/// Result of the `agent` command.
#[derive(Debug, Serialize, JsonSchema)]
pub struct AgentRunResult {
    /// Whether the agent execution succeeded.
    pub success: bool,
    /// Process exit code.
    pub exit_code: Option<i32>,
    /// Whether the agent timed out.
    pub timed_out: bool,
    /// Agent output text.
    pub output: String,
}
