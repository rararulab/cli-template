# Agent Quickstart — From Template to Production CLI

> This guide teaches you (AI Agent) how to initialize a new project from cli-template
> and add features to it. Follow it top-to-bottom for initialization, or jump to a
> specific section when adding features to an existing project.

---

## Core Flow

```
 ┌──────────────────┐     ┌──────────────────┐     ┌──────────────────┐     ┌──────────┐
 │ 1. Initialize    │ ──▶ │ 2. Understand    │ ──▶ │ 3. Build         │ ──▶ │ 4. Verify │
 │    (placeholders) │     │    (architecture) │     │    (commands/mods)│     │    (check)│
 └──────────────────┘     └──────────────────┘     └──────────────────┘     └──────────┘
```

---

## Step 1: Project Initialization

### 1a. Placeholder Inventory

The template uses **3 placeholder variables** throughout the codebase:

| Placeholder | Format | Example | Where Used |
|-------------|--------|---------|------------|
| `{{project-name}}` | kebab-case | `my-awesome-cli` | Cargo.toml, CLI name, paths, npm, CI, docs |
| `{{crate_name}}` | snake_case | `my_awesome_cli` | Rust `use` statements, test binary name |
| `{{github-org}}` | GitHub org/user | `myorg` | Repository URLs, npm scope |

> **Rule**: `{{crate_name}}` is always `{{project-name}}` with hyphens replaced by underscores.

### 1b. Replace All Placeholders

```bash
# Set your values
PROJECT_NAME="my-awesome-cli"
CRATE_NAME="${PROJECT_NAME//-/_}"    # my_awesome_cli
GITHUB_ORG="myorg"

# Replace in all files (macOS)
find . -type f \( -name '*.rs' -o -name '*.toml' -o -name '*.yml' \
  -o -name '*.yaml' -o -name '*.json' -o -name '*.js' -o -name '*.md' \
  -o -name '*.sh' -o -name 'justfile' -o -name '.gitignore' \) \
  -not -path './.git/*' -not -path './target/*' \
  -exec sed -i '' \
    "s/{{project-name}}/${PROJECT_NAME}/g; \
     s/{{crate_name}}/${CRATE_NAME}/g; \
     s/{{github-org}}/${GITHUB_ORG}/g" {} +

# On Linux, drop the '' after -i:
#  -exec sed -i "s/..." {} +
```

### 1c. Post-Replacement Checklist

After replacing placeholders, customize these files:

| File | What to Change |
|------|----------------|
| `Cargo.toml` | `description`, `repository` URL, verify `license` |
| `CLAUDE.md` | Replace the `TODO:` line in Project Identity with your project description |
| `README.md` | Rewrite the project description and usage examples |
| `src/cli/mod.rs` | Update the `/// Your CLI application` doc comment |
| `src/app_config.rs` | Replace `ExampleConfig` with your own config sections |
| `src/main.rs` | Replace the `Hello` command dispatch with your own commands |
| `cliff.toml` | Verify the `repo` field matches your GitHub repo name |

### 1d. Verify Initialization

```bash
cargo check                  # Must compile without errors
cargo test                   # Baseline tests pass
cargo run -- --help          # CLI shows your project name
cargo run -- hello world     # Example command works
```

### 1e. Clean Up Example Code

Once your first real command is in place, remove the scaffolding:

1. Delete the `Hello` variant from `Command` enum in `src/cli/mod.rs`
2. Delete its `match` arm in `src/main.rs`
3. Replace `ExampleConfig` in `src/app_config.rs` with your own config
4. Update `set_config_field`, `get_config_field`, `config_as_map` in `src/main.rs`

---

## Step 2: Architecture Overview

### Module Map

```
src/
├── main.rs          # Entry point: CLI parsing → command dispatch → JSON output
├── lib.rs           # Public module re-exports (add new modules here)
├── cli/
│   └── mod.rs       # Clap definitions: Cli struct, Command enum, subcommand enums
├── error.rs         # AppError enum (snafu) — add variants for new error sources
├── app_config.rs    # TOML config: AppConfig struct + load()/save()
├── paths.rs         # ~/.{project-name}/ data directory resolution
├── http.rs          # Shared reqwest clients (client() + download_client())
└── agent/           # AI agent CLI integration (usually don't need to modify)
    ├── mod.rs       # Re-exports
    ├── backend.rs   # Backend presets (claude, gemini, codex, etc.)
    ├── config.rs    # AgentConfig struct
    └── executor.rs  # Process spawning + streaming
```

### Data Flow

```
User input
  │
  ▼
Cli::parse()              ← src/cli/mod.rs (clap derive)
  │
  ▼
match cli.command          ← src/main.rs (dispatch)
  │
  ├─▶ Your module logic    ← src/yourmodule/mod.rs
  │     ├── reads config   ← app_config::load()
  │     ├── makes HTTP     ← http::client()
  │     └── returns data
  │
  ▼
JSON output to stdout      ← serde_json::json!({"ok": true, ...})
Logs/errors to stderr      ← eprintln!() / tracing
```

### Key Conventions

- **stdout** = machine-readable JSON (for piping, scripting)
- **stderr** = human-readable messages (logs, progress, errors)
- **Config** = `~/.{project-name}/config.toml`, loaded once via `OnceLock`
- **Errors** = `snafu` enums with `.context(XxxSnafu)?` propagation
- **Singletons** = `OnceLock` for config, HTTP clients, data directory

---

## Step 3: Adding a Subcommand

### Decision Tree

```
What does your command need?
  │
  ├─ Nothing special (pure logic, string processing, file I/O)
  │   → Pattern A: Simple Command
  │
  ├─ Read/write config values
  │   → Pattern B: Config-Dependent Command
  │
  └─ Make HTTP API calls
      → Pattern C: HTTP Command
```

### Pattern A: Simple Command

**Step 1** — Add variant to `src/cli/mod.rs`:

```rust
#[derive(Subcommand)]
pub enum Command {
    // ... existing commands ...

    /// Count lines in a file
    Count {
        /// Path to the file
        path: std::path::PathBuf,

        /// Only count non-empty lines
        #[arg(long, default_value_t = false)]
        no_empty: bool,
    },
}
```

**Step 2** — Add dispatch in `src/main.rs`:

```rust
Command::Count { path, no_empty } => {
    let content = std::fs::read_to_string(&path).context(IoSnafu)?;
    let count = if no_empty {
        content.lines().filter(|l| !l.trim().is_empty()).count()
    } else {
        content.lines().count()
    };
    eprintln!("{count} lines in {}", path.display());
    println!(
        "{}",
        serde_json::json!({"ok": true, "action": "count", "lines": count})
    );
}
```

Done. Two files changed, zero new modules.

### Pattern B: Config-Dependent Command

Same as Pattern A, plus:

**Step 3** — Add config section in `src/app_config.rs`:

```rust
/// Download-related configuration.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct DownloadConfig {
    /// Default output directory.
    pub output_dir: String,
    /// Maximum concurrent downloads.
    pub max_concurrent: u32,
}

impl Default for DownloadConfig {
    fn default() -> Self {
        Self {
            output_dir: "./downloads".to_string(),
            max_concurrent: 4,
        }
    }
}
```

Add the field to `AppConfig`:

```rust
pub struct AppConfig {
    pub download: DownloadConfig,  // ← add this
    pub agent: AgentConfig,
}
```

**Step 4** — Wire up `set_config_field`/`get_config_field`/`config_as_map` in `src/main.rs`:

```rust
// In set_config_field:
"download.output_dir" => cfg.download.output_dir = value.to_string(),
"download.max_concurrent" => {
    cfg.download.max_concurrent = value.parse().map_err(|_| {
        error::AppError::Config {
            message: format!("invalid integer for {key}: {value}"),
        }
    })?;
}

// In get_config_field:
"download.output_dir" => Ok(Some(cfg.download.output_dir.clone())),
"download.max_concurrent" => Ok(Some(cfg.download.max_concurrent.to_string())),

// In config_as_map, add entries:
("download.output_dir".to_string(), cfg.download.output_dir.clone()),
("download.max_concurrent".to_string(), cfg.download.max_concurrent.to_string()),
```

### Pattern C: HTTP Command

Same as Pattern A, plus use `http::client()`:

```rust
Command::Fetch { url } => {
    let resp = http::client()
        .get(&url)
        .send()
        .await
        .context(HttpSnafu)?
        .json::<serde_json::Value>()
        .await
        .context(HttpSnafu)?;
    println!(
        "{}",
        serde_json::json!({"ok": true, "action": "fetch", "data": resp})
    );
}
```

> **Tip**: For complex HTTP logic (multiple endpoints, pagination, auth), create a dedicated
> module (see Step 4).

---

## Step 4: Adding a Module

When a feature grows beyond a few lines in `main.rs`, extract it into its own module.

### 4a. Create the Module Files

```
src/
└── download/
    ├── mod.rs       # Re-exports + module doc
    └── client.rs    # Core logic
```

**`src/download/mod.rs`**:
```rust
//! Download manager for fetching remote resources.

mod client;

pub use client::download_file;
```

**`src/download/client.rs`**:
```rust
//! HTTP download client with progress tracking.

use snafu::{ResultExt, Snafu};

#[derive(Debug, Snafu)]
#[snafu(visibility(pub))]
pub enum DownloadError {
    #[snafu(display("HTTP request failed: {source}"))]
    Http { source: reqwest::Error },

    #[snafu(display("failed to write file: {source}"))]
    WriteFile { source: std::io::Error },
}

pub type Result<T> = std::result::Result<T, DownloadError>;

/// Download a file from `url` to `dest`.
pub async fn download_file(url: &str, dest: &std::path::Path) -> Result<u64> {
    let resp = crate::http::client()
        .get(url)
        .send()
        .await
        .context(HttpSnafu)?;

    let bytes = resp.bytes().await.context(HttpSnafu)?;
    let len = bytes.len() as u64;

    std::fs::write(dest, &bytes).context(WriteFileSnafu)?;
    Ok(len)
}
```

### 4b. Wire It Up

**`src/lib.rs`** — add module declaration:
```rust
pub mod download;  // ← add this line
```

**`src/error.rs`** — add error variant if the module error should propagate to top-level:
```rust
#[snafu(display("download failed: {source}"))]
Download { source: crate::download::client::DownloadError },
```

**`src/main.rs`** — dispatch:
```rust
Command::Download { url, output } => {
    let dest = output.unwrap_or_else(|| std::path::PathBuf::from("."));
    let bytes = your_crate::download::download_file(&url, &dest)
        .await
        .context(DownloadSnafu)?;
    eprintln!("Downloaded {bytes} bytes to {}", dest.display());
    println!(
        "{}",
        serde_json::json!({"ok": true, "action": "download", "bytes": bytes})
    );
}
```

### 4c. Module Checklist

- [ ] `mod.rs` has `//!` module doc and only re-exports
- [ ] Error enum follows `{Module}Error` naming with `#[derive(Debug, Snafu)]`
- [ ] Module-level `pub type Result<T>` defined
- [ ] All `pub` items have `///` doc comments (English)
- [ ] Registered in `src/lib.rs`
- [ ] Error variant added to `src/error.rs` (if needed at top-level)
- [ ] Structs with 3+ fields use `#[derive(bon::Builder)]`

---

## Step 5: Common Patterns Reference

### Error Handling (snafu)

```rust
// Define (per module)
use snafu::{ResultExt, Snafu};

#[derive(Debug, Snafu)]
#[snafu(visibility(pub))]
pub enum FooError {
    #[snafu(display("failed to read {path}: {source}"))]
    ReadFile { path: String, source: std::io::Error },

    #[snafu(display("invalid format: {message}"))]
    InvalidFormat { message: String },
}

pub type Result<T> = std::result::Result<T, FooError>;

// Propagate
let data = std::fs::read_to_string(&path)
    .context(ReadFileSnafu { path: path.display().to_string() })?;

// Fail directly
return InvalidFormatSnafu { message: "expected JSON" }.fail();
```

### Builder Pattern (bon)

```rust
use bon::Builder;

/// Configuration for a download task.
#[derive(Debug, Builder)]
pub struct DownloadTask {
    /// Source URL.
    url: String,
    /// Destination path.
    dest: std::path::PathBuf,
    /// Maximum retries.
    #[builder(default = 3)]
    max_retries: u32,
}

// Usage
let task = DownloadTask::builder()
    .url("https://example.com/file.tar.gz".to_string())
    .dest("/tmp/file.tar.gz".into())
    .build();
```

### HTTP Requests

```rust
use crate::http;

// GET JSON
let data: MyResponse = http::client()
    .get("https://api.example.com/data")
    .send()
    .await
    .context(HttpSnafu)?
    .json()
    .await
    .context(HttpSnafu)?;

// POST with body
let resp = http::client()
    .post("https://api.example.com/submit")
    .json(&serde_json::json!({"key": "value"}))
    .send()
    .await
    .context(HttpSnafu)?;
```

### Config Access

```rust
use crate::app_config;

// Read (cached, returns &'static AppConfig)
let cfg = app_config::load();
let dir = &cfg.download.output_dir;

// Write
let mut cfg = app_config::load().clone();
cfg.download.max_concurrent = 8;
app_config::save(&cfg).context(IoSnafu)?;
```

### JSON Output Convention

```rust
// Success
println!("{}", serde_json::json!({
    "ok": true,
    "action": "my_action",
    "data": result
}));

// Error (handled automatically in main.rs, but for reference)
println!("{}", serde_json::json!({
    "ok": false,
    "error": "something went wrong"
}));
```

---

## Step 6: Common Pitfalls

| Mistake | Symptom | Fix |
|---------|---------|-----|
| Forgot to replace `{{crate_name}}` | `use {{crate_name}}::...` compile error | Run the sed command from Step 1b again |
| Replaced in `{{project-name}}` inside Jinja-style CI expressions like `${{ github.ref }}` | CI workflow syntax broken | Only replace `{{project-name}}`, `{{crate_name}}`, `{{github-org}}` — the sed command handles this correctly since GitHub Actions uses `${{` not bare `{{` |
| Added module but forgot `src/lib.rs` | `unresolved import` error | Add `pub mod yourmodule;` to `lib.rs` |
| Used `thiserror` instead of `snafu` | Clippy + review will flag it | Use `snafu` exclusively — see rust-style.md |
| Manual `fn new()` on struct with 3+ fields | Review will flag it | Use `#[derive(bon::Builder)]` |
| Hardcoded config defaults in Rust | Inconsistent with config file | Use the `Default` impl + `#[serde(default)]` pattern |
| Wrote comments in non-English | Review will flag it | All comments, docs, and string literals in English |
| Printed user-facing text to stdout | Breaks JSON piping | Use `eprintln!()` for human text, `println!()` for JSON only |
| Used `unwrap()` in non-test code | Clippy warning | Use `.expect("context why this is safe")` |
| Forgot `Closes #N` in commit body | Issue won't auto-close on merge | Follow commit-style.md: `type(scope): desc (#N)\n\nCloses #N` |
| Edited files in main checkout | Workflow violation | All work happens in `.worktrees/issue-{N}-{name}/` |
| Used wildcard import `use foo::*` | Clippy warning, unclear dependencies | Import specific items |
| Added `async_trait` without `Send + Sync` | Won't compile in multi-threaded tokio | Add `Send + Sync` bounds on async trait definitions |

---

## Step 7: Verification Checklists

### After Initialization

```bash
cargo check                    # Compiles without errors
cargo test                     # All tests pass
cargo clippy --all-targets --all-features -- -D warnings  # No warnings
cargo run -- --help            # Shows YOUR project name, not {{project-name}}
grep -r '{{project-name}}\|{{crate_name}}\|{{github-org}}' \
  --include='*.rs' --include='*.toml' --include='*.json' \
  --include='*.js' --include='*.yml' --include='*.md' \
  --exclude-dir=target --exclude-dir=.git  # Should return ZERO matches
```

### After Adding a Feature

```bash
cargo check                    # Compiles
cargo test                     # Tests pass (add tests for new code!)
cargo clippy --all-targets --all-features -- -D warnings  # Clean
cargo run -- your-new-command --help  # Command shows up and works
```

Or use the project's `justfile`:

```bash
just pre-commit                # Runs fmt-check + clippy + doc + test in one shot
```
