//! Setup command: collect parameters, validate, render template, run post-setup.

use std::io::{self, BufRead, Write};
use std::path::PathBuf;

use snafu::ResultExt;

use crate::error::{self, IoSnafu, ValidationSnafu};

/// Collected setup parameters.
pub struct SetupParams {
    pub project_name: String,
    pub crate_name: String,
    pub github_org: String,
    pub output_dir: PathBuf,
}

/// Collect setup parameters from CLI args and interactive prompts.
pub fn collect_params(
    name: Option<String>,
    org: Option<String>,
    path: Option<PathBuf>,
) -> error::Result<SetupParams> {
    let (project_name, github_org) = {
        let stdin = io::stdin();
        let mut reader = stdin.lock();

        let project_name = match name {
            Some(n) => n,
            None => prompt(&mut reader, "Project name (kebab-case)")?,
        };

        validate_project_name(&project_name)?;

        let github_org = match org {
            Some(o) => o,
            None => prompt_with_default(&mut reader, "GitHub org/username", "rararulab")?,
        };

        drop(reader);
        (project_name, github_org)
    };

    let crate_name = project_name.replace('-', "_");

    let output_dir = path.map_or_else(
        || PathBuf::from(&project_name),
        |p| p.join(&project_name),
    );

    if output_dir.exists() {
        return ValidationSnafu {
            message: format!("directory already exists: {}", output_dir.display()),
        }
        .fail();
    }

    Ok(SetupParams {
        project_name,
        crate_name,
        github_org,
        output_dir,
    })
}

/// Run the full setup: render template + post-setup steps.
pub async fn run(params: &SetupParams) -> error::Result<()> {
    eprintln!(
        "Creating project \"{}\" (org: {})...",
        params.project_name, params.github_org
    );

    std::fs::create_dir_all(&params.output_dir).context(IoSnafu)?;

    crate::template::render(
        &params.output_dir,
        &params.project_name,
        &params.crate_name,
        &params.github_org,
    )
    .map_err(|e| error::AppError::Setup {
        message: e.to_string(),
    })?;

    crate::post_setup::run(&params.output_dir, &params.project_name).await;

    Ok(())
}

fn validate_project_name(name: &str) -> error::Result<()> {
    if name.is_empty() {
        return ValidationSnafu {
            message: "project name cannot be empty".to_string(),
        }
        .fail();
    }

    if !name
        .chars()
        .all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '-')
    {
        return ValidationSnafu {
            message: format!(
                "project name must be kebab-case (lowercase letters, digits, hyphens): {name}"
            ),
        }
        .fail();
    }

    if name.starts_with('-') || name.ends_with('-') {
        return ValidationSnafu {
            message: "project name cannot start or end with a hyphen".to_string(),
        }
        .fail();
    }

    Ok(())
}

fn prompt(reader: &mut impl BufRead, label: &str) -> error::Result<String> {
    eprint!("{label}: ");
    io::stderr().flush().context(IoSnafu)?;
    let mut input = String::new();
    reader.read_line(&mut input).context(IoSnafu)?;
    let trimmed = input.trim().to_string();
    if trimmed.is_empty() {
        return ValidationSnafu {
            message: format!("{label} is required"),
        }
        .fail();
    }
    Ok(trimmed)
}

fn prompt_with_default(
    reader: &mut impl BufRead,
    label: &str,
    default: &str,
) -> error::Result<String> {
    eprint!("{label} [{default}]: ");
    io::stderr().flush().context(IoSnafu)?;
    let mut input = String::new();
    reader.read_line(&mut input).context(IoSnafu)?;
    let trimmed = input.trim();
    if trimmed.is_empty() {
        Ok(default.to_string())
    } else {
        Ok(trimmed.to_string())
    }
}
