//! Post-setup steps: git init, cargo check, and agent prompt output.

use std::path::Path;
use std::process::Stdio;

/// Run all post-setup steps in the generated project directory.
pub async fn run(project_dir: &Path, project_name: &str) {
    git_init(project_dir);
    cargo_check(project_dir).await;
    print_agent_prompt(project_name, project_dir);
}

fn git_init(project_dir: &Path) {
    eprint!("Initializing git repository... ");
    let init = std::process::Command::new("git")
        .args(["init", "--initial-branch=main"])
        .current_dir(project_dir)
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status();

    match init {
        Ok(s) if s.success() => {}
        _ => {
            eprintln!("warning: git init failed, skipping");
            return;
        }
    }

    let add = std::process::Command::new("git")
        .args(["add", "-A"])
        .current_dir(project_dir)
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status();

    if add.is_err() || !add.unwrap().success() {
        eprintln!("warning: git add failed, skipping initial commit");
        return;
    }

    let version = env!("CARGO_PKG_VERSION");
    let msg = format!("chore: init from rara-cli-template v{version}");
    let commit = std::process::Command::new("git")
        .args(["commit", "-m", &msg])
        .current_dir(project_dir)
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status();

    match commit {
        Ok(s) if s.success() => eprintln!("done"),
        _ => eprintln!("warning: initial commit failed"),
    }
}

async fn cargo_check(project_dir: &Path) {
    eprint!("Running cargo check... ");
    let result = tokio::process::Command::new("cargo")
        .arg("check")
        .current_dir(project_dir)
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status()
        .await;

    match result {
        Ok(s) if s.success() => eprintln!("ok"),
        Ok(_) => eprintln!("warning: cargo check failed — verify Rust toolchain is installed"),
        Err(_) => eprintln!("warning: cargo not found — skipping check"),
    }
}

fn print_agent_prompt(project_name: &str, project_dir: &Path) {
    let dir_display = project_dir.display();
    eprintln!();
    eprintln!("✅ Project {project_name} created at {dir_display}");
    eprintln!();
    eprintln!("To start developing with an AI agent, copy the prompt below:");
    eprintln!();
    eprintln!("---");
    eprintln!("I have a new Rust CLI project \"{project_name}\" initialized from rara-cli-template.");
    eprintln!("The project is at {dir_display} with git already initialized.");
    eprintln!();
    eprintln!("Read CLAUDE.md and docs/guides/agent-quickstart.md first, then:");
    eprintln!("1. Update CLAUDE.md with the project description");
    eprintln!("2. Replace the Hello example command with actual CLI commands");
    eprintln!("3. Customize ExampleConfig in src/app_config.rs");
    eprintln!("4. Test agent discovery: `{project_name} --agent-describe`");
    eprintln!("5. Add new commands: define in cli/mod.rs, add {{Name}}Result in response.rs");
    eprintln!("6. Run `just pre-commit` to verify everything passes");
    eprintln!("---");
}
