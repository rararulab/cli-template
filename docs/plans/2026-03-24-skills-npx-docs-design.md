# Design: Claude Code Skills, npx Install, and Documentation Updates

**Issue**: #14
**Date**: 2026-03-24

## Summary

Add three capabilities to cli-template:

1. **npm/npx install support** — An npm wrapper package that downloads pre-built
   platform binaries from GitHub Releases via a postinstall script. Users can run
   `npx @org/project --help` without installing Rust.

2. **Claude Code setup skill** — A `/setup` skill teaching Claude Code how to
   initialize and configure projects created from this template.

3. **Documentation updates** — Update README.md and the GitHub Pages landing page
   to surface npx installation and Claude Code skills.

## Approach

### npm Package (`npm/`)

- **Updated**: switched from postinstall download to optionalDependencies
  platform packages (Biome/Turbo pattern). Zero postinstall scripts.
- `package.json` with `bin` entry + `optionalDependencies` per platform
- `bin/cli.js` — resolves platform package via `require.resolve`, spawns binary
- Platform packages (`@org/cli-darwin-arm64`, etc.) published by CI
- Supports: macOS (x64, arm64), Linux (x64, arm64)
- npm automatically installs only the matching platform package via os/cpu fields

### CI Workflows

- `build-binaries.yml` — reusable workflow that cross-compiles for all 4 targets
- `publish-npm.yml` — triggered on GitHub release, builds binaries, attaches to
  release, publishes npm package
- `ci.yml` — extended with npm package validation job

### Setup Skill (`.claude/skills/setup/`)

- Covers project creation via `cargo generate`
- Post-setup configuration checklist
- Development commands and project conventions

### Documentation

- README.md: add Installation section (npx + cargo), setup skill description,
  npm directory in project structure
- `web/index.html`: add npx install terminal block, update feature cards
