# {{project-name}}

Install and run {{project-name}} via npm/npx.

## Quick Install

```bash
npx @{{github-org}}/{{project-name}} --help
```

## Global Install

```bash
npm install -g @{{github-org}}/{{project-name}}
{{project-name}} --help
```

## How It Works

This package downloads the pre-built binary for your platform from [GitHub Releases](https://github.com/{{github-org}}/{{project-name}}/releases). Supported platforms:

- macOS (x64, arm64)
- Linux (x64, arm64)

For other platforms, build from source:

```bash
cargo install --git https://github.com/{{github-org}}/{{project-name}}
```
