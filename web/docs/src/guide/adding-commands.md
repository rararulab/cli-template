# Adding Commands

Three patterns, from simplest to most complex.

## Pattern A: Simple Command (2 files)

Add a `count` command that counts lines in a file.

### 1. Add variant to `Command` enum

**`src/cli/mod.rs`**

```rust
/// Count lines in a file
Count {
    /// Path to the file
    path: std::path::PathBuf,
    /// Skip empty lines
    #[arg(long)]
    no_empty: bool,
},
```

### 2. Add dispatch

**`src/main.rs`**

```rust
Command::Count { path, no_empty } => {
    let content = std::fs::read_to_string(&path).context(IoSnafu)?;
    let count = if no_empty {
        content.lines().filter(|l| !l.trim().is_empty()).count()
    } else {
        content.lines().count()
    };
    eprintln!("{count} lines in {}", path.display());
    println!("{}", serde_json::json!({"ok": true, "action": "count", "lines": count}));
}
```

That's it. Two files, done.

---

## Pattern B: Config-Dependent Command

Everything from Pattern A, plus config integration.

### 1. Define config struct

**`src/app_config.rs`**

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DownloadConfig {
    /// Directory to save downloaded files
    #[serde(default = "default_output_dir")]
    pub output_dir: String,

    /// Maximum concurrent downloads
    #[serde(default = "default_max_concurrent")]
    pub max_concurrent: usize,
}

fn default_output_dir() -> String {
    "./downloads".to_string()
}

fn default_max_concurrent() -> usize {
    4
}

impl Default for DownloadConfig {
    fn default() -> Self {
        Self {
            output_dir: default_output_dir(),
            max_concurrent: default_max_concurrent(),
        }
    }
}
```

### 2. Add field to `AppConfig`

```rust
pub struct AppConfig {
    // ... existing fields ...
    #[serde(default)]
    pub download: DownloadConfig,
}
```

### 3. Wire config accessors

In `set_config_field`:

```rust
"download.output_dir" => {
    self.download.output_dir = value.to_string();
}
"download.max_concurrent" => {
    self.download.max_concurrent = value.parse().context(ParseIntSnafu)?;
}
```

In `get_config_field`:

```rust
"download.output_dir" => Some(self.download.output_dir.clone()),
"download.max_concurrent" => Some(self.download.max_concurrent.to_string()),
```

In `config_as_map`:

```rust
map.insert("download.output_dir".into(), self.download.output_dir.clone());
map.insert("download.max_concurrent".into(), self.download.max_concurrent.to_string());
```

### 4. Add command variant and dispatch

Same as Pattern A. Access config via `config.download.output_dir`.

---

## Pattern C: HTTP Command

Everything from Pattern A, plus an HTTP call using the shared client.

### Command variant

```rust
/// Fetch data from a URL
Fetch {
    /// Target URL
    url: String,
},
```

### Dispatch

```rust
Command::Fetch { url } => {
    let resp = http::client()
        .get(&url)
        .send().await.context(HttpSnafu)?
        .json::<serde_json::Value>().await.context(HttpSnafu)?;
    println!("{}", serde_json::to_string_pretty(&resp).context(SerializeSnafu)?);
}
```

`http::client()` returns a pre-configured `reqwest::Client` with timeouts and default headers already set.

---

## Extracting into a Module

When command logic outgrows the dispatch block, extract it.

### File structure

```
src/yourmodule/
  mod.rs       # //! doc + re-exports
  client.rs    # error enum + logic
```

### `src/yourmodule/mod.rs`

```rust
//! Your module — does X, Y, Z.

mod client;

pub use client::{YourModuleClient, YourModuleError};
```

### `src/yourmodule/client.rs`

```rust
use snafu::{ResultExt, Snafu};

#[derive(Debug, Snafu)]
#[snafu(visibility(pub))]
pub enum YourModuleError {
    #[snafu(display("request failed: {source}"))]
    Request { source: reqwest::Error },
}

pub type Result<T> = std::result::Result<T, YourModuleError>;

/// Client for interacting with the service.
pub struct YourModuleClient {
    http: reqwest::Client,
}

impl YourModuleClient {
    pub fn new(http: reqwest::Client) -> Self {
        Self { http }
    }

    pub async fn do_thing(&self) -> Result<()> {
        // ...
        Ok(())
    }
}
```

### Register in `src/lib.rs`

```rust
pub mod yourmodule;
```

### Add error variant to `src/error.rs`

```rust
#[snafu(display("yourmodule error: {source}"))]
YourModule { source: yourmodule::YourModuleError },
```

---

## Checklist

Before opening a PR, verify:

- [ ] Module has `//!` doc comment
- [ ] Error enum named `{Module}Error` with `#[derive(Debug, Snafu)]`
- [ ] All `pub` items have `///` doc comments
- [ ] Module registered in `src/lib.rs`
- [ ] Structs with 3+ fields use `#[derive(bon::Builder)]`
