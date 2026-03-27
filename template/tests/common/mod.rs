//! Shared test helpers for integration tests.

use assert_cmd::Command;

/// Create a command for the project binary.
pub fn cmd() -> Command {
    Command::cargo_bin(env!("CARGO_PKG_NAME")).expect("binary should be built")
}
