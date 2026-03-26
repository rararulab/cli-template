# Error Handling

## Why snafu

[snafu](https://docs.rs/snafu) gives you contextual, structured errors without manual `impl` boilerplate. Each error variant carries the data needed to produce a useful message, and context selectors make propagation a one-liner.

## Defining Errors

```rust
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
```

Key points:

- `#[snafu(visibility(pub))]` makes context selectors (`ReadFileSnafu`, etc.) public so callers can use them.
- Variants with a `source` field wrap an underlying error. Variants without `source` are leaf errors.
- Always define a module-level `Result<T>` type alias.

## Propagating Errors

Use `.context()` to convert a lower-level error into your domain error:

```rust
use std::fs;

pub fn read_config(path: &str) -> Result<String> {
    fs::read_to_string(path).context(ReadFileSnafu { path })
}
```

The `ReadFileSnafu` selector is auto-generated from the `ReadFile` variant. It captures `path` and wraps the `io::Error` as `source`.

For quick-and-dirty context when you don't need structured fields:

```rust
use snafu::Whatever;

fn do_thing() -> Result<(), Whatever> {
    std::fs::remove_file("/tmp/x").whatever_context("failed to clean up temp file")?;
    Ok(())
}
```

## Creating Errors Directly

When there's no underlying error to wrap, use `.fail()`:

```rust
pub fn validate(input: &str) -> Result<()> {
    if input.is_empty() {
        return InvalidFormatSnafu {
            message: "input must not be empty",
        }
        .fail();
    }
    Ok(())
}
```

## Module Result Type

Every module with its own error enum should define:

```rust
pub type Result<T> = std::result::Result<T, MyModuleError>;
```

This keeps signatures clean:

```rust
// Good
pub fn parse(input: &str) -> Result<Config> { ... }

// Avoid
pub fn parse(input: &str) -> std::result::Result<Config, MyModuleError> { ... }
```

## Rules

> **Warning**: Never use `thiserror` or manual `impl Error` for new code. Never use `.unwrap()` outside of tests — use `.expect("why this is safe")` if you must assert.
