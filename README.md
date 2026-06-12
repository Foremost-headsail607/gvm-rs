<div align="center">

# 🐹 gvm - Go Version Manager

**A fast, cross-platform Go version manager written in Rust.**
Install, switch, and pin any Go release - no `sudo`, no system dependencies, no fuss.

[![License: MIT](https://img.shields.io/badge/license-MIT-blue?style=flat-square)](LICENSE)
[![Rust](https://img.shields.io/badge/built_with-Rust-orange?style=flat-square&logo=rust)](https://www.rust-lang.org/)
[![Platform](https://img.shields.io/badge/platform-Windows_%7C_Linux_%7C_macOS-brightgreen?style=flat-square&logo=github)](https://github.com/jhonsferg/gvm/releases)
[![Version](https://img.shields.io/badge/version-1.0.0-blueviolet?style=flat-square)](https://github.com/jhonsferg/gvm/releases/tag/v1.0.0)
[![TLS](<https://img.shields.io/badge/TLS-rustls_(no_OpenSSL)-lightgrey?style=flat-square&logo=letsencrypt>)](https://github.com/rustls/rustls)

</div>

---

## ✨ What makes gvm different?

gvm is a Go version manager built from scratch in Rust. It was designed with a single goal: work everywhere, require nothing.

- **No Go required** - you don't need Go installed to install Go. gvm downloads the official toolchain directly from go.dev.
- **No `sudo`, no root** - everything lives under `~/.gvm` in your home directory.
- **Zero system dependencies** - a single static binary is all you need.
- **Truly cross-platform** - one codebase, one behavior across Windows, Linux, and macOS on both x86_64 and ARM64.
- **SHA-256 verified downloads** - every archive is checked against go.dev's official checksum before extraction.
- **Session-scoped activation** - `gvm shell <version>` activates a version for the current terminal only, without touching any files.
- **Self-updating** - `gvm upgrade` downloads and replaces the binary in-place.
- **Clean uninstall** - `gvm implode` removes everything gvm ever touched.

---

## 🚀 Features

- 📥 **Install any Go version** - by exact version, minor range, or `latest`
- 🌍 **Global default** - set a system-wide version with `gvm use`
- 📌 **Per-project pinning** - drop a `.go-version` file; gvm activates it automatically
- 🔐 **SHA-256 verification** - every download is checked against go.dev's official checksum
- 🐚 **Shell integration** - automatic `PATH` and `GOROOT` injection for PowerShell, Bash, Zsh, and Fish
- ⚡ **`gvm exec`** - run a command with any Go version without changing the global default
- 🩺 **`gvm doctor`** - diagnose your setup with actionable hints
- 🔄 **`gvm upgrade`** - self-update to the latest release from GitHub
- 💣 **`gvm implode`** - completely remove gvm and all installed versions cleanly
- 🏁 **Shell completions** - Bash, Zsh, Fish, and PowerShell
- 🖥️ **Cross-platform** - Windows, Linux, macOS × x86_64 and ARM64

---

## 📦 Installation

### 🪟 Windows (PowerShell)

```powershell
irm https://raw.githubusercontent.com/jhonsferg/gvm/main/install/install.ps1 | iex
```

> Installs `gvm.exe` to `~\.local\bin`, adds it to your user `PATH` via the registry, and injects the shell hook into your PowerShell profile.

### 🐧 Linux and 🍎 macOS

```sh
curl -fsSL https://raw.githubusercontent.com/jhonsferg/gvm/main/install/install.sh | sh
```

> Installs `gvm` to `~/.local/bin`, updates your shell profile (`~/.bashrc`, `~/.zshrc`, or `~/.config/fish/config.fish`), and injects the `gvm env` hook.

### 📂 Custom install directory

```powershell
# 🪟 Windows
$env:GVM_INSTALL_DIR = "C:\tools\gvm"; irm .../install.ps1 | iex
```

```sh
# 🐧 Linux / 🍎 macOS
GVM_INSTALL_DIR=~/.bin curl -fsSL .../install.sh | sh
```

### ✅ Verify the installation

```sh
gvm doctor
```

---

## ⚡ Quick Start

```sh
# 📥 Install the latest stable Go release
gvm install latest

# 🌍 Activate it globally
gvm use latest

# 🔍 Check the active version
gvm current

# 📌 Pin a version for the current project
gvm local 1.22

# ⚡ Run tests with a different version, without changing the global default
gvm exec 1.21 go test ./...
```

---

## 📖 Commands

### 📥 `gvm install <version>`

Downloads and installs a Go release from go.dev. The archive is verified against the official SHA-256 checksum before extraction.

```sh
gvm install latest          # 🆕 latest stable release
gvm install 1.22            # 🔢 latest patch of Go 1.22
gvm install 1.22.4          # 🎯 exact version
gvm install 1.22.4 --force  # 🔄 reinstall even if already present
```

---

### 🌍 `gvm use <version>` · `gvm default <version>`

Sets the global default Go version. The version must already be installed.

```sh
gvm use latest
gvm use 1.22
gvm use 1.22.4
```

> 💡 The change takes effect in any new terminal session, or immediately after reloading your profile.

---

### 📌 `gvm local <version>`

Writes a `.go-version` file in the current directory. gvm reads this file on every shell startup and activates the pinned version automatically.

```sh
# In your project root:
gvm local 1.21.9
```

> The file contains a plain version string (`go1.21.9`) and can be committed to version control so every contributor uses the same toolchain.

> ⚠️ If the pinned version is not installed, gvm prints a warning and falls back to the global default.

---

### 🗑️ `gvm uninstall <version>`

Removes an installed Go version from disk.

```sh
gvm uninstall 1.21.9
```

---

### 📋 `gvm list`

Lists all locally installed Go versions. The active version is highlighted.

```
  go1.23.0  (active)
  go1.22.4
  go1.21.9
```

---

### 🌐 `gvm list-remote`

Lists stable Go versions available for download from go.dev.

```sh
gvm list-remote          # 📄 latest patch per minor (compact view)
gvm list-remote --all    # 📜 every patch release
```

Already-installed versions are marked with `✓`.

---

### 🔍 `gvm current`

Prints the active Go version and where it came from.

```
go1.22.4  (local .go-version)
```

or

```
go1.23.0  (global)
```

---

### 📂 `gvm path [version]`

Prints the `bin/` directory of the active (or specified) version. Useful for scripting.

```sh
gvm path              # active version
gvm path 1.21         # specific version
export GOROOT=$(dirname $(gvm path))
```

---

### 🐚 `gvm env [--shell <name>]`

Emits shell commands that set `PATH` and `GOROOT` for the active version. This is what the shell hook calls on every prompt.

```sh
eval "$(gvm env)"               # 🔍 auto-detect shell
gvm env --shell bash
gvm env --shell zsh
gvm env --shell fish
```

```powershell
# 🪟 PowerShell
gvm env --shell powershell | Out-String | Invoke-Expression
```

---

### 🔧 `gvm setup [--shell <name>]`

Injects the `gvm env` hook into your shell profile. The install scripts run this automatically; you only need it manually if you move the binary or change your shell.

```sh
gvm setup               # 🔍 auto-detect shell
gvm setup --shell zsh   # 🎯 explicit target
```

> 🛡️ The `# gvm init` marker prevents duplicate entries - re-running `setup` is always safe.

---

### ⚡ `gvm exec <version> <command> [args…]`

Runs any command with a specific Go version injected into `PATH` and `GOROOT`, **without changing the global default**.

```sh
# 🏗️ Build with Go 1.21 while Go 1.22 is the global default
gvm exec 1.21 go build ./...

# 🧪 Run tests on multiple versions in CI
gvm exec 1.20 go test ./...
gvm exec 1.21 go test ./...
gvm exec 1.22 go test ./...

# 🔍 Check the exact Go binary
gvm exec 1.22.4 go version
```

> The exit code of the subprocess is forwarded to the calling process.

---

### 🩺 `gvm doctor [--shell <name>]`

Checks your gvm installation and reports issues with actionable hints:

- 🔍 `gvm` binary is in `PATH`
- 🌍 A global Go version is set
- 💾 The global version is installed on disk
- 📂 `GOROOT` resolves to a valid directory
- 🐚 The `gvm env` hook is present in the shell profile
- 📌 The local `.go-version` (if any) is installed

```sh
gvm doctor
gvm doctor --shell zsh
```

> Exits with code `1` if any issue is found - perfect for CI health checks.

---

### 🔄 `gvm upgrade [--force]`

Self-updates gvm to the latest release published on GitHub.

```sh
gvm upgrade           # 🔍 check and update if a newer version exists
gvm upgrade --force   # 🔄 reinstall the latest even if already up to date
```

> 🔒 On Unix the replacement is **atomic** (same-filesystem rename). On Windows the old binary is renamed first to free its name, then the new binary takes the original path. A rollback is attempted automatically if the replacement fails.

---

### 💣 `gvm implode [--force]`

**Completely removes gvm** and everything it manages from the system.

```sh
gvm implode           # 🗑️ shows a summary, asks for confirmation
gvm implode --force   # 💥 removes everything immediately, no questions asked
```

What gets removed:

- 📁 The entire `~/.gvm/` data directory (all installed Go versions)
- 🔧 The `gvm` binary itself
- 🐚 Every gvm-managed line from your shell profile

> ⚠️ This operation is **irreversible**. Your installed Go versions will be deleted. Use `gvm upgrade` instead if you just want to update.

---

### 🏁 `gvm completions <shell>`

Prints a shell completion script to stdout.

```sh
# 🐧 Bash
gvm completions bash > ~/.local/share/bash-completion/completions/gvm

# 🐚 Zsh
gvm completions zsh > "${fpath[1]}/_gvm"

# 🐟 Fish
gvm completions fish > ~/.config/fish/completions/gvm.fish

# 🪟 PowerShell
gvm completions powershell >> $PROFILE
```

---

## 🔢 Version Syntax

All commands that accept a version support these forms:

| Input      | Meaning                               |
| ---------- | ------------------------------------- |
| `latest`   | 🆕 Newest stable release              |
| `1.22`     | 🔢 Latest installed patch of Go 1.22  |
| `1.22.4`   | 🎯 Exact version go1.22.4             |
| `go1.22.4` | ✅ Same as `1.22.4` (prefix accepted) |

---

## 📌 Per-project Versions

Place a `.go-version` file in any directory:

```
go1.22.4
```

gvm walks up the directory tree from the current working directory (up to 20 levels) looking for `.go-version`. When found, it takes precedence over the global default.

> 🔗 The file is compatible with other tools such as [goenv](https://github.com/syndbg/goenv) and the VS Code Go extension.

---

## 🐚 Shell Integration

After installation, your shell profile contains a hook line:

| Shell         | Hook                                                            |
| ------------- | --------------------------------------------------------------- |
| 🐧 Bash       | `eval "$(gvm env --shell bash)"`                                |
| 🐚 Zsh        | `eval "$(gvm env --shell zsh)"`                                 |
| 🐟 Fish       | `gvm env --shell fish \| source`                                |
| 🪟 PowerShell | `gvm env --shell powershell \| Out-String \| Invoke-Expression` |

On every new shell session the hook:

1. 🔍 Reads the active version (`.go-version` → global default)
2. ➕ Prepends the version's `bin/` directory to `PATH`
3. 📂 Sets `GOROOT` to the version's root directory

> 🔇 No daemons, no background processes, no side effects.

---

## ⚙️ Configuration

| Variable  | Default  | Description                        |
| --------- | -------- | ---------------------------------- |
| `GVM_DIR` | `~/.gvm` | 📁 Root directory for all gvm data |

### 📂 Directory layout

```
~/.gvm/
├── version          # 🌍 active global version (plain text)
├── versions/
│   ├── go1.22.4/    # 📦 extracted Go toolchain
│   │   ├── bin/
│   │   ├── src/
│   │   └── …
│   └── go1.23.0/
└── tmp/             # ⏳ download staging area (cleaned after install)
```

---

## 🛠️ Building from Source

Requires [Rust](https://rustup.rs) 1.75 or newer. No system dependencies - TLS is handled by [rustls](https://github.com/rustls/rustls) (pure Rust, no OpenSSL needed).

```sh
git clone https://github.com/jhonsferg/gvm.git
cd gvm
cargo build --release
```

The binary is placed at `target/release/gvm` (or `gvm.exe` on Windows).

```sh
# ✅ Run the self-check after building
./target/release/gvm doctor
```

---

## 📦 Release Artifacts

Releases are automated via GitHub Actions. Pushing a version tag triggers cross-compilation for all supported targets:

| Artifact                 | Target                       | Notes            |
| ------------------------ | ---------------------------- | ---------------- |
| `gvm-windows-x86_64.exe` | `x86_64-pc-windows-msvc`     |                  |
| `gvm-linux-x86_64`       | `x86_64-unknown-linux-musl`  | ⚡ static binary |
| `gvm-linux-aarch64`      | `aarch64-unknown-linux-musl` | ⚡ static binary |
| `gvm-darwin-x86_64`      | `x86_64-apple-darwin`        |                  |
| `gvm-darwin-aarch64`     | `aarch64-apple-darwin`       | 🍎 Apple Silicon |

Each release also includes `checksums.txt` with SHA-256 hashes for all artifacts.

```sh
# 🏷️ Publish a new release
git tag v1.0.0
git push origin v1.0.0
```

---

## 📄 License

MIT - see [LICENSE](LICENSE).

---

<div align="center">

Made with 🦀 Rust · Maintained with ❤️

</div>
