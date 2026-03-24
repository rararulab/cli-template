---
name: setup
description: Use when setting up, installing, or initializing a project created from cli-template. Keywords: install, setup, getting started, new project, cargo generate, npx.
---

# Project Setup Guide

## Creating a New Project

```bash
cargo generate rararulab/cli-template
cd <your-project-name>
```

After generation, the `{{project-name}}` and `{{crate_name}}` placeholders are replaced with your project name.

## Installing Pre-built Binary

### Via npx (no Rust required)
```bash
npx @<org>/<project-name> --help
```

### Via cargo
```bash
cargo install --path .
```

## Post-Setup Checklist

1. Update `CLAUDE.md` — fill in the "Project Identity" section
2. Update `Cargo.toml` — fill in `description`
3. Run `just setup-hooks` to install pre-commit hooks
4. Run `just pre-commit` to verify everything works
5. Push to GitHub and verify CI passes

## Development Commands

```bash
just fmt          # Format code
just clippy       # Run clippy
just test         # Run tests
just lint         # Full lint suite
just pre-commit   # All pre-commit checks
just build        # Build debug binary
```

## Project Conventions

- **Errors**: Use `snafu` — never `thiserror` or manual impls
- **Builders**: Use `bon::Builder` for structs with 3+ fields
- **Workflow**: All changes go through issue → worktree → PR → merge
- **Commits**: Conventional commits format: `type(scope): description (#N)`
- **Comments**: All code comments in English

## Key Directories

- `src/` — Rust source code
- `docs/guides/` — Development conventions
- `.claude/skills/dev/` — `/dev` autonomous development pipeline
- `web/` — GitHub Pages landing site
- `npm/` — npx install package (downloads pre-built binary)

## Configuration

Config file location: `~/.<project-name>/config.toml`

```toml
[agent]
backend = "claude"
```
