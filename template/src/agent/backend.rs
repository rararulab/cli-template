//! CLI backend definitions for different AI tools.
//!
//! Provides table-driven presets for configuring various agent CLIs
//! (Claude, Kiro, Gemini, Codex, etc.) with the correct flags and
//! prompt-passing conventions.

use std::io::Write;

use snafu::Snafu;
use tempfile::NamedTempFile;

use super::config::AgentConfig;

/// Module-level result type.
pub type Result<T> = std::result::Result<T, BackendError>;

/// Prompts longer than this are written to a temp file and the agent is
/// asked to read from that file instead. This avoids OS `ARG_MAX` limits.
const LARGE_PROMPT_THRESHOLD: usize = 7000;

/// Output format supported by a CLI backend.
///
/// This allows adapters to declare whether they emit structured JSON
/// for real-time streaming or plain text output.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum OutputFormat {
    /// Plain text output (default for most adapters).
    #[default]
    Text,
    /// Newline-delimited JSON stream (Claude with `--output-format stream-json`).
    StreamJson,
    /// Newline-delimited JSON stream (Pi with `--mode json`).
    PiStreamJson,
    /// Agent Client Protocol over stdio (Kiro v2).
    Acp,
}

/// Errors that can occur when constructing a backend.
#[derive(Debug, Snafu)]
#[snafu(visibility(pub))]
pub enum BackendError {
    /// Custom backend requires a command to be specified.
    #[snafu(display("custom backend requires a command to be specified"))]
    CustomBackendRequiresCommand,

    /// Unknown backend name.
    #[snafu(display("unknown backend: {name}"))]
    UnknownBackend {
        /// The unrecognized backend name.
        name: String,
    },
}

/// How to pass prompts to the CLI tool.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PromptMode {
    /// Pass prompt as a command-line argument.
    Arg,
    /// Write prompt to stdin.
    Stdin,
}

/// Preset configuration for a named backend.
struct BackendPreset {
    command: &'static str,
    args: &'static [&'static str],
    prompt_mode: PromptMode,
    prompt_flag: Option<&'static str>,
    output_format: OutputFormat,
}

impl From<&BackendPreset> for CliBackend {
    fn from(preset: &BackendPreset) -> Self {
        Self {
            command: preset.command.to_string(),
            args: preset.args.iter().map(|s| (*s).to_string()).collect(),
            prompt_mode: preset.prompt_mode,
            prompt_flag: preset.prompt_flag.map(str::to_string),
            output_format: preset.output_format,
            env_vars: vec![],
        }
    }
}

/// Headless/autonomous backend presets (non-interactive, exits after completion).
static HEADLESS_PRESETS: &[(&str, BackendPreset)] = &[
    ("claude", BackendPreset {
        command: "claude",
        args: &["--dangerously-skip-permissions", "--verbose", "--output-format", "stream-json"],
        prompt_mode: PromptMode::Arg,
        prompt_flag: Some("-p"),
        output_format: OutputFormat::StreamJson,
    }),
    ("kiro", BackendPreset {
        command: "kiro-cli",
        args: &["chat", "--no-interactive", "--trust-all-tools"],
        prompt_mode: PromptMode::Arg,
        prompt_flag: None,
        output_format: OutputFormat::Text,
    }),
    ("kiro-acp", BackendPreset {
        command: "kiro-cli",
        args: &["acp"],
        prompt_mode: PromptMode::Stdin,
        prompt_flag: None,
        output_format: OutputFormat::Acp,
    }),
    ("gemini", BackendPreset {
        command: "gemini",
        args: &["--yolo"],
        prompt_mode: PromptMode::Arg,
        prompt_flag: Some("-p"),
        output_format: OutputFormat::Text,
    }),
    ("codex", BackendPreset {
        command: "codex",
        args: &["exec", "--full-auto"],
        prompt_mode: PromptMode::Arg,
        prompt_flag: None,
        output_format: OutputFormat::Text,
    }),
    ("amp", BackendPreset {
        command: "amp",
        args: &["--dangerously-allow-all"],
        prompt_mode: PromptMode::Arg,
        prompt_flag: Some("-x"),
        output_format: OutputFormat::Text,
    }),
    ("copilot", BackendPreset {
        command: "copilot",
        args: &["--allow-all-tools"],
        prompt_mode: PromptMode::Arg,
        prompt_flag: Some("-p"),
        output_format: OutputFormat::Text,
    }),
    ("opencode", BackendPreset {
        command: "opencode",
        args: &["run"],
        prompt_mode: PromptMode::Arg,
        prompt_flag: None,
        output_format: OutputFormat::Text,
    }),
    ("pi", BackendPreset {
        command: "pi",
        args: &["-p", "--mode", "json", "--no-session"],
        prompt_mode: PromptMode::Arg,
        prompt_flag: None,
        output_format: OutputFormat::PiStreamJson,
    }),
    ("roo", BackendPreset {
        command: "roo",
        args: &["--print", "--ephemeral"],
        prompt_mode: PromptMode::Arg,
        prompt_flag: None,
        output_format: OutputFormat::Text,
    }),
];

/// Interactive backend presets (TUI mode with initial prompt support).
static INTERACTIVE_PRESETS: &[(&str, BackendPreset)] = &[
    ("claude", BackendPreset {
        command: "claude",
        args: &["--dangerously-skip-permissions"],
        prompt_mode: PromptMode::Arg,
        prompt_flag: None,
        output_format: OutputFormat::Text,
    }),
    ("kiro", BackendPreset {
        command: "kiro-cli",
        args: &["chat", "--trust-all-tools"],
        prompt_mode: PromptMode::Arg,
        prompt_flag: None,
        output_format: OutputFormat::Text,
    }),
    ("gemini", BackendPreset {
        command: "gemini",
        args: &["--yolo"],
        prompt_mode: PromptMode::Arg,
        prompt_flag: Some("-i"),
        output_format: OutputFormat::Text,
    }),
    ("codex", BackendPreset {
        command: "codex",
        args: &[],
        prompt_mode: PromptMode::Arg,
        prompt_flag: None,
        output_format: OutputFormat::Text,
    }),
    ("amp", BackendPreset {
        command: "amp",
        args: &[],
        prompt_mode: PromptMode::Arg,
        prompt_flag: Some("-x"),
        output_format: OutputFormat::Text,
    }),
    ("copilot", BackendPreset {
        command: "copilot",
        args: &[],
        prompt_mode: PromptMode::Arg,
        prompt_flag: Some("-p"),
        output_format: OutputFormat::Text,
    }),
    ("opencode", BackendPreset {
        command: "opencode",
        args: &[],
        prompt_mode: PromptMode::Arg,
        prompt_flag: Some("--prompt"),
        output_format: OutputFormat::Text,
    }),
    ("pi", BackendPreset {
        command: "pi",
        args: &["--no-session"],
        prompt_mode: PromptMode::Arg,
        prompt_flag: None,
        output_format: OutputFormat::Text,
    }),
    ("roo", BackendPreset {
        command: "roo",
        args: &[],
        prompt_mode: PromptMode::Arg,
        prompt_flag: None,
        output_format: OutputFormat::Text,
    }),
];

/// Looks up a named preset in a table and converts it to a [`CliBackend`].
fn lookup_preset(table: &[(&str, BackendPreset)], name: &str) -> Result<CliBackend> {
    table
        .iter()
        .find(|(n, _)| *n == name)
        .map(|(_, preset)| CliBackend::from(preset))
        .ok_or_else(|| BackendError::UnknownBackend { name: name.to_string() })
}

/// Prepared command ready for execution.
///
/// Returned by [`CliBackend::build_command`]. The `_temp_file` handle must
/// be kept alive for the duration of command execution — dropping it deletes
/// the underlying file.
pub struct CommandSpec {
    /// The command binary to execute.
    pub command: String,
    /// Fully resolved argument list (includes prompt).
    pub args: Vec<String>,
    /// If set, this string should be written to the child's stdin.
    pub stdin_input: Option<String>,
    /// Temp file handle — kept alive so the agent can read from it.
    _temp_file: Option<NamedTempFile>,
}

/// A CLI backend configuration for executing prompts.
///
/// Factory methods construct backends with the correct flags for each
/// supported agent CLI. All prompts are passed to CLI processes directly
/// via [`std::process::Command`] (no shell involved), so there is no
/// shell injection risk. However, callers should treat this as a local
/// execution boundary — only pass prompts from trusted, local sources.
#[derive(Debug, Clone)]
pub struct CliBackend {
    /// The command to execute.
    pub command: String,
    /// Additional arguments before the prompt.
    pub args: Vec<String>,
    /// How to pass the prompt.
    pub prompt_mode: PromptMode,
    /// Argument flag for prompt (if `prompt_mode` is `Arg`).
    pub prompt_flag: Option<String>,
    /// Output format emitted by this backend.
    pub output_format: OutputFormat,
    /// Environment variables to set when spawning the process.
    pub env_vars: Vec<(String, String)>,
}

impl CliBackend {
    /// Creates a backend from an [`AgentConfig`].
    ///
    /// Delegates to [`Self::from_name`] for named backends and applies
    /// config overrides (extra args, command path).
    ///
    /// # Errors
    /// Returns [`BackendError`] if the backend is "custom" but no command is
    /// specified, or if the backend name is unrecognized.
    pub fn from_agent_config(config: &AgentConfig) -> Result<Self> {
        if config.backend == "custom" {
            return Self::custom(config);
        }

        let mut backend = Self::from_name(&config.backend)?;

        // Apply configured extra args for named backends too.
        backend.args.extend(config.args.iter().cloned());
        if backend.command == "codex" {
            Self::reconcile_codex_args(&mut backend.args);
        }

        // Honor command override for named backends (e.g., custom binary path)
        if let Some(ref cmd) = config.command {
            backend.command.clone_from(cmd);
        }

        Ok(backend)
    }

    /// Creates a headless backend from a named preset.
    ///
    /// # Errors
    /// Returns [`BackendError::UnknownBackend`] if the name is not recognized.
    pub fn from_name(name: &str) -> Result<Self> {
        lookup_preset(HEADLESS_PRESETS, name)
    }

    /// Creates a headless backend with additional args.
    ///
    /// # Errors
    /// Returns error if the backend name is invalid.
    pub fn from_name_with_args(name: &str, extra_args: &[String]) -> Result<Self> {
        let mut backend = Self::from_name(name)?;
        backend.args.extend(extra_args.iter().cloned());
        if backend.command == "codex" {
            Self::reconcile_codex_args(&mut backend.args);
        }
        Ok(backend)
    }

    /// Creates an interactive backend with initial prompt support.
    ///
    /// # Errors
    /// Returns [`BackendError::UnknownBackend`] if the backend name is not recognized.
    pub fn for_interactive_prompt(name: &str) -> Result<Self> {
        lookup_preset(INTERACTIVE_PRESETS, name)
    }

    /// Creates a custom backend from configuration.
    ///
    /// # Errors
    /// Returns [`BackendError::CustomBackendRequiresCommand`] if no command is specified.
    pub fn custom(config: &AgentConfig) -> Result<Self> {
        let command = config
            .command
            .clone()
            .ok_or(BackendError::CustomBackendRequiresCommand)?;
        let prompt_mode = match config.prompt_mode {
            super::config::ConfigPromptMode::Stdin => PromptMode::Stdin,
            super::config::ConfigPromptMode::Arg => PromptMode::Arg,
        };

        Ok(Self {
            command,
            args: config.args.clone(),
            prompt_mode,
            prompt_flag: config.prompt_flag.clone(),
            output_format: OutputFormat::Text,
            env_vars: vec![],
        })
    }

    /// Builds roo prompt-file args: writes prompt to a temp file and
    /// appends `--prompt-file <path>` to args. Falls back to positional
    /// arg if temp file creation fails.
    fn build_roo_prompt_file(
        args: &mut Vec<String>,
        prompt: &str,
    ) -> (Option<String>, Option<NamedTempFile>) {
        match NamedTempFile::new() {
            Ok(mut file) => {
                if let Err(e) = file.write_all(prompt.as_bytes()) {
                    tracing::warn!("Failed to write roo prompt to temp file: {e}");
                    args.push(prompt.to_string());
                    (None, None)
                } else {
                    args.push("--prompt-file".to_string());
                    args.push(file.path().display().to_string());
                    (None, Some(file))
                }
            }
            Err(e) => {
                tracing::warn!("Failed to create temp file for roo: {e}");
                args.push(prompt.to_string());
                (None, None)
            }
        }
    }

    /// Builds the full command with arguments for execution.
    ///
    /// # Safety assumptions
    ///
    /// The prompt is passed directly to the child process via
    /// [`std::process::Command`] — no shell is involved, so there is no
    /// shell-injection risk. This function is intended for local, trusted
    /// prompts only. Do not pass untrusted external input without
    /// validation.
    pub fn build_command(&self, prompt: &str, interactive: bool) -> CommandSpec {
        let mut args = self.args.clone();

        // Filter args based on execution mode
        if interactive {
            args = self.filter_args_for_interactive(args);
        }

        // Handle prompt passing: Roo uses --prompt-file, all others use temp file for large prompts
        let (stdin_input, temp_file) = match self.prompt_mode {
            PromptMode::Arg => {
                // Roo headless: always use --prompt-file for all prompts
                if self.command == "roo" && args.contains(&"--print".to_string()) {
                    Self::build_roo_prompt_file(&mut args, prompt)
                } else {
                    let (prompt_text, temp_file) = if prompt.len() > LARGE_PROMPT_THRESHOLD {
                        match NamedTempFile::new() {
                            Ok(mut file) => {
                                if let Err(e) = file.write_all(prompt.as_bytes()) {
                                    tracing::warn!(
                                        "Failed to write prompt to temp file: {e}"
                                    );
                                    (prompt.to_string(), None)
                                } else {
                                    let path = file.path().display().to_string();
                                    (
                                        format!(
                                            "Please read and execute the task in {path}"
                                        ),
                                        Some(file),
                                    )
                                }
                            }
                            Err(e) => {
                                tracing::warn!("Failed to create temp file: {e}");
                                (prompt.to_string(), None)
                            }
                        }
                    } else {
                        (prompt.to_string(), None)
                    };

                    if let Some(ref flag) = self.prompt_flag {
                        args.push(flag.clone());
                    }
                    args.push(prompt_text);
                    (None, temp_file)
                }
            }
            PromptMode::Stdin => (Some(prompt.to_string()), None),
        };

        tracing::debug!(
            command = %self.command,
            args_count = args.len(),
            prompt_len = prompt.len(),
            interactive = interactive,
            uses_stdin = stdin_input.is_some(),
            uses_temp_file = temp_file.is_some(),
            "Built CLI command"
        );
        tracing::trace!(prompt = %prompt, "Full prompt content");

        CommandSpec {
            command: self.command.clone(),
            args,
            stdin_input,
            _temp_file: temp_file,
        }
    }

    /// Filters args for interactive mode per spec table.
    fn filter_args_for_interactive(&self, args: Vec<String>) -> Vec<String> {
        match self.command.as_str() {
            "kiro-cli" => args
                .into_iter()
                .filter(|a| a != "--no-interactive")
                .collect(),
            "codex" => args.into_iter().filter(|a| a != "--full-auto").collect(),
            "amp" => args
                .into_iter()
                .filter(|a| a != "--dangerously-allow-all")
                .collect(),
            "copilot" => args
                .into_iter()
                .filter(|a| a != "--allow-all-tools")
                .collect(),
            "roo" => args
                .into_iter()
                .filter(|a| a != "--print" && a != "--ephemeral")
                .collect(),
            _ => args, // claude, gemini, opencode unchanged
        }
    }

    /// Reconciles codex args to resolve conflicting and deprecated flags.
    ///
    /// Replaces deprecated `--dangerously-bypass-approvals-and-sandbox` and
    /// `--yolo` with `--full-auto`, and deduplicates `--full-auto` entries.
    fn reconcile_codex_args(args: &mut Vec<String>) {
        let had_deprecated = args.iter().any(|arg| {
            arg == "--dangerously-bypass-approvals-and-sandbox" || arg == "--yolo"
        });
        if had_deprecated {
            args.retain(|arg| {
                arg != "--dangerously-bypass-approvals-and-sandbox" && arg != "--yolo"
            });
            if !args.iter().any(|arg| arg == "--full-auto") {
                if let Some(pos) = args.iter().position(|arg| arg == "exec") {
                    args.insert(pos + 1, "--full-auto".to_string());
                } else {
                    args.push("--full-auto".to_string());
                }
            }
        }

        // Collapse duplicate --full-auto entries.
        let mut seen = false;
        args.retain(|arg| {
            if arg == "--full-auto" {
                if seen {
                    return false;
                }
                seen = true;
            }
            true
        });
    }
}
