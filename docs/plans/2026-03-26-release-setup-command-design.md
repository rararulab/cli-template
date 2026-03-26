# rara-cli-template Release & Setup Command Design

Date: 2026-03-26

## Overview

将 cli-template 项目从"模板仓库"转变为可安装的 CLI 工具 `rara-cli-template`，通过 Homebrew 和 shell installer 发布。核心功能是 `setup` 命令，帮助用户零依赖地快速创建新项目。

## CLI 设计

### 工具名

`rara-cli-template`

### 子命令

只有一个：`setup`（YAGNI）

### 用法

```bash
# 交互式
rara-cli-template setup

# flags 优先，缺少的交互补充
rara-cli-template setup my-cool-cli --org myorg

# 全 flags
rara-cli-template setup my-cool-cli --org myorg --path ./projects
```

### 参数

| 参数 | Flag | 交互 Prompt | 默认值 |
|------|------|------------|--------|
| project-name | 位置参数（可选） | "Project name (kebab-case):" | 无（必填） |
| github-org | `--org` | "GitHub org/username:" | `rararulab` |
| output path | `--path` | 无 | `./{project-name}` |

`crate_name` 自动从 `project-name` 转换（`-` → `_`），不询问。

## 项目结构

```
cli-template/
├── src/
│   ├── main.rs          # 入口，clap dispatch
│   ├── cli.rs           # Clap 定义：setup 子命令
│   ├── setup.rs         # setup 核心逻辑：收集参数 → 渲染模板 → post-setup
│   ├── template.rs      # include_dir! 嵌入模板 + 占位符替换引擎
│   ├── post_setup.rs    # git init → commit → cargo check → 输出 agent prompt
│   └── error.rs         # snafu 错误类型
├── template/            # 模板源文件（被 include_dir! 嵌入二进制）
│   ├── Cargo.toml.tmpl
│   ├── src/
│   ├── docs/
│   └── ...
├── Cargo.toml           # 独立 package，name = "rara-cli-template"
└── ...
```

### 关键点

- 模板文件放在 `template/` 目录，编译时通过 `include_dir` crate 嵌入二进制
- 当前项目根目录的模板文件迁移到 `template/` 下
- CLI 自身代码和模板代码完全分离

## 模板引擎

### 嵌入方式

```rust
use include_dir::{include_dir, Dir};
static TEMPLATE: Dir = include_dir!("$CARGO_MANIFEST_DIR/template");
```

### 占位符替换

| 占位符 | 替换值 | 示例 |
|--------|--------|------|
| `{{project-name}}` | 用户输入的项目名 | `my-cool-cli` |
| `{{crate_name}}` | project-name 转 snake_case | `my_cool_cli` |
| `{{github-org}}` | 用户输入的 org | `rararulab` |

### 文件处理

- 文本文件（`.rs`, `.toml`, `.yaml`, `.yml`, `.md`, `.json`, `.js`, `.sh`）：做占位符替换
- 二进制文件：直接复制
- `.tmpl` 后缀：替换后去掉后缀（`Cargo.toml.tmpl` → `Cargo.toml`）
- 保留目录结构和文件权限
- 不做条件逻辑、循环等模板语法，只做字符串替换

## Setup 执行流程

1. **收集参数** — flags 优先，缺少的用 stdin 交互询问
2. **校验** — project-name 必须是合法 kebab-case，目标目录不存在
3. **渲染模板** — 遍历 template/ 内嵌文件，替换占位符，写入目标目录
4. **Post-setup**：
   - `git init` + `git add -A` + commit `chore: init from rara-cli-template vX.Y.Z`
   - `cargo check`（验证编译通过，失败则警告但不中断）
   - 输出 agent-ready prompt

### Agent Prompt 输出

```
✅ Project my-cool-cli created at ./my-cool-cli

To start developing with an AI agent, copy the prompt below:

---
I have a new Rust CLI project "my-cool-cli" initialized from rara-cli-template.
The project is at ./my-cool-cli with git already initialized.

Read CLAUDE.md and docs/guides/agent-quickstart.md first, then:
1. Update CLAUDE.md with the project description
2. Replace the Hello example command with actual CLI commands
3. Customize ExampleConfig in src/app_config.rs
4. Run `just pre-commit` to verify everything passes
---
```

## Release & Distribution

### cargo-dist 配置

```toml
[workspace.metadata.dist]
cargo-dist-version = "0.28.0"
ci = "github"
installers = ["shell", "homebrew"]
targets = ["aarch64-apple-darwin", "x86_64-unknown-linux-gnu"]
tap = "rararulab/homebrew-tap"
publish-jobs = ["homebrew"]
```

### 发布流程

1. release-plz 检测 conventional commits → 创建 release PR
2. PR 合并后打 tag `vX.Y.Z`
3. cargo-dist GitHub Actions 触发：
   - 构建 macOS arm64 + Linux x86_64 二进制
   - 生成 shell installer
   - 生成 Homebrew formula → 推送到 `rararulab/homebrew-tap`
   - 创建 GitHub Release

### 安装方式

```bash
# Homebrew
brew install rararulab/homebrew-tap/rara-cli-template

# Shell installer
curl --proto '=https' --tlsv1.2 -LsSf https://github.com/rararulab/rara-cli-template/releases/latest/download/rara-cli-template-installer.sh | sh
```

### 不做

- NPM 发布（保留给模板生成的项目用）
- Windows 支持（暂时）
- 多模板变体
