#!/usr/bin/env node

// Platform-specific binary resolver.
// Resolves the correct binary from optionalDependencies, then spawns it.

import { spawnSync } from "node:child_process";
import { createRequire } from "node:module";

const require = createRequire(import.meta.url);

const PLATFORMS = {
  "darwin-arm64": "@{{github-org}}/{{project-name}}-darwin-arm64",
  "darwin-x64": "@{{github-org}}/{{project-name}}-darwin-x64",
  "linux-arm64": "@{{github-org}}/{{project-name}}-linux-arm64",
  "linux-x64": "@{{github-org}}/{{project-name}}-linux-x64",
};

function getBinaryPath() {
  const key = `${process.platform}-${process.arch}`;
  const pkg = PLATFORMS[key];

  if (!pkg) {
    console.error(
      `Unsupported platform: ${key}\n` +
      `Supported: ${Object.keys(PLATFORMS).join(", ")}\n` +
      `Build from source: cargo install --git https://github.com/{{github-org}}/{{project-name}}`
    );
    process.exit(1);
  }

  // Allow override via environment variable
  if (process.env.{{crate_name}}_BINARY) {
    return process.env.{{crate_name}}_BINARY;
  }

  try {
    return require.resolve(`${pkg}/{{project-name}}`);
  } catch {
    console.error(
      `Platform package ${pkg} not found.\n` +
      `Try reinstalling: npm install @{{github-org}}/{{project-name}}\n` +
      `Or build from source: cargo install --git https://github.com/{{github-org}}/{{project-name}}`
    );
    process.exit(1);
  }
}

const result = spawnSync(getBinaryPath(), process.argv.slice(2), {
  stdio: "inherit",
  // Forward signals to the child process
  windowsHide: true,
});

if (result.error) {
  console.error(result.error.message);
  process.exit(1);
}

process.exit(result.status ?? 1);
