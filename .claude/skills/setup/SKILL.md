---
name: setup
description: "Install, set up, and configure a Rust CLI built from cli-template. Use when user says: 'install this CLI', 'set up the project', 'getting started', 'how to use', 'cargo generate', 'npx install', 'configure', 'post-setup', 'initialize project', 'new project from template', 'what commands are available', 'how do I run this'. Actions: install, setup, init, configure, generate, build, run. Objects: CLI, binary, template, project, config, npx, cargo."
---

IRON LAW: NEVER skip the post-setup checklist. An unconfigured project will fail CI and confuse contributors.

# Setup & Installation

## Installation Methods

Pick ONE based on the user's environment:

### npx (no Rust required)
```bash
npx @<org>/<project-name> --help
```

### cargo install (from source)
```bash
cargo install --path .
```

### From template (new project)
```bash
cargo generate rararulab/cli-template
cd <your-project-name>
```

After generation, `{{project-name}}` and `{{crate_name}}` placeholders are replaced automatically.

## Post-Setup Workflow ⚠️ REQUIRED

Run through every item. Do NOT skip any step.

```
- [ ] 1. Update CLAUDE.md — fill in "Project Identity" section with what the CLI does
- [ ] 2. Update Cargo.toml — fill in `description` field
- [ ] 3. Install pre-commit hooks: `just setup-hooks`
- [ ] 4. Verify everything works: `just pre-commit`
- [ ] 5. Push to GitHub and confirm CI passes
```

Ask: "Did CI pass? If not, what failed?" — fix before proceeding.

## Usage

### CLI commands
```bash
<project-name> --help              # Show all commands
<project-name> agent "prompt"      # Run AI agent backend
<project-name> config set key val  # Update config
```

### Development commands
```bash
just fmt          # Format code
just clippy       # Run clippy
just test         # Run tests
just pre-commit   # All checks (format + lint + test)
just build        # Build debug binary
```

### Configuration

Config file: `~/.<project-name>/config.toml`

Override via CLI: `<project-name> config set <key> <value>`

For conventions (error handling, commit style, workflow), read CLAUDE.md — do NOT duplicate here.

## Anti-Patterns

- Do NOT start coding before completing the post-setup checklist
- Do NOT hardcode the project name — use the config system or CLI args
- Do NOT skip `just pre-commit` before first push — template defaults may need adjustment
- Do NOT duplicate CLAUDE.md conventions in this skill — reference the guides instead
