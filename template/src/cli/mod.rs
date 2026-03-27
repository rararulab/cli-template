//! CLI command definitions and subcommand modules.

use agent_describe::AgentDescribe;
use clap::{Parser, Subcommand};

/// Your CLI application — update this doc comment.
#[derive(Parser)]
#[command(name = "{{project-name}}", version)]
pub struct Cli {
    /// Output agent-describe schema (for AI agent discovery)
    #[arg(long, hide = true)]
    pub agent_describe: bool,

    #[command(subcommand)]
    pub command: Option<Command>,
}

/// Available subcommands.
///
/// Commands follow the agent-friendly CLI pattern:
/// - JSON on stdout, human text on stderr
/// - Every error includes a `suggestion` field
/// - All parameters passable via flags (non-interactive)
#[derive(Subcommand, AgentDescribe)]
#[agent(cli = Cli)]
pub enum Command {
    /// Say hello (example command — replace with your own)
    #[command(after_help = "\
EXAMPLES:
    {{project-name}} hello
    {{project-name}} hello Alice")]
    #[agent(output = crate::response::HelloResult)]
    Hello {
        /// Name to greet
        #[arg(default_value = "world")]
        name: String,
    },

    /// Manage config values
    #[command(after_help = "\
EXAMPLES:
    {{project-name}} config list
    {{project-name}} config get agent.backend
    {{project-name}} config set agent.backend gemini")]
    Config {
        #[command(subcommand)]
        action: ConfigAction,
    },

    /// Run a prompt through the configured agent backend
    #[command(after_help = "\
EXAMPLES:
    {{project-name}} agent \"explain this codebase\"
    {{project-name}} agent --backend codex \"refactor main.rs\"")]
    #[agent(output = crate::response::AgentRunResult)]
    Agent {
        /// The prompt to send to the agent
        prompt: String,
        /// Override the backend (e.g., "claude", "codex")
        #[arg(long)]
        backend: Option<String>,
    },
}

/// Config management subcommands.
#[derive(Subcommand)]
pub enum ConfigAction {
    /// Set a config value
    #[command(after_help = "\
EXAMPLES:
    {{project-name}} config set example.setting myvalue
    {{project-name}} config set agent.backend gemini
    {{project-name}} config set agent.idle_timeout_secs 60")]
    Set {
        /// Config key (e.g. example.setting)
        key:   String,
        /// Config value
        value: String,
    },
    /// Get a config value
    #[command(after_help = "\
EXAMPLES:
    {{project-name}} config get example.setting
    {{project-name}} config get agent.backend")]
    Get {
        /// Config key to look up
        key: String,
    },
    /// List all config values
    #[command(after_help = "\
EXAMPLES:
    {{project-name}} config list")]
    List,
}
