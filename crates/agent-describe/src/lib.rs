//! Self-describing CLI protocol for AI agents (`agent-cli/1`).

pub use agent_describe_derive::AgentDescribe;

pub mod response;
pub mod schema;

pub use response::AgentResponse;
pub use schema::{AgentSchema, ArgSchema, CommandSchema, args_from_clap_command};
