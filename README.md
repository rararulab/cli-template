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

## Quick Start

1. Use this template to create a new repo
2. Find and replace `{{project-name}}` with your project name
3. Update `CLAUDE.md` with your project description
4. Run `just setup-hooks` to install pre-commit hooks
5. Start building!

## Development

```bash
just fmt          # Format code
just clippy       # Run clippy
just test         # Run tests
just lint         # Full lint suite (clippy + doc + deny)
just pre-commit   # All pre-commit checks
just build        # Build debug binary
```

## Claude Code Integration

Built-in `/dev` skill for autonomous development pipeline:

```
/dev <requirement>        # Full cycle: design → implement → review → ship
/dev --quick <requirement> # Skip design & review for trivial changes
```

Includes:
- `CLAUDE.md` with project conventions and code style guides
- `/dev` skill with subagent prompts for implementation, code review, and design review
- Development guides: workflow, commit style, Rust style, code comments, anti-patterns

## Project Structure

```
src/
├── main.rs         # Entry point, command dispatch
├── lib.rs          # Public module exports
├── cli/
│   └── mod.rs      # Clap CLI definitions
├── error.rs        # Snafu error types
├── app_config.rs   # TOML config with OnceLock
├── paths.rs        # Centralized data directory paths
└── http.rs         # Shared reqwest HTTP clients

.claude/
└── skills/
    └── dev/        # /dev autonomous development pipeline
        ├── SKILL.md
        └── references/
            ├── templates.md
            └── subagent-prompts.md

docs/guides/        # Development conventions
├── workflow.md     # Issue → worktree → PR → merge
├── commit-style.md # Conventional commits
├── rust-style.md   # Snafu, bon, functional style
├── code-comments.md
└── anti-patterns.md
```
