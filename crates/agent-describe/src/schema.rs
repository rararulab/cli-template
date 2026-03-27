use serde::Serialize;

/// Top-level schema for `--agent-describe` output.
#[derive(Debug, Serialize)]
pub struct AgentSchema {
    pub protocol: &'static str,
    pub name: String,
    pub version: String,
    pub description: String,
    pub commands: Vec<CommandSchema>,
    pub error_format: serde_json::Value,
}

#[derive(Debug, Serialize)]
pub struct CommandSchema {
    pub name: String,
    pub description: String,
    pub args: Vec<ArgSchema>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output: Option<serde_json::Value>,
}

#[derive(Debug, Serialize)]
pub struct ArgSchema {
    pub name: String,
    pub r#type: String,
    pub required: bool,
    pub description: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#enum: Option<Vec<String>>,
}

impl AgentSchema {
    /// Standard error format included in every schema.
    pub fn default_error_format() -> serde_json::Value {
        serde_json::json!({
            "type": "object",
            "properties": {
                "ok": { "const": false },
                "error": { "type": "string" },
                "suggestion": { "type": "string" }
            },
            "required": ["ok", "error"]
        })
    }
}

/// Extract argument schemas from a Clap `Command` at runtime.
///
/// Used by the derive macro to flatten subcommand args.
pub fn args_from_clap_command(cmd: &clap::Command) -> Vec<ArgSchema> {
    cmd.get_arguments()
        .filter(|a| a.get_id() != "help" && a.get_id() != "version")
        .map(|arg| {
            let name = if arg.get_long().is_some() {
                format!("--{}", arg.get_id())
            } else {
                arg.get_id().to_string()
            };
            let type_str = if arg.get_action().takes_values() {
                "string"
            } else {
                "bool"
            };
            let possible: Vec<String> = arg.get_possible_values()
                .iter()
                .filter_map(|v| v.get_name_and_aliases().next().map(String::from))
                .collect();

            ArgSchema {
                name,
                r#type: type_str.to_string(),
                required: arg.is_required_set(),
                description: arg.get_help()
                    .map(|s| s.to_string())
                    .unwrap_or_default(),
                r#enum: if possible.is_empty() { None } else { Some(possible) },
            }
        })
        .collect()
}
