#!/usr/bin/env node

// Thin wrapper that delegates to the platform-specific binary.
// The binary is downloaded during postinstall.

import { execFileSync } from "node:child_process";
import { existsSync } from "node:fs";
import { join, dirname } from "node:path";
import { fileURLToPath } from "node:url";

const __dirname = dirname(fileURLToPath(import.meta.url));
const binName = process.platform === "win32" ? "{{project-name}}.exe" : "{{project-name}}";
const binPath = join(__dirname, binName);

if (!existsSync(binPath)) {
  console.error(
    `Binary not found at ${binPath}\n` +
    `Run "npm rebuild" or install from source:\n` +
    `  cargo install --git https://github.com/{{github-org}}/{{project-name}}`
  );
  process.exit(1);
}

try {
  execFileSync(binPath, process.argv.slice(2), { stdio: "inherit" });
} catch (err) {
  process.exit(err.status ?? 1);
}
