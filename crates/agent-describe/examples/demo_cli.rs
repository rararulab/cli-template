use agent_describe::{AgentDescribe, AgentResponse};
use clap::{Parser, Subcommand, ValueEnum};
use schemars::JsonSchema;
use serde::Serialize;

#[derive(Parser)]
#[command(name = "demo", version = "0.1.0", about = "Demo agent-friendly CLI")]
struct Cli {
    /// Output agent-describe schema
    #[arg(long)]
    agent_describe: bool,

    #[command(subcommand)]
    command: Option<DemoCommand>,
}

#[derive(Subcommand, AgentDescribe)]
#[agent(cli = Cli)]
enum DemoCommand {
    /// Deploy application
    Deploy {
        /// Target environment
        #[arg(value_enum)]
        env: Environment,
        /// Dry run
        #[arg(long)]
        dry_run: bool,
    },

    /// Manage configuration
    Config {
        #[command(subcommand)]
        action: ConfigAction,
    },
}

#[derive(Subcommand)]
enum ConfigAction {
    /// Set a config value
    Set {
        /// Config key
        key: String,
        /// Config value
        value: String,
    },
    /// Get a config value
    Get {
        /// Config key
        key: String,
    },
}

#[derive(Clone, ValueEnum)]
enum Environment {
    Staging,
    Production,
}

/// Convention: Deploy -> DeployResult
#[derive(Serialize, JsonSchema)]
struct DeployResult {
    /// Deployment URL
    url: String,
    /// Time taken in seconds
    took_secs: f64,
}

fn main() {
    let cli = Cli::parse();

    if cli.agent_describe {
        let schema = DemoCommand::agent_schema();
        println!("{}", serde_json::to_string_pretty(&schema).unwrap());
        return;
    }

    match cli.command {
        Some(DemoCommand::Deploy { env: _, dry_run: _ }) => {
            AgentResponse::ok(DeployResult {
                url: "https://app.example.com".into(),
                took_secs: 3.2,
            })
            .print();
        }
        Some(DemoCommand::Config { .. }) => {
            AgentResponse::ok(serde_json::json!({"key": "example", "value": "test"})).print();
        }
        None => {
            eprintln!("No command specified. Try --help or --agent-describe");
            std::process::exit(1);
        }
    }
}
