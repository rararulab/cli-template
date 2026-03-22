//! Centralized path management for application data directories.
//!
//! All paths derive from a single data root, resolved once via `OnceLock`.
//! The root can be overridden by setting the `APP_DATA_DIR` environment variable.

use std::{
    path::{Path, PathBuf},
    sync::OnceLock,
};

static DATA_DIR: OnceLock<PathBuf> = OnceLock::new();

/// Root data directory, resolved in order:
/// 1. `APP_DATA_DIR` env var (must be non-empty and an absolute path)
/// 2. `~/.{{project-name}}`
pub fn data_dir() -> &'static Path {
    DATA_DIR.get_or_init(|| {
        if let Ok(dir) = std::env::var("APP_DATA_DIR") {
            let path = PathBuf::from(&dir);
            assert!(
                !dir.is_empty() && path.is_absolute(),
                "APP_DATA_DIR must be a non-empty absolute path, got: {dir:?}"
            );
            return path;
        }

        dirs::home_dir()
            .expect("home directory must be resolvable — set APP_DATA_DIR as a fallback")
            .join(".{{project-name}}")
    })
}

/// Config file path: `<data>/config.toml`
pub fn config_file() -> PathBuf { data_dir().join("config.toml") }

/// Cache directory: `<data>/cache`
pub fn cache_dir() -> PathBuf { data_dir().join("cache") }
