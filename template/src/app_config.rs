//! Application configuration backed by TOML file.

use std::sync::OnceLock;

use serde::{Deserialize, Serialize};
use snafu::ResultExt;

use crate::agent::AgentConfig;
use crate::error::{self, ConfigParseSnafu};

static APP_CONFIG: OnceLock<AppConfig> = OnceLock::new();

/// Application configuration.
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct AppConfig {
    /// Example configuration section.
    pub example: ExampleConfig,
    /// Agent backend configuration.
    pub agent: AgentConfig,
}

/// Example configuration section — replace with your own.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct ExampleConfig {
    /// An example setting.
    pub setting: String,
}

impl Default for ExampleConfig {
    fn default() -> Self {
        Self {
            setting: "default-value".to_string(),
        }
    }
}

/// Load config from TOML file, cache in `OnceLock`, and return a reference.
///
/// If the config file does not exist, `AppConfig::default()` is used.
/// If the config file exists but is malformed, the parse error is returned.
///
/// Must be called once (typically at startup) before [`get`].
pub fn init() -> error::Result<&'static AppConfig> {
    if let Some(existing) = APP_CONFIG.get() {
        return Ok(existing);
    }

    let path = crate::paths::config_file();
    let cfg = if path.exists() {
        let settings = config::Config::builder()
            .add_source(config::File::from(path.as_ref()))
            .build()
            .context(ConfigParseSnafu)?;
        settings.try_deserialize().context(ConfigParseSnafu)?
    } else {
        AppConfig::default()
    };
    Ok(APP_CONFIG.get_or_init(|| cfg))
}

/// Infallible accessor — returns the cached config.
///
/// # Panics
///
/// Panics if [`init`] has not been called yet.
pub fn get() -> &'static AppConfig {
    APP_CONFIG
        .get()
        .expect("app_config::init() must be called before app_config::get()")
}

/// Save config to TOML file.
pub fn save(cfg: &AppConfig) -> std::io::Result<()> {
    let path = crate::paths::config_file();
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent)?;
    }
    let content = toml::to_string_pretty(cfg).expect("config serialization should not fail");
    std::fs::write(path, content)
}
