# Configuration

## Location

Configuration lives at `~/.{project-name}/config.toml`. It is auto-created with defaults on first use.

## Structure

`AppConfig` is the top-level struct. Each section is a nested struct tagged with `#[serde(default)]` so missing fields fall back to defaults.

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppConfig {
    #[serde(default)]
    pub general: GeneralConfig,
    // add new sections here
}
```

## Adding a Section

Full example adding a `DownloadConfig` section.

### 1. Define the struct with a Default impl

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DownloadConfig {
    pub output_dir: String,
    pub max_retries: u32,
}

impl Default for DownloadConfig {
    fn default() -> Self {
        Self {
            output_dir: "./downloads".into(),
            max_retries: 3,
        }
    }
}
```

### 2. Add to AppConfig

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppConfig {
    #[serde(default)]
    pub general: GeneralConfig,
    #[serde(default)]
    pub download: DownloadConfig,
}
```

### 3. Wire the CLI helpers

Update these three functions so `config set` / `config get` work for your new fields:

- **`set_config_field`** — match on `"download.output_dir"`, `"download.max_retries"`, etc. and mutate the config.
- **`get_config_field`** — match on the same keys and return the current value as a string.
- **`config_as_map`** — insert each field into the map so `config list` can display them.

## Reading

```rust
let cfg = app_config::load(); // returns &'static AppConfig (OnceLock-cached)
println!("{}", cfg.download.output_dir);
```

The config is loaded once and cached for the lifetime of the process.

## Writing

```rust
let mut cfg = app_config::load().clone();
cfg.download.output_dir = "./out".into();
app_config::save(&cfg)?;
```

## CLI Commands

```bash
# Set a value
myapp config set download.output_dir ./out

# Get a value
myapp config get download.output_dir
```
