# Design: Agent Quickstart Guide

**Date**: 2026-03-25
**Issue**: TBD

## Goal

Create `docs/guides/agent-quickstart.md` — a single comprehensive guide that teaches AI agents how to:
1. Initialize a new project from the cli-template (primary)
2. Add features to an initialized project (secondary)

Inspired by OpenCLI's CLI-EXPLORER.md pattern: decision trees, copy-paste templates, common pitfalls.

## Approach

Single file, referenced from CLAUDE.md via `@docs/guides/agent-quickstart.md`.

## Sections

### 1. Project Initialization
- Core flow diagram: clone → replace placeholders → customize → verify
- Complete placeholder inventory (3 variables across all files)
- `sed` one-liner to replace all placeholders
- Post-replacement checklist: Cargo.toml metadata, CLAUDE.md identity, README.md, paths.rs

### 2. Architecture Overview
- Module relationship diagram (ASCII)
- Data flow: CLI parsing → command dispatch → module logic → JSON output
- File-to-responsibility mapping table

### 3. Adding a Subcommand
- Decision tree: simple vs config-dependent vs HTTP-dependent
- Step-by-step: cli/mod.rs → main.rs dispatch → (optional) new module
- Copy-paste code template for each variant

### 4. Adding a Module
- New directory + mod.rs + error type + config section
- Wiring: lib.rs re-export, error.rs variant, app_config.rs section
- Copy-paste boilerplate

### 5. Common Patterns Reference
- snafu error propagation
- bon::Builder struct
- HTTP client usage
- Config read/write
- All as copy-paste snippets

### 6. Common Pitfalls
- Table format: mistake → symptom → fix
- Template-specific traps (forgotten placeholders, missing mod declarations, etc.)

### 7. Verification Checklist
- Post-init checklist
- Post-feature checklist

## Affected Files

- **New**: `docs/guides/agent-quickstart.md`
- **Modified**: `CLAUDE.md` (add `@` reference)

## Key Decisions

- Single file over multiple files (agent context window efficiency)
- Code templates use the actual template's `{{project-name}}` placeholders so they work pre- and post-initialization
- English content (per code-comments.md rule), but CLAUDE.md reference section name stays consistent with existing style
