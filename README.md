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
