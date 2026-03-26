# npx Distribution

## How It Works

An npm wrapper package detects the user's platform and architecture, downloads the matching pre-built Rust binary, and runs it. No Rust toolchain required on the user's machine.

## Directory Structure

```
npm/
├── package.json          # root wrapper with install/bin scripts
├── darwin-arm64/         # macOS Apple Silicon
├── darwin-x64/           # macOS Intel
├── linux-arm64/          # Linux ARM
└── linux-x64/            # Linux x86_64
```

Each platform directory contains its own `package.json` and is published as a separate npm package marked as an optional dependency.

## User Experience

```bash
npx @your-org/your-project --help
```

That's it. npm resolves the correct platform package automatically.

## Publishing

Automated by the `publish-npm.yml` workflow. On each GitHub release, it updates the version in every platform `package.json` and publishes all packages to npm.

## Customizing

Update `npm/package.json` and each platform `package.json` with:

- Your npm scope and package name (e.g., `@your-org/your-project`)
- The binary name that matches your Cargo build output
