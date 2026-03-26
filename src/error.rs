//! Application-level error types.

use snafu::Snafu;

#[derive(Debug, Snafu)]
#[snafu(visibility(pub))]
pub enum AppError {
    #[snafu(display("IO error: {source}"))]
    Io { source: std::io::Error },

    #[snafu(display("{message}"))]
    Validation { message: String },

    #[snafu(display("setup failed: {message}"))]
    Setup { message: String },

    #[snafu(display("post-setup command failed: {message}"))]
    PostSetup { message: String },
}

pub type Result<T> = std::result::Result<T, AppError>;
