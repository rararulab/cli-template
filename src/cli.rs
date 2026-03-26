//! CLI command definitions.

use std::path::PathBuf;

use clap::{Parser, Subcommand};

/// Scaffold new Rust CLI projects from the rara-cli-template.
#[derive(Parser)]
#[command(name = "rara-cli-template", version)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Command,
}

/// Available subcommands.
#[derive(Subcommand)]
pub enum Command {
    /// Create a new project from the template
    #[command(after_help = "\
EXAMPLES:
    rara-cli-template setup
    rara-cli-template setup my-cool-cli
    rara-cli-template setup my-cool-cli --org myorg
    rara-cli-template setup my-cool-cli --org myorg --path ./projects")]
    Setup {
        /// Project name (kebab-case, e.g. my-cool-cli)
        name: Option<String>,

        /// GitHub organization or username
        #[arg(long, short)]
        org: Option<String>,

        /// Output directory (defaults to ./{project-name})
        #[arg(long, short)]
        path: Option<PathBuf>,
    },
}
