# Project Structure

## Module Map

```
src/
├── main.rs          # Entry: CLI parse → dispatch → JSON output
├── lib.rs           # Module re-exports
├── cli/mod.rs       # Clap: Cli struct, Command enum
├── error.rs         # AppError (snafu)
├── app_config.rs    # TOML config: load()/save()
├── paths.rs         # Data directory (~/.project-name/)
├── http.rs          # Shared reqwest clients
└── agent/           # AI agent backend integration
    ├── mod.rs
    ├── backend.rs
    ├── config.rs
    └── executor.rs
```

## Data Flow

```
User input → Cli::parse() → match command → module logic → JSON stdout
                                                         → logs stderr
```

All commands follow the same pattern: parse arguments, execute logic, serialize results to JSON on stdout. Human-readable logs go to stderr via `tracing`.

## Module Reference

### `main.rs`
Entry point. Parses CLI args, dispatches to the matched command, and prints the result as JSON to stdout.

### `lib.rs`
Re-exports all modules so they can be used from integration tests and other crates.

### `cli/mod.rs`
Clap derive definitions. Contains the top-level `Cli` struct and the `Command` enum with all subcommands.

### `error.rs`
Top-level `AppError` type built with snafu. Each error variant maps to a specific failure mode with structured context.

### `app_config.rs`
TOML-based config with `load()` and `save()` functions. Uses `OnceLock` to cache the parsed config for the lifetime of the process.

### `paths.rs`
Resolves the data directory (`~/.project-name/`) and config file paths. All filesystem locations are derived from here.

### `http.rs`
Provides `client()` and `download_client()` singletons via `OnceLock`. Both return shared `reqwest::Client` instances with sensible defaults.

### `agent/`
AI backend integration layer. `backend.rs` defines provider presets, `config.rs` handles agent-specific settings, and `executor.rs` spawns and manages agent processes.

## Key Conventions

- **stdout = JSON only.** Every command outputs machine-readable JSON. Never `println!` free-form text to stdout.
- **stderr = human-readable logs.** Use `tracing::info!`, `tracing::warn!`, etc. for all human-facing output.
- **Config is cached via `OnceLock`.** Call `load()` freely — the file is read once and reused.
- **HTTP clients are singletons.** Use `http::client()` instead of constructing your own `reqwest::Client`.
- **Structs with 3+ fields use `bon::Builder`.** Derive `Builder` for any struct that would otherwise need a verbose constructor.
