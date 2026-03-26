# cli-template

A batteries-included Rust CLI template with structured errors, config management, CI/CD pipelines, and automated releases.

## What You Get

- **Clap** argument parsing with derive macros
- **snafu** structured error handling
- **tokio** async runtime
- **tracing** observability
- **TOML** config system with CLI get/set commands
- **reqwest** HTTP client singletons
- **GitHub Actions** CI/CD (lint, test, release)
- **cargo-dist** cross-platform binary builds
- **npx** distribution — users run your CLI without Rust installed
- **Agent-friendly** output: JSON stdout, logs stderr

## Quick Start

```bash
cargo generate rararulab/cli-template
cd my-awesome-cli
cargo run -- --help
```

See [Getting Started](getting-started.md) for the full walkthrough.
