# CLAUDE.md — {{project-name}} Development Guide

## Communication
- 用中文与用户交流

## Project Identity

{{project-name}} is a CLI tool built in Rust. TODO: describe your project here.

## Development Workflow

All changes — no matter how small — follow the issue → worktree → PR → merge flow. No exceptions.

@docs/guides/workflow.md
@docs/guides/commit-style.md

## Code Quality

@docs/guides/rust-style.md
@docs/guides/code-comments.md

## Guardrails

@docs/guides/anti-patterns.md

## Agent Protocol

This CLI implements `agent-cli/1`. Run `{{project-name}} --agent-describe` to get the full schema.
- All command outputs are typed structs with `#[derive(Serialize, JsonSchema)]`
- Response wrapper: `AgentResponse::ok(data)` / `AgentResponse::err(msg, suggestion)`
- Naming convention: command `Foo` → response type `FooResult` in `src/response.rs`
- Add `#[agent(skip)]` to exclude commands, `#[agent(output = T)]` to override convention

## Agent Quickstart

How to initialize a project from this template and add features to it.

@docs/guides/agent-quickstart.md
