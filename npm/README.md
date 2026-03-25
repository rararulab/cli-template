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

This package uses platform-specific `optionalDependencies`. When you install,
npm automatically downloads only the binary for your platform:

| Package | Platform |
|---------|----------|
| `@{{github-org}}/{{project-name}}-darwin-arm64` | macOS Apple Silicon |
| `@{{github-org}}/{{project-name}}-darwin-x64` | macOS Intel |
| `@{{github-org}}/{{project-name}}-linux-arm64` | Linux ARM64 |
| `@{{github-org}}/{{project-name}}-linux-x64` | Linux x86_64 |

No `postinstall` scripts. No network calls at install time beyond the npm registry.

For other platforms, build from source:

```bash
cargo install --git https://github.com/{{github-org}}/{{project-name}}
```
