use clap::Parser;

use rara_cli_template::cli::{Cli, Command};

#[tokio::main]
async fn main() {
    let cli = Cli::parse();

    if let Err(e) = run(cli).await {
        eprintln!("Error: {e}");
        std::process::exit(1);
    }
}

async fn run(cli: Cli) -> rara_cli_template::error::Result<()> {
    match cli.command {
        Command::Setup { name, org, path } => {
            let params = rara_cli_template::setup::collect_params(name, org, path)?;
            rara_cli_template::setup::run(&params).await?;
        }
    }
    Ok(())
}
