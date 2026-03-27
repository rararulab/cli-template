use serde::Serialize;

/// Unified response wrapper for agent-friendly CLI output.
///
/// All commands return either `Ok { data }` or `Err { error, suggestion }`.
/// Serialized to JSON on stdout; human-readable text goes to stderr.
#[derive(Debug, Serialize)]
#[serde(untagged)]
pub enum AgentResponse<T: Serialize> {
    Ok {
        ok:   bool,
        data: T,
    },
    Err {
        ok:         bool,
        error:      String,
        suggestion: Option<String>,
    },
}

impl<T: Serialize> AgentResponse<T> {
    /// Create a success response wrapping the given data.
    pub fn ok(data: T) -> Self { Self::Ok { ok: true, data } }

    /// Create an error response with an optional suggestion for
    /// self-correction.
    pub fn err(error: impl Into<String>, suggestion: Option<impl Into<String>>) -> Self {
        Self::Err {
            ok:         false,
            error:      error.into(),
            suggestion: suggestion.map(Into::into),
        }
    }

    /// Serialize to JSON string.
    pub fn to_json(&self) -> String {
        serde_json::to_string(self).expect("AgentResponse serialization should not fail")
    }

    /// Print JSON to stdout.
    pub fn print(&self) {
        println!("{}", self.to_json());
    }
}
