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
All changes — no matter how small — follow the issue → worktree → PR → merge flow.

## Code Quality
- `snafu` for error handling (never `thiserror`)
- `bon::Builder` for structs with 3+ fields
- Tokio async, early returns with `?`, functional style
- No wildcard imports, all `pub` items have doc comments (English)

## Guardrails
- No manual `impl Error` (use snafu)
- No hardcoded config defaults
- No work on main, all changes via worktree + PR
