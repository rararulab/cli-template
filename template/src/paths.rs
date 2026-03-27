//! Centralized path management for application data directories.
//!
//! All paths derive from a single data root, resolved once via `OnceLock`.
//! The root can be overridden by setting the `APP_DATA_DIR` environment variable.
//!
//! Call [`init_data_dir`] early in startup to validate paths before use.

use std::{
    path::{Path, PathBuf},
    sync::OnceLock,
};

use snafu::ensure;

use crate::error::{self, PathResolutionSnafu};

static DATA_DIR: OnceLock<PathBuf> = OnceLock::new();

/// Validate and cache the data directory.
///
/// Resolves the root in order:
/// 1. `APP_DATA_DIR` env var (must be non-empty and an absolute path)
/// 2. `~/.{{project-name}}`
///
/// Must be called once during startup before any calls to [`data_dir`].
pub fn init_data_dir() -> error::Result<&'static Path> {
    if let Some(existing) = DATA_DIR.get() {
        return Ok(existing.as_path());
    }

    let path = if let Ok(dir) = std::env::var("APP_DATA_DIR") {
        let path = PathBuf::from(&dir);
        ensure!(
            !dir.is_empty() && path.is_absolute(),
            PathResolutionSnafu {
                message: format!("APP_DATA_DIR must be a non-empty absolute path, got: {dir:?}")
            }
        );
        path
    } else {
        dirs::home_dir()
            .ok_or_else(|| {
                PathResolutionSnafu {
                    message: "home directory must be resolvable — set APP_DATA_DIR as a fallback"
                        .to_string(),
                }
                .build()
            })?
            .join(".{{project-name}}")
    };

    Ok(DATA_DIR.get_or_init(|| path))
}

/// Root data directory, resolved during [`init_data_dir`].
///
/// # Panics
/// Panics if called before [`init_data_dir`].
pub fn data_dir() -> &'static Path {
    DATA_DIR
        .get()
        .expect("data_dir() called before init_data_dir()")
}

/// Config file path: `<data>/config.toml`
pub fn config_file() -> PathBuf { data_dir().join("config.toml") }

/// Cache directory: `<data>/cache`
pub fn cache_dir() -> PathBuf { data_dir().join("cache") }
