# Getting Started

## Prerequisites

Install the Rust toolchain and `cargo-generate`:

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
cargo install cargo-generate
```

## Create a Project

```bash
cargo generate rararulab/cli-template
```

You'll be prompted for three values:

| Placeholder      | Format     | Example          | Used For                          |
|------------------|------------|------------------|-----------------------------------|
| `project-name`   | kebab-case | `my-awesome-cli` | Binary name, directory, npm pkg   |
| `crate_name`     | snake_case | *(auto-derived)* | Rust module name                  |
| `github-org`     | —          | `myorg`          | Repo URLs, CI badges              |

> `crate_name` is derived automatically from `project-name` — you rarely need to change it.

## First Run

```bash
cd my-awesome-cli
cargo check
cargo test
cargo run -- --help
cargo run -- hello world
```

The `hello` command is a working example wired end-to-end. Use it as a reference when adding your own commands.

## What to Customize

Once you've verified the template builds, update these files:

- **`CLAUDE.md`** — add a description of your project for AI-assisted development
- **`Cargo.toml`** — set `description`, `repository`, and `homepage`
- **`src/cli/mod.rs`** — replace the example `Hello` command with your own
- **`src/app_config.rs`** — replace `ExampleConfig` with your app's config fields
- **`README.md`** — rewrite for your project

## Clean Up Example Code

Once your first real command is in place, remove the scaffolding:

1. Delete the `Hello` variant from the `Command` enum in `src/cli/mod.rs`
2. Delete its match arm in `main.rs`
3. Replace `ExampleConfig` in `src/app_config.rs` with your own config struct

> Don't delete the example code until you have a real command working. It serves as a reference for the patterns used throughout the template.

## Verify

```bash
cargo check && cargo test && cargo clippy
```

All three should pass cleanly before your first commit.

## Next Steps

- [Project Structure](guide/project-structure.md) — understand the module layout and conventions
