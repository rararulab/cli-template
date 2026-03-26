# rara-cli-template Setup Command & Release — Implementation Plan

> **For Claude:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task.

**Goal:** Transform cli-template from a cargo-generate template repo into an installable CLI tool (`rara-cli-template`) with a `setup` command that scaffolds new projects, distributed via Homebrew and shell installer.

**Architecture:** The CLI binary embeds all template files at compile time via `include_dir`. The `setup` command collects parameters (project-name, github-org) via CLI flags or interactive stdin prompts, performs string replacement on embedded templates, writes them to disk, then runs post-setup steps (git init, cargo check, agent prompt output). Release uses cargo-dist for binary builds + Homebrew formula generation.

**Tech Stack:** Rust 2024, clap 4, include_dir, snafu, tokio (for cargo check subprocess), cargo-dist for release

---

## Overview: What Changes

The project currently serves dual purpose: it's both the template source AND uses template placeholders (`{{project-name}}`) in its own code. We need to split this into:

1. **CLI source** (`src/`, `Cargo.toml`, `tests/`) — the `rara-cli-template` binary itself, with real Rust code (no placeholders)
2. **Template files** (`template/`) — the current template content (with placeholders), embedded into the binary at compile time

### File Reorganization

**Stay at root (CLI tool itself):**
- `Cargo.toml` — rewritten for `rara-cli-template` package
- `Cargo.lock` — regenerated
- `src/` — new CLI source code (setup command)
- `tests/` — new integration tests
- `justfile` — for the CLI tool's own development
- `CLAUDE.md` — updated for the CLI tool
- `.github/workflows/` — CI + release workflows
- `cliff.toml`, `deny.toml`, `release-plz.toml`, `rustfmt.toml`, `rust-toolchain.toml` — kept for CLI tool
- `.pre-commit-config.yaml` — kept

**Move to `template/`:**
- Current `Cargo.toml` → `template/Cargo.toml`
- Current `src/` → `template/src/`
- Current `tests/` → `template/tests/`
- Current `justfile` → `template/justfile`
- Current `CLAUDE.md` → `template/CLAUDE.md`
- Current `README.md` → `template/README.md`
- `docs/guides/` → `template/docs/guides/`
- `npm/` → `template/npm/`
- `scripts/` → `template/scripts/`
- `web/` → `template/web/`
- `vendor/` → `template/vendor/`
- `cargo-generate.toml` → `template/cargo-generate.toml` (kept for users who still want cargo-generate)
- `template/cliff.toml`, `template/deny.toml`, `template/release-plz.toml`, `template/rustfmt.toml`, `template/rust-toolchain.toml`
- `template/.pre-commit-config.yaml`
- `template/.github/` (template CI workflows)
- `template/LICENSE`

---

### Task 1: Create `template/` directory and move template files

**Files:**
- Create: `template/` directory structure
- Move: all current template files into `template/`

**Step 1: Create template directory and move files**

```bash
cd /Users/ryan/code/rararulab/cli-template

# Create template directory
mkdir -p template

# Move template source files (these contain {{placeholders}})
mv src template/src
mv tests template/tests
mv npm template/npm
mv scripts template/scripts
mv web template/web
mv vendor template/vendor
mv docs/guides template/docs/guides  # keep docs/plans/ at root

# Move template config files
mv cargo-generate.toml template/
mv justfile template/
mv CLAUDE.md template/
mv README.md template/
mv LICENSE template/

# Copy config files that both the CLI tool and template need
# (CLI tool needs its own versions, template keeps originals)
cp cliff.toml template/
cp deny.toml template/
cp release-plz.toml template/
cp rustfmt.toml template/
cp rust-toolchain.toml template/
cp .pre-commit-config.yaml template/

# Move template's CI workflows
mv .github/workflows template/.github-workflows-tmp
mkdir -p .github/workflows
mv template/.github-workflows-tmp template/.github/workflows
# Note: we'll create new CI workflows for the CLI tool in a later task

# Move Cargo.toml (the template version) and Cargo.lock
mv Cargo.toml template/Cargo.toml
rm Cargo.lock  # will regenerate for the CLI tool
```

**Step 2: Verify template directory structure**

```bash
ls template/
# Expected: Cargo.toml, CLAUDE.md, README.md, LICENSE, justfile, cargo-generate.toml,
#           cliff.toml, deny.toml, release-plz.toml, rustfmt.toml, rust-toolchain.toml,
#           .pre-commit-config.yaml, src/, tests/, npm/, scripts/, web/, vendor/, docs/
```

**Step 3: Commit**

```bash
git add -A
git commit -m "chore: move template files to template/ directory

Preparation for rara-cli-template binary that embeds template files.
All {{placeholder}} files now live under template/."
```

---

### Task 2: Create new `Cargo.toml` for the CLI tool

**Files:**
- Create: `Cargo.toml`

**Step 1: Write the new Cargo.toml**

```toml
[package]
name = "rara-cli-template"
version = "0.1.0"
edition = "2024"
description = "Scaffold new Rust CLI projects from the rara-cli-template"
license = "MIT"
repository = "https://github.com/rararulab/cli-template"

[[bin]]
name = "rara-cli-template"
path = "src/main.rs"

[dependencies]
clap = { version = "4", features = ["derive"] }
include_dir = "0.7"
snafu = "0.9"
tokio = { version = "1", features = ["rt-multi-thread", "macros", "process"] }

[dev-dependencies]
assert_cmd = "2"
predicates = "3"
tempfile = "3"

[lints.rust]
unsafe_code = "deny"

[lints.clippy]
all = { level = "warn", priority = -1 }
pedantic = { level = "warn", priority = -1 }
nursery = { level = "warn", priority = -1 }
module_name_repetitions = "allow"
missing_errors_doc = "allow"
missing_panics_doc = "allow"
must_use_candidate = "allow"
redundant_pub_crate = "allow"
```

**Step 2: Verify**

This won't compile yet (no src/main.rs). That's fine — next task creates the source files.

**Step 3: Commit**

```bash
git add Cargo.toml
git commit -m "chore: add Cargo.toml for rara-cli-template CLI tool"
```

---

### Task 3: Create `src/error.rs` — error types

**Files:**
- Create: `src/error.rs`

**Step 1: Write error types**

```rust
//! Application-level error types.

use snafu::Snafu;

#[derive(Debug, Snafu)]
#[snafu(visibility(pub))]
pub enum AppError {
    #[snafu(display("IO error: {source}"))]
    Io { source: std::io::Error },

    #[snafu(display("{message}"))]
    Validation { message: String },

    #[snafu(display("setup failed: {message}"))]
    Setup { message: String },

    #[snafu(display("post-setup command failed: {message}"))]
    PostSetup { message: String },
}

pub type Result<T> = std::result::Result<T, AppError>;
```

**Step 2: Commit**

```bash
git add src/error.rs
git commit -m "feat(error): add error types for rara-cli-template"
```

---

### Task 4: Create `src/cli.rs` — clap definitions

**Files:**
- Create: `src/cli.rs`

**Step 1: Write CLI definitions**

```rust
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
```

**Step 2: Commit**

```bash
git add src/cli.rs
git commit -m "feat(cli): add clap definitions for setup command"
```

---

### Task 5: Create `src/template.rs` — template embedding and rendering

**Files:**
- Create: `src/template.rs`

**Step 1: Write template engine**

```rust
//! Template embedding and placeholder replacement.

use std::path::Path;

use include_dir::{include_dir, Dir};
use snafu::ResultExt;

use crate::error::{self, IoSnafu};

static TEMPLATE: Dir = include_dir!("$CARGO_MANIFEST_DIR/template");

/// Text file extensions that should have placeholders replaced.
const TEXT_EXTENSIONS: &[&str] = &[
    "rs", "toml", "yaml", "yml", "md", "json", "js", "sh", "toml", "lock",
];

/// Files/directories to skip when rendering.
const SKIP_PATTERNS: &[&str] = &["target", ".git", ".worktrees", "cargo-generate.toml"];

/// Render all template files into `output_dir`, replacing placeholders.
pub fn render(
    output_dir: &Path,
    project_name: &str,
    crate_name: &str,
    github_org: &str,
) -> error::Result<()> {
    render_dir(&TEMPLATE, output_dir, project_name, crate_name, github_org)
}

fn render_dir(
    dir: &Dir,
    output_dir: &Path,
    project_name: &str,
    crate_name: &str,
    github_org: &str,
) -> error::Result<()> {
    for entry in dir.dirs() {
        let dir_name = entry.path().file_name().and_then(|n| n.to_str()).unwrap_or("");
        if SKIP_PATTERNS.contains(&dir_name) {
            continue;
        }
        let target = output_dir.join(entry.path());
        std::fs::create_dir_all(&target).context(IoSnafu)?;
        render_dir(entry, output_dir, project_name, crate_name, github_org)?;
    }

    for file in dir.files() {
        let rel_path = file.path();

        // Skip patterns
        if rel_path
            .components()
            .any(|c| SKIP_PATTERNS.contains(&c.as_os_str().to_str().unwrap_or("")))
        {
            continue;
        }

        let target_path = output_dir.join(rel_path);

        // Ensure parent directory exists
        if let Some(parent) = target_path.parent() {
            std::fs::create_dir_all(parent).context(IoSnafu)?;
        }

        let contents = file.contents();

        if is_text_file(rel_path) {
            // Replace placeholders in text files
            let text = String::from_utf8_lossy(contents);
            let replaced = replace_placeholders(&text, project_name, crate_name, github_org);
            std::fs::write(&target_path, replaced.as_bytes()).context(IoSnafu)?;
        } else {
            // Copy binary files as-is
            std::fs::write(&target_path, contents).context(IoSnafu)?;
        }

        // Restore executable permission for .sh files
        #[cfg(unix)]
        if rel_path.extension().and_then(|e| e.to_str()) == Some("sh") {
            use std::os::unix::fs::PermissionsExt;
            let perms = std::fs::Permissions::from_mode(0o755);
            std::fs::set_permissions(&target_path, perms).context(IoSnafu)?;
        }
    }

    Ok(())
}

fn is_text_file(path: &Path) -> bool {
    let ext = path.extension().and_then(|e| e.to_str()).unwrap_or("");
    if TEXT_EXTENSIONS.contains(&ext) {
        return true;
    }
    // Files without extensions that are text: justfile, .gitignore, LICENSE, Dockerfile
    let name = path.file_name().and_then(|n| n.to_str()).unwrap_or("");
    matches!(
        name,
        "justfile" | ".gitignore" | ".pre-commit-config.yaml" | "LICENSE" | "Dockerfile"
    )
}

fn replace_placeholders(text: &str, project_name: &str, crate_name: &str, github_org: &str) -> String {
    text.replace("{{project-name}}", project_name)
        .replace("{{crate_name}}", crate_name)
        .replace("{{github-org}}", github_org)
}
```

**Step 2: Commit**

```bash
git add src/template.rs
git commit -m "feat(template): add template embedding and placeholder replacement engine"
```

---

### Task 6: Create `src/post_setup.rs` — git init, cargo check, agent prompt

**Files:**
- Create: `src/post_setup.rs`

**Step 1: Write post-setup logic**

```rust
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
    eprintln!("4. Run `just pre-commit` to verify everything passes");
    eprintln!("---");
}
```

**Step 2: Commit**

```bash
git add src/post_setup.rs
git commit -m "feat(post-setup): add git init, cargo check, and agent prompt output"
```

---

### Task 7: Create `src/setup.rs` — main setup orchestration

**Files:**
- Create: `src/setup.rs`

**Step 1: Write setup orchestration**

```rust
//! Setup command: collect parameters, validate, render template, run post-setup.

use std::io::{self, BufRead, Write};
use std::path::PathBuf;

use snafu::ResultExt;

use crate::error::{self, IoSnafu, SetupSnafu, ValidationSnafu};

/// Collected setup parameters.
pub struct SetupParams {
    pub project_name: String,
    pub crate_name: String,
    pub github_org: String,
    pub output_dir: PathBuf,
}

/// Collect setup parameters from CLI args and interactive prompts.
pub fn collect_params(
    name: Option<String>,
    org: Option<String>,
    path: Option<PathBuf>,
) -> error::Result<SetupParams> {
    let stdin = io::stdin();
    let mut reader = stdin.lock();

    let project_name = match name {
        Some(n) => n,
        None => prompt(&mut reader, "Project name (kebab-case)")?,
    };

    validate_project_name(&project_name)?;

    let github_org = match org {
        Some(o) => o,
        None => prompt_with_default(&mut reader, "GitHub org/username", "rararulab")?,
    };

    let crate_name = project_name.replace('-', "_");

    let output_dir = match path {
        Some(p) => p.join(&project_name),
        None => PathBuf::from(&project_name),
    };

    if output_dir.exists() {
        return ValidationSnafu {
            message: format!("directory already exists: {}", output_dir.display()),
        }
        .fail();
    }

    Ok(SetupParams {
        project_name,
        crate_name,
        github_org,
        output_dir,
    })
}

/// Run the full setup: render template + post-setup steps.
pub async fn run(params: &SetupParams) -> error::Result<()> {
    eprintln!(
        "Creating project \"{}\" (org: {})...",
        params.project_name, params.github_org
    );

    std::fs::create_dir_all(&params.output_dir).context(IoSnafu)?;

    crate::template::render(
        &params.output_dir,
        &params.project_name,
        &params.crate_name,
        &params.github_org,
    )
    .map_err(|e| error::AppError::Setup {
        message: e.to_string(),
    })?;

    crate::post_setup::run(&params.output_dir, &params.project_name).await;

    Ok(())
}

fn validate_project_name(name: &str) -> error::Result<()> {
    if name.is_empty() {
        return ValidationSnafu {
            message: "project name cannot be empty".to_string(),
        }
        .fail();
    }

    // Must be kebab-case: lowercase letters, digits, hyphens
    if !name
        .chars()
        .all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '-')
    {
        return ValidationSnafu {
            message: format!(
                "project name must be kebab-case (lowercase letters, digits, hyphens): {name}"
            ),
        }
        .fail();
    }

    if name.starts_with('-') || name.ends_with('-') {
        return ValidationSnafu {
            message: "project name cannot start or end with a hyphen".to_string(),
        }
        .fail();
    }

    Ok(())
}

fn prompt(reader: &mut impl BufRead, label: &str) -> error::Result<String> {
    eprint!("{label}: ");
    io::stderr().flush().context(IoSnafu)?;
    let mut input = String::new();
    reader.read_line(&mut input).context(IoSnafu)?;
    let trimmed = input.trim().to_string();
    if trimmed.is_empty() {
        return ValidationSnafu {
            message: format!("{label} is required"),
        }
        .fail();
    }
    Ok(trimmed)
}

fn prompt_with_default(
    reader: &mut impl BufRead,
    label: &str,
    default: &str,
) -> error::Result<String> {
    eprint!("{label} [{default}]: ");
    io::stderr().flush().context(IoSnafu)?;
    let mut input = String::new();
    reader.read_line(&mut input).context(IoSnafu)?;
    let trimmed = input.trim();
    if trimmed.is_empty() {
        Ok(default.to_string())
    } else {
        Ok(trimmed.to_string())
    }
}
```

**Step 2: Commit**

```bash
git add src/setup.rs
git commit -m "feat(setup): add parameter collection, validation, and setup orchestration"
```

---

### Task 8: Create `src/main.rs` and `src/lib.rs` — entry point

**Files:**
- Create: `src/main.rs`
- Create: `src/lib.rs`

**Step 1: Write lib.rs**

```rust
pub mod cli;
pub mod error;
pub mod post_setup;
pub mod setup;
pub mod template;
```

**Step 2: Write main.rs**

```rust
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
```

**Step 3: Verify it compiles**

```bash
cargo check
```

**Step 4: Commit**

```bash
git add src/main.rs src/lib.rs
git commit -m "feat: add main entry point and lib module exports"
```

---

### Task 9: Write integration tests

**Files:**
- Create: `tests/setup_test.rs`

**Step 1: Write the tests**

```rust
use assert_cmd::Command;
use predicates::prelude::predicate;
use tempfile::TempDir;

fn cmd() -> Command {
    Command::cargo_bin("rara-cli-template").expect("binary should exist")
}

#[test]
fn setup_creates_project_with_all_flags() {
    let tmp = TempDir::new().expect("failed to create temp dir");
    let project_dir = tmp.path().join("my-test-cli");

    cmd()
        .args([
            "setup",
            "my-test-cli",
            "--org",
            "testorg",
            "--path",
            tmp.path().to_str().unwrap(),
        ])
        .assert()
        .success()
        .stderr(predicate::str::contains("Project my-test-cli created"));

    // Verify key files exist
    assert!(project_dir.join("Cargo.toml").exists());
    assert!(project_dir.join("src/main.rs").exists());
    assert!(project_dir.join("src/cli/mod.rs").exists());
    assert!(project_dir.join("CLAUDE.md").exists());
    assert!(project_dir.join("justfile").exists());

    // Verify placeholders were replaced
    let cargo_toml = std::fs::read_to_string(project_dir.join("Cargo.toml")).unwrap();
    assert!(cargo_toml.contains("my-test-cli"));
    assert!(!cargo_toml.contains("{{project-name}}"));

    let main_rs = std::fs::read_to_string(project_dir.join("src/main.rs")).unwrap();
    assert!(main_rs.contains("my_test_cli"));
    assert!(!main_rs.contains("{{crate_name}}"));

    // Verify git was initialized
    assert!(project_dir.join(".git").exists());
}

#[test]
fn setup_rejects_invalid_project_name() {
    let tmp = TempDir::new().expect("failed to create temp dir");

    cmd()
        .args([
            "setup",
            "My_Bad_Name",
            "--org",
            "testorg",
            "--path",
            tmp.path().to_str().unwrap(),
        ])
        .assert()
        .failure()
        .stderr(predicate::str::contains("kebab-case"));
}

#[test]
fn setup_rejects_existing_directory() {
    let tmp = TempDir::new().expect("failed to create temp dir");
    // Create the target directory first
    std::fs::create_dir(tmp.path().join("already-exists")).unwrap();

    cmd()
        .args([
            "setup",
            "already-exists",
            "--org",
            "testorg",
            "--path",
            tmp.path().to_str().unwrap(),
        ])
        .assert()
        .failure()
        .stderr(predicate::str::contains("already exists"));
}

#[test]
fn setup_replaces_github_org() {
    let tmp = TempDir::new().expect("failed to create temp dir");

    cmd()
        .args([
            "setup",
            "org-test",
            "--org",
            "mycompany",
            "--path",
            tmp.path().to_str().unwrap(),
        ])
        .assert()
        .success();

    let project_dir = tmp.path().join("org-test");
    let cargo_toml = std::fs::read_to_string(project_dir.join("Cargo.toml")).unwrap();
    assert!(cargo_toml.contains("mycompany"));
    assert!(!cargo_toml.contains("{{github-org}}"));
}
```

**Step 2: Run tests**

```bash
cargo test
```

Expected: all 4 tests pass.

**Step 3: Commit**

```bash
git add tests/setup_test.rs
git commit -m "test(setup): add integration tests for setup command"
```

---

### Task 10: Update root-level config files

**Files:**
- Modify: `cliff.toml` — update repo to `rararulab/cli-template`
- Modify: `release-plz.toml` — verify settings
- Create: `README.md` — new README for the CLI tool
- Modify: `CLAUDE.md` — new CLAUDE.md for the CLI tool

**Step 1: Write new README.md**

```markdown
# rara-cli-template

Scaffold new Rust CLI projects with batteries included.

## Install

```bash
# Homebrew
brew install rararulab/homebrew-tap/rara-cli-template

# Shell
curl --proto '=https' --tlsv1.2 -LsSf https://github.com/rararulab/cli-template/releases/latest/download/rara-cli-template-installer.sh | sh
```

## Usage

```bash
# Interactive setup
rara-cli-template setup

# With flags
rara-cli-template setup my-cool-cli --org myorg
```

## What's Included

Projects created from this template include:

- **Rust 2024** with clap 4, snafu, tokio, reqwest, serde
- **AI Agent integration** — 10+ backend support (Claude, Gemini, Codex, etc.)
- **NPX distribution** — users can run your CLI without installing Rust
- **CI/CD** — GitHub Actions for lint, test, release
- **Changelog** — git-cliff with conventional commits
- **Pre-commit hooks** — via prek

## Development

```bash
just fmt          # Format code
just clippy       # Lint
just test         # Run tests
just pre-commit   # All checks
```
```

**Step 2: Write new CLAUDE.md**

```markdown
# CLAUDE.md — rara-cli-template Development Guide

## Communication
- 用中文与用户交流

## Project Identity

rara-cli-template is a CLI tool that scaffolds new Rust CLI projects. It embeds template files at compile time via `include_dir` and generates new projects with placeholder replacement, git initialization, and agent-ready prompts.

## Architecture

- `src/main.rs` — Entry point, clap dispatch
- `src/cli.rs` — Clap definitions (setup subcommand)
- `src/setup.rs` — Parameter collection, validation, orchestration
- `src/template.rs` — include_dir embedding + placeholder replacement
- `src/post_setup.rs` — git init, cargo check, agent prompt output
- `src/error.rs` — snafu error types
- `template/` — Template source files (embedded into binary)

## Development Workflow

@docs/guides/workflow.md
@docs/guides/commit-style.md

## Code Quality

@docs/guides/rust-style.md
@docs/guides/code-comments.md

## Guardrails

@docs/guides/anti-patterns.md
```

Note: The docs/guides/ referenced above are in the template directory. For the CLI tool's own development, we need minimal copies or the CLI tool references its own guides. Actually — keep it simple: the CLI tool itself follows the same Rust style conventions. We can symlink or just reference the same guide content. For the plan, just write the CLAUDE.md as above. The guides are well-known conventions that don't need to be files — the CLAUDE.md contains enough inline guidance.

**Step 3: Update cliff.toml repo field**

Verify `cliff.toml` already has the correct repository URL (`rararulab/cli-template`). If it references `{{project-name}}`, update it.

**Step 4: Commit**

```bash
git add README.md CLAUDE.md cliff.toml
git commit -m "docs: add README and CLAUDE.md for rara-cli-template CLI tool"
```

---

### Task 11: Add cargo-dist configuration for release

**Files:**
- Modify: `Cargo.toml` — add `[workspace.metadata.dist]` section

**Step 1: Add cargo-dist config to Cargo.toml**

Append to `Cargo.toml`:

```toml
# Config for cargo-dist — generates release binaries + Homebrew formula
[workspace.metadata.dist]
cargo-dist-version = "0.28.0"
ci = "github"
installers = ["shell", "homebrew"]
targets = ["aarch64-apple-darwin", "x86_64-unknown-linux-gnu"]
tap = "rararulab/homebrew-tap"
publish-jobs = ["homebrew"]
install-path = "CARGO_HOME"
```

**Step 2: Generate release workflow**

```bash
# This generates .github/workflows/release.yml
cargo dist init
# Review the generated workflow, accept defaults
```

Note: if `cargo dist` is not installed, install it first:
```bash
cargo install cargo-dist
```

**Step 3: Commit**

```bash
git add Cargo.toml .github/workflows/release.yml
git commit -m "ci(release): add cargo-dist configuration for Homebrew and shell installer"
```

---

### Task 12: Update CI workflows

**Files:**
- Create: `.github/workflows/ci.yml` — basic CI for the CLI tool
- Create: `.github/workflows/release-pr.yml` — release PR automation

**Step 1: Write ci.yml**

```yaml
name: CI

on:
  push:
    branches: [main]
  pull_request:
    branches: [main]

concurrency:
  group: ci-${{ github.ref }}
  cancel-in-progress: true

jobs:
  check:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - uses: dtolnay/rust-toolchain@stable
      - uses: dtolnay/rust-toolchain@nightly
        with:
          components: rustfmt
      - uses: Swatinem/rust-cache@v2

      - name: Format check
        run: cargo +nightly fmt --all --check

      - name: Clippy
        run: cargo clippy --all-targets --all-features -- -D warnings

      - name: Test
        run: cargo test

      - name: Doc check
        run: cargo doc --no-deps --document-private-items
        env:
          RUSTDOCFLAGS: "-D warnings"
```

**Step 2: Write release-pr.yml**

```yaml
name: Release PR

on:
  push:
    branches: [main]

permissions:
  pull-requests: write
  contents: write

jobs:
  release-plz:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
        with:
          fetch-depth: 0
      - uses: dtolnay/rust-toolchain@stable
      - uses: MarcoIeni/release-plz-action@v0.5
        env:
          GITHUB_TOKEN: ${{ secrets.GITHUB_TOKEN }}
          CARGO_REGISTRY_TOKEN: ${{ secrets.CARGO_REGISTRY_TOKEN }}
```

**Step 3: Commit**

```bash
git add .github/workflows/ci.yml .github/workflows/release-pr.yml
git commit -m "ci: add CI and release-pr workflows for rara-cli-template"
```

---

### Task 13: Final verification and cleanup

**Step 1: Full build and test**

```bash
cargo check
cargo test
cargo clippy --all-targets --all-features -- -D warnings
cargo build
```

**Step 2: Manual smoke test**

```bash
# Build the binary
cargo build

# Run setup in a temp directory
cd /tmp
/path/to/target/debug/rara-cli-template setup smoke-test --org testorg

# Verify the generated project
cd smoke-test
cargo check
cargo test
cargo run -- --help
cargo run -- hello

# Cleanup
cd /tmp && rm -rf smoke-test
```

**Step 3: Verify no template placeholders leak into CLI source**

```bash
grep -r '{{project-name}}\|{{crate_name}}\|{{github-org}}' src/ tests/ Cargo.toml
# Should return ZERO matches (only template/ should contain these)
```

**Step 4: Final commit if any fixes needed**

```bash
git add -A
git commit -m "chore: final cleanup and verification"
```

---

## Task Dependency Graph

```
Task 1 (move files)
  └── Task 2 (new Cargo.toml)
       └── Task 3 (error.rs)
       └── Task 4 (cli.rs)
       └── Task 5 (template.rs)
       └── Task 6 (post_setup.rs)
            └── Task 7 (setup.rs)
                 └── Task 8 (main.rs + lib.rs) → compiles!
                      └── Task 9 (tests)
                      └── Task 10 (README, CLAUDE.md)
                      └── Task 11 (cargo-dist)
                      └── Task 12 (CI workflows)
                           └── Task 13 (final verification)
```

Tasks 3-6 can be done in parallel. Tasks 9-12 can be done in parallel after Task 8.
