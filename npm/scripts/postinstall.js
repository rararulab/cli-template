#!/usr/bin/env node

// Downloads the correct platform-specific binary from GitHub releases.
// Falls back gracefully with instructions if download fails.

import { chmodSync, existsSync, mkdirSync, unlinkSync } from "node:fs";
import { join, dirname } from "node:path";
import { fileURLToPath } from "node:url";
import { execSync } from "node:child_process";

const __dirname = dirname(fileURLToPath(import.meta.url));
const pkg = JSON.parse(
  await import("node:fs").then((fs) =>
    fs.promises.readFile(join(__dirname, "..", "package.json"), "utf8")
  )
);
const VERSION = pkg.version;
const NAME = "{{project-name}}";
const ORG = "{{github-org}}";

const PLATFORM_MAP = {
  "darwin-x64": `${NAME}-v${VERSION}-x86_64-apple-darwin`,
  "darwin-arm64": `${NAME}-v${VERSION}-aarch64-apple-darwin`,
  "linux-x64": `${NAME}-v${VERSION}-x86_64-unknown-linux-gnu`,
  "linux-arm64": `${NAME}-v${VERSION}-aarch64-unknown-linux-gnu`,
};

const key = `${process.platform}-${process.arch}`;
const artifact = PLATFORM_MAP[key];

if (!artifact) {
  console.warn(
    `[${NAME}] Unsupported platform: ${key}\n` +
    `Supported: ${Object.keys(PLATFORM_MAP).join(", ")}\n` +
    `Build from source: cargo install --git https://github.com/${ORG}/${NAME}`
  );
  process.exit(0); // Don't fail install for unsupported platforms
}

const binDir = join(__dirname, "..", "bin");
const binPath = join(binDir, NAME);

// Skip if binary already exists
if (existsSync(binPath)) {
  console.log(`[${NAME}] Binary already exists, skipping download.`);
  process.exit(0);
}

const url = `https://github.com/${ORG}/${NAME}/releases/download/v${VERSION}/${artifact}.tar.gz`;

console.log(`[${NAME}] Downloading ${key} binary from v${VERSION}...`);

try {
  mkdirSync(binDir, { recursive: true });

  const tmpFile = join(binDir, `${NAME}.tar.gz`);

  // Download using curl (available on all supported platforms)
  execSync(`curl -fsSL "${url}" -o "${tmpFile}"`, { stdio: "pipe" });

  // Extract
  execSync(`tar -xzf "${tmpFile}" -C "${binDir}"`, { stdio: "pipe" });

  // Clean up archive
  unlinkSync(tmpFile);

  // Make executable
  chmodSync(binPath, 0o755);

  console.log(`[${NAME}] Installed successfully.`);
} catch (err) {
  console.warn(
    `[${NAME}] Failed to download binary: ${err.message}\n` +
    `You can install manually:\n` +
    `  curl -fsSL "${url}" | tar -xz -C "${binDir}"\n` +
    `Or build from source:\n` +
    `  cargo install --git https://github.com/${ORG}/${NAME}`
  );
  // Don't fail the install — the bin/cli.js wrapper will show a helpful error
  process.exit(0);
}
