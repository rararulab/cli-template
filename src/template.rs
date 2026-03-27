//! Template embedding and placeholder replacement.

use std::path::Path;

use include_dir::{Dir, include_dir};
use snafu::ResultExt;

use crate::error::{self, IoSnafu};

static TEMPLATE: Dir = include_dir!("$CARGO_MANIFEST_DIR/template");

/// Text file extensions that should have placeholders replaced.
const TEXT_EXTENSIONS: &[&str] = &[
    "rs", "toml", "yaml", "yml", "md", "json", "js", "sh", "lock",
];

/// Files/directories to skip when rendering.
const SKIP_PATTERNS: &[&str] = &["target", ".git", ".worktrees", "cargo-generate.toml"];

/// Render all template files into `output_dir`, replacing placeholders.
pub fn render(
    output_dir: &Path,
    project_name: &str,
    crate_name: &str,
    github_org: &str,
) -> error::Result<()> {
    render_dir(&TEMPLATE, output_dir, project_name, crate_name, github_org)
}

fn render_dir(
    dir: &Dir,
    output_dir: &Path,
    project_name: &str,
    crate_name: &str,
    github_org: &str,
) -> error::Result<()> {
    for entry in dir.dirs() {
        let dir_name = entry
            .path()
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("");
        if SKIP_PATTERNS.contains(&dir_name) {
            continue;
        }
        let target = output_dir.join(entry.path());
        std::fs::create_dir_all(&target).context(IoSnafu)?;
        render_dir(entry, output_dir, project_name, crate_name, github_org)?;
    }

    for file in dir.files() {
        let rel_path = file.path();

        if rel_path
            .components()
            .any(|c| SKIP_PATTERNS.contains(&c.as_os_str().to_str().unwrap_or("")))
        {
            continue;
        }

        let target_path = output_dir.join(rel_path);

        if let Some(parent) = target_path.parent() {
            std::fs::create_dir_all(parent).context(IoSnafu)?;
        }

        let contents = file.contents();

        if is_text_file(rel_path) {
            let text = String::from_utf8_lossy(contents);
            let replaced = replace_placeholders(&text, project_name, crate_name, github_org);
            std::fs::write(&target_path, replaced.as_bytes()).context(IoSnafu)?;
        } else {
            std::fs::write(&target_path, contents).context(IoSnafu)?;
        }

        #[cfg(unix)]
        if rel_path.extension().and_then(|e| e.to_str()) == Some("sh") {
            use std::os::unix::fs::PermissionsExt;
            let perms = std::fs::Permissions::from_mode(0o755);
            std::fs::set_permissions(&target_path, perms).context(IoSnafu)?;
        }
    }

    Ok(())
}

fn is_text_file(path: &Path) -> bool {
    let ext = path.extension().and_then(|e| e.to_str()).unwrap_or("");
    if TEXT_EXTENSIONS.contains(&ext) {
        return true;
    }
    let name = path.file_name().and_then(|n| n.to_str()).unwrap_or("");
    matches!(
        name,
        "justfile" | ".gitignore" | ".pre-commit-config.yaml" | "LICENSE" | "Dockerfile"
    )
}

fn replace_placeholders(
    text: &str,
    project_name: &str,
    crate_name: &str,
    github_org: &str,
) -> String {
    text.replace("{{project-name}}", project_name)
        .replace("{{crate_name}}", crate_name)
        .replace("{{github-org}}", github_org)
}
