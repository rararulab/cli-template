# {{project-name}}

Opinionated Rust CLI template with batteries included.

## What's Included

- **CLI framework**: [clap](https://docs.rs/clap) with derive macros and subcommands
- **Error handling**: [snafu](https://docs.rs/snafu) with per-module `Result` types
- **Async runtime**: [tokio](https://docs.rs/tokio) with full features
- **Config system**: TOML-based with lazy `OnceLock` initialization
- **HTTP client**: Shared [reqwest](https://docs.rs/reqwest) clients (general + download)
- **Path management**: Centralized `~/.{{project-name}}` data directory
- **Logging**: [tracing](https://docs.rs/tracing) with env-filter
- **Builder pattern**: [bon](https://docs.rs/bon) for struct construction

## Tooling

- **Formatting**: `rustfmt` (nightly, opinionated config)
- **Linting**: `clippy` (pedantic + nursery) + `cargo-deny` (advisories, licenses, bans)
- **Testing**: `cargo-nextest`
- **Changelog**: `git-cliff` with conventional commits
- **Release**: `release-plz` for automated version bumping
- **Pre-commit**: `prek` hooks for format, lint, doc, and commit message validation
- **CI/CD**: GitHub Actions (lint → rust → release PR)

## Installation

### Via npx (recommended, no Rust required)

```bash
npx @<your-org>/<your-project> --help
```

### Via cargo

```bash
cargo install --path .
```

### From template

```bash
cargo generate rararulab/cli-template
```

You'll be prompted for project name and GitHub org. Then:

1. `cd <your-project>`
2. Update `CLAUDE.md` with your project description
3. Run `just setup-hooks` to install pre-commit hooks
4. Start building!

## Development

```bash
just fmt          # Format code
just clippy       # Run clippy
just test         # Run tests
just lint         # Full lint suite (clippy + doc + deny)
just pre-commit   # All pre-commit checks
just build        # Build debug binary
```

## Agent Backend

Invoke local AI agent CLIs without implementing LLM API integration. The agent module spawns CLI tools as child processes with streaming output and inactivity timeout.

### Usage

```bash
# Use default backend (claude)
{{project-name}} agent "explain this codebase"

# Override backend
{{project-name}} agent --backend codex "refactor main.rs"
{{project-name}} agent --backend gemini "summarize README"
```

### Supported Backends

| Backend | CLI Tool | Notes |
|---------|----------|-------|
| `claude` (default) | `claude` | Anthropic Claude Code |
| `kiro` | `kiro` | AWS Kiro |
| `gemini` | `gemini` | Google Gemini CLI |
| `codex` | `codex` | OpenAI Codex CLI |
| `amp` | `amp` | Sourcegraph Amp |
| `copilot` | `gh copilot` | GitHub Copilot |
| `opencode` | `opencode` | OpenCode |
| `pi` | `pi` | Inflection Pi |
| `roo` | `roo` | Roo Code |
| `custom` | (configurable) | Bring your own CLI |

### Configuration

Config file at `~/.{{project-name}}/config.toml`:

```toml
[agent]
backend = "claude"          # Backend name or "custom"
# command = "/path/to/cli"  # Override binary path
# args = ["--flag"]         # Extra CLI arguments
# prompt_mode = "arg"       # "arg" (default) or "stdin"
# idle_timeout_secs = 30    # Kill after N seconds of no output (0 = disable)
```

Override via CLI:

```bash
{{project-name}} config set agent.backend gemini
{{project-name}} config set agent.idle_timeout_secs 60
```

## Agent-Friendly CLI Design

This template follows [rararulab agent-friendly CLI standards](https://github.com/rararulab/.github/blob/main/docs/agent-friendly-cli.md):

- **JSON stdout, logs stderr** — all commands output structured JSON on stdout
- **Fail fast with suggestions** — errors include `suggestion` field for self-correction
- **Non-interactive** — every parameter passable via flags
- **Example-driven help** — each subcommand shows runnable examples in `--help`

## Project Structure

```
src/
├── main.rs         # Entry point, command dispatch
├── lib.rs          # Public module exports
├── cli/
│   └── mod.rs      # Clap CLI definitions
├── agent/
│   ├── mod.rs      # Re-exports
│   ├── backend.rs  # Backend presets and command building
│   ├── config.rs   # TOML [agent] config
│   └── executor.rs # Process spawning and streaming
├── error.rs        # Snafu error types
├── app_config.rs   # TOML config with OnceLock
├── paths.rs        # Centralized data directory paths
└── http.rs         # Shared reqwest HTTP clients

npm/                # npx install package (optionalDependencies pattern)
├── package.json    # Main package with platform optionalDependencies
└── bin/cli.js      # Platform binary resolver + launcher

docs/guides/        # Development conventions
├── workflow.md     # Issue → worktree → PR → merge
├── commit-style.md # Conventional commits
├── rust-style.md   # Snafu, bon, functional style
├── code-comments.md
└── anti-patterns.md
```
