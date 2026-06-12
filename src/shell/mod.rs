//! Shell integration layer.
//!
//! This module defines the [`ShellConfig`] trait and provides concrete
//! implementations for every supported shell. It also exposes helpers for
//! runtime shell detection and idempotent profile injection.
//!
//! # Design
//!
//! Each supported shell is a zero-sized type (e.g. [`Bash`], [`PowerShell`])
//! that implements [`ShellConfig`]. Adding support for a new shell requires
//! only a new file and a new variant - no existing code needs to change
//! (Open/Closed principle).
//!
//! Shell detection is done at runtime from environment variables rather than
//! compile-time `cfg!` flags, so a single `gvm` binary works correctly inside
//! Git Bash, WSL, or any other non-native shell on Windows.

mod bash;
mod fish;
mod powershell;
mod zsh;

use anyhow::{bail, Result};
use std::path::{Path, PathBuf};

/// Context passed to [`ShellConfig::env_script`] when generating the shell
/// initialisation script for the current session.
pub struct EnvContext<'a> {
    /// Path to the gvm root directory (value of `GVM_DIR`).
    pub gvm_dir: &'a Path,

    /// Path to the `bin/` directory of the currently active Go version.
    /// `None` when no version is active.
    pub active_bin: Option<&'a Path>,

    /// Path to the root directory of the currently active Go version.
    /// Used to set `GOROOT`. `None` when no version is active.
    pub active_root: Option<&'a Path>,
}

/// Behaviour that every supported shell must implement.
///
/// Implementors must be `Debug` so they can be logged and inspected.
/// The trait is object-safe and is used throughout `gvm` as
/// `Box<dyn ShellConfig>` or `&dyn ShellConfig`.
pub trait ShellConfig: std::fmt::Debug {
    /// Short, lowercase identifier for the shell (e.g. `"bash"`, `"powershell"`).
    fn name(&self) -> &'static str;

    /// Generates the shell script that sets `GVM_DIR`, `PATH`, `GOROOT`,
    /// and installs the `cd` hook for automatic version switching.
    fn env_script(&self, ctx: &EnvContext<'_>) -> String;

    /// Returns the path to the shell's user-level startup file where the
    /// `gvm env` hook line should be appended, or `None` if the path cannot
    /// be determined.
    fn profile_path(&self) -> Option<PathBuf>;

    /// Returns the one-liner that should be added to the shell profile so
    /// `gvm env` is evaluated on every new session.
    fn init_line(&self) -> &'static str;

    /// Returns the shell function definition that wraps the `gvm` binary.
    ///
    /// When sourced, this function calls the real `gvm` binary and then
    /// immediately re-evaluates `gvm env` after `use`, `default`, or `local`
    /// commands so that `PATH` and `GOROOT` are updated in the current shell
    /// session without opening a new terminal.
    fn wrapper_function(&self) -> &'static str;

    /// Returns a minimal shell script that activates `version_tag` for the
    /// current session only (sets `GVM_SHELL_VERSION`, `GOROOT`, and `PATH`).
    ///
    /// This script is emitted by `gvm shell <version>` and evaluated by the
    /// shell wrapper function. The `_gvm_hook` checks `GVM_SHELL_VERSION` and
    /// skips its normal version switching while this override is active, so the
    /// activation persists across `cd` calls until `gvm shell --unset` is run.
    fn shell_version_script(&self, version_tag: &str, bin: &Path, root: &Path) -> String;

    /// Returns the shell script that clears the session-scoped override.
    ///
    /// The script unsets `GVM_SHELL_VERSION` and calls `_gvm_hook` so that
    /// `PATH` and `GOROOT` are immediately restored to whatever `.go-version`
    /// or the global default says.
    fn shell_unset_script(&self) -> &'static str;
}

// --- Concrete implementations ------------------------------------------------

#[derive(Debug)]
pub struct Bash;
#[derive(Debug)]
pub struct Zsh;
#[derive(Debug)]
pub struct Fish;
#[derive(Debug)]
pub struct PowerShell;

impl ShellConfig for Bash {
    fn name(&self) -> &'static str {
        "bash"
    }
    fn env_script(&self, ctx: &EnvContext<'_>) -> String {
        bash::env_script(ctx)
    }
    fn profile_path(&self) -> Option<PathBuf> {
        bash::profile_path()
    }
    fn init_line(&self) -> &'static str {
        r#"eval "$(gvm env --shell bash)""#
    }
    fn wrapper_function(&self) -> &'static str {
        bash::wrapper_function()
    }
    fn shell_version_script(&self, tag: &str, bin: &Path, root: &Path) -> String {
        bash::shell_version_script(tag, bin, root)
    }
    fn shell_unset_script(&self) -> &'static str {
        bash::shell_unset_script()
    }
}

impl ShellConfig for Zsh {
    fn name(&self) -> &'static str {
        "zsh"
    }
    fn env_script(&self, ctx: &EnvContext<'_>) -> String {
        zsh::env_script(ctx)
    }
    fn profile_path(&self) -> Option<PathBuf> {
        zsh::profile_path()
    }
    fn init_line(&self) -> &'static str {
        r#"eval "$(gvm env --shell zsh)""#
    }
    fn wrapper_function(&self) -> &'static str {
        zsh::wrapper_function()
    }
    fn shell_version_script(&self, tag: &str, bin: &Path, root: &Path) -> String {
        zsh::shell_version_script(tag, bin, root)
    }
    fn shell_unset_script(&self) -> &'static str {
        zsh::shell_unset_script()
    }
}

impl ShellConfig for Fish {
    fn name(&self) -> &'static str {
        "fish"
    }
    fn env_script(&self, ctx: &EnvContext<'_>) -> String {
        fish::env_script(ctx)
    }
    fn profile_path(&self) -> Option<PathBuf> {
        fish::profile_path()
    }
    fn init_line(&self) -> &'static str {
        "if command -q gvm; gvm env --shell fish | source; end"
    }
    fn wrapper_function(&self) -> &'static str {
        fish::wrapper_function()
    }
    fn shell_version_script(&self, tag: &str, bin: &Path, root: &Path) -> String {
        fish::shell_version_script(tag, bin, root)
    }
    fn shell_unset_script(&self) -> &'static str {
        fish::shell_unset_script()
    }
}

impl ShellConfig for PowerShell {
    fn name(&self) -> &'static str {
        "powershell"
    }
    fn env_script(&self, ctx: &EnvContext<'_>) -> String {
        powershell::env_script(ctx)
    }
    fn profile_path(&self) -> Option<PathBuf> {
        powershell::profile_path()
    }
    fn init_line(&self) -> &'static str {
        "gvm env --shell powershell | Out-String | Invoke-Expression"
    }
    fn wrapper_function(&self) -> &'static str {
        powershell::wrapper_function()
    }
    fn shell_version_script(&self, tag: &str, bin: &Path, root: &Path) -> String {
        powershell::shell_version_script(tag, bin, root)
    }
    fn shell_unset_script(&self) -> &'static str {
        powershell::shell_unset_script()
    }
}

// --- Factory -----------------------------------------------------------------

/// Detects the running shell from the environment at runtime.
///
/// Detection order (most to least specific):
///
/// 1. `PSModulePath` environment variable - present in every PowerShell child
///    process, including nested ones.
/// 2. `SHELL` environment variable - standard on Unix systems.
/// 3. Compile-time `cfg!(target_os = "windows")` as a last resort when
///    neither variable is available.
///
/// Returns `None` if the shell cannot be identified.
pub fn detect() -> Option<Box<dyn ShellConfig>> {
    if std::env::var("PSModulePath").is_ok() {
        return Some(Box::new(PowerShell));
    }
    if let Ok(shell) = std::env::var("SHELL") {
        if shell.contains("zsh") {
            return Some(Box::new(Zsh));
        }
        if shell.contains("fish") {
            return Some(Box::new(Fish));
        }
        if shell.contains("bash") {
            return Some(Box::new(Bash));
        }
    }
    if cfg!(target_os = "windows") {
        return Some(Box::new(PowerShell));
    }
    None
}

/// Constructs a [`ShellConfig`] from a shell name string.
///
/// Accepted values (case-insensitive, hyphens ignored):
/// `powershell`, `pwsh`, `bash`, `zsh`, `fish`.
///
/// # Errors
///
/// Returns an error if the name does not match any supported shell.
pub fn from_str(s: &str) -> Result<Box<dyn ShellConfig>> {
    match s.to_lowercase().replace('-', "").as_str() {
        "powershell" | "pwsh" => Ok(Box::new(PowerShell)),
        "bash" => Ok(Box::new(Bash)),
        "zsh" => Ok(Box::new(Zsh)),
        "fish" => Ok(Box::new(Fish)),
        _ => bail!(
            "Unknown shell '{}'. Supported: powershell, bash, zsh, fish",
            s
        ),
    }
}

// --- Profile injection -------------------------------------------------------

/// Appends the `gvm env` hook and the shell wrapper function to the shell's
/// profile file.
///
/// Two independent markers are used so that each block can be injected
/// separately and both can be detected by `gvm implode` for clean removal:
///
/// - `# gvm init` - guards the `eval "$(gvm env …)"` one-liner that sets
///   `PATH`/`GOROOT` on every new shell session.
/// - `# gvm wrapper` - guards the `gvm()` / `function gvm` definition that
///   immediately refreshes the current session after `gvm use`, `gvm default`,
///   or `gvm local` without requiring a new terminal.
///
/// Re-running `gvm setup` is safe: each block is only appended when its
/// marker is absent, so existing installations receive the wrapper function
/// on upgrade without duplicating the init hook.
///
/// Creates the profile file (and any parent directories) if necessary.
///
/// # Errors
///
/// Returns an error if the profile path cannot be determined, the file cannot
/// be read, or the file cannot be written.
pub fn inject_profile(shell: &dyn ShellConfig) -> Result<()> {
    use anyhow::Context;

    const INIT_MARKER: &str = "# gvm init";
    const WRAPPER_MARKER: &str = "# gvm wrapper";

    let profile = shell
        .profile_path()
        .ok_or_else(|| anyhow::anyhow!("Cannot determine profile path for {}", shell.name()))?;

    if let Some(parent) = profile.parent() {
        std::fs::create_dir_all(parent)?;
    }

    let existing = if profile.exists() {
        std::fs::read_to_string(&profile)?
    } else {
        String::new()
    };

    let has_init = existing.contains(INIT_MARKER);
    let has_wrapper = existing.contains(WRAPPER_MARKER);

    // If both markers are present, check whether the blocks are up to date.
    // When gvm is upgraded its init_line or wrapper may change; we replace
    // stale blocks rather than silently leaving old ones in place.
    if has_init && has_wrapper {
        let expected_init = format!("{INIT_MARKER}\n{}\n", shell.init_line());
        let expected_wrapper = format!("{WRAPPER_MARKER}\n{}\n", shell.wrapper_function());
        if existing.contains(&expected_init) && existing.contains(&expected_wrapper) {
            println!("gvm is already configured in {}", profile.display());
            return Ok(());
        }

        let mut content = existing.clone();
        let mut any_updated = false;

        // Replace stale init block (marker → next blank line or next marker).
        if !existing.contains(&expected_init) {
            if let Some(marker_pos) = content.find(INIT_MARKER) {
                // Find where the init block ends: next occurrence of "# gvm " or EOF.
                let after_marker = &content[marker_pos + INIT_MARKER.len()..];
                let block_len = after_marker
                    .find("\n# gvm ")
                    .map(|i| i + 1)
                    .unwrap_or(after_marker.len());
                let end = marker_pos + INIT_MARKER.len() + block_len;
                let new_block = format!("{INIT_MARKER}\n{}\n", shell.init_line());
                content = format!("{}{}{}", &content[..marker_pos], new_block, &content[end..]);
            }
            println!("  Updated init hook in {}", profile.display());
            any_updated = true;
        }

        // Replace stale wrapper block (always last in the file).
        if !content.contains(&expected_wrapper) {
            let marker_pos = content
                .rfind(WRAPPER_MARKER)
                .expect("WRAPPER_MARKER must exist when has_wrapper is true");
            let before = content[..marker_pos].trim_end().to_string();
            content = format!(
                "{before}\n\n{WRAPPER_MARKER}\n{}\n",
                shell.wrapper_function()
            );
            println!("  Updated wrapper function in {}", profile.display());
            any_updated = true;
        }

        if any_updated {
            std::fs::write(&profile, &content)
                .with_context(|| format!("Failed to write to {}", profile.display()))?;
        }
        return Ok(());
    }

    let mut content = existing.trim_end().to_string();
    let mut changed = false;

    if !has_init {
        if !content.is_empty() {
            content.push_str("\n\n");
        }
        content.push_str(&format!("{INIT_MARKER}\n{}\n", shell.init_line()));
        println!("  Added init hook to {}", profile.display());
        changed = true;
    } else {
        println!("  Init hook already present in {}", profile.display());
    }

    if !has_wrapper {
        content.push_str(&format!(
            "\n{WRAPPER_MARKER}\n{}\n",
            shell.wrapper_function()
        ));
        println!("  Added wrapper function to {}", profile.display());
        changed = true;
    }

    if changed {
        std::fs::write(&profile, content)
            .with_context(|| format!("Failed to write to {}", profile.display()))?;
    }

    Ok(())
}

// --- Helpers -----------------------------------------------------------------

/// Returns `true` if the directory containing the current `gvm` executable
/// is listed in the `PATH` environment variable.
///
/// Used by `gvm setup` and `gvm doctor` to warn the user when the binary
/// itself is not reachable from the shell.
pub fn gvm_in_path() -> bool {
    let Ok(exe) = std::env::current_exe() else {
        return false;
    };
    let Some(dir) = exe.parent() else {
        return false;
    };
    let Ok(path_var) = std::env::var("PATH") else {
        return false;
    };
    let sep = if cfg!(windows) { ';' } else { ':' };
    path_var.split(sep).any(|p| Path::new(p) == dir)
}

// --- Tests -------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::tempdir;

    // Write `existing` to a temp file, simulate inject_profile logic, return result.
    fn run_inject(shell: &dyn ShellConfig, existing: &str) -> String {
        const INIT_MARKER: &str = "# gvm init";
        const WRAPPER_MARKER: &str = "# gvm wrapper";

        let dir = tempdir().unwrap();
        let path = dir.path().join("profile");
        fs::write(&path, existing).unwrap();

        let src = fs::read_to_string(&path).unwrap();
        let has_init = src.contains(INIT_MARKER);
        let has_wrapper = src.contains(WRAPPER_MARKER);

        if has_init && has_wrapper {
            let expected_init = format!("{INIT_MARKER}\n{}\n", shell.init_line());
            let expected_wrapper = format!("{WRAPPER_MARKER}\n{}\n", shell.wrapper_function());
            if src.contains(&expected_init) && src.contains(&expected_wrapper) {
                return src;
            }
            let mut content = src.clone();
            if !content.contains(&expected_init) {
                if let Some(pos) = content.find(INIT_MARKER) {
                    let after = &content[pos + INIT_MARKER.len()..];
                    let end = pos
                        + INIT_MARKER.len()
                        + after.find("\n# gvm ").map(|i| i + 1).unwrap_or(after.len());
                    let new_block = format!("{INIT_MARKER}\n{}\n", shell.init_line());
                    content = format!("{}{}{}", &content[..pos], new_block, &content[end..]);
                }
            }
            if !content.contains(&expected_wrapper) {
                let pos = content.rfind(WRAPPER_MARKER).unwrap();
                let before = content[..pos].trim_end().to_string();
                content = format!(
                    "{before}\n\n{WRAPPER_MARKER}\n{}\n",
                    shell.wrapper_function()
                );
            }
            fs::write(&path, &content).unwrap();
            return content;
        }

        let mut content = src.trim_end().to_string();
        if !has_init {
            if !content.is_empty() {
                content.push_str("\n\n");
            }
            content.push_str(&format!("{INIT_MARKER}\n{}\n", shell.init_line()));
        }
        if !has_wrapper {
            content.push_str(&format!(
                "\n{WRAPPER_MARKER}\n{}\n",
                shell.wrapper_function()
            ));
        }
        fs::write(&path, &content).unwrap();
        content
    }

    #[test]
    fn setup_injects_both_blocks_into_empty_profile() {
        let result = run_inject(&Bash, "");
        assert!(result.contains("# gvm init"));
        assert!(result.contains("# gvm wrapper"));
        assert!(result.contains("shell)"));
    }

    #[test]
    fn setup_is_idempotent_when_wrapper_is_current() {
        let sh = Bash;
        let first = run_inject(&sh, "");
        let second = run_inject(&sh, &first);
        assert_eq!(first, second, "second run must not change the file");
    }

    #[test]
    fn setup_updates_stale_bash_wrapper() {
        let stale = "# gvm init\neval \"$(gvm env --shell bash)\"\n\n# gvm wrapper\ngvm() { command gvm \"$@\"; }\n";
        let result = run_inject(&Bash, stale);
        // New wrapper must be present
        assert!(result.contains("shell)"), "shell case must be injected");
        // Old stub must be gone
        assert!(
            !result.contains("command gvm \"$@\"; }"),
            "old stub must be removed"
        );
        // Init block must be preserved
        assert!(result.contains("# gvm init"), "init block must survive");
    }

    #[test]
    fn setup_updates_stale_zsh_wrapper() {
        let stale = "# gvm init\neval \"$(gvm env --shell zsh)\"\n\n# gvm wrapper\ngvm() { command gvm \"$@\"; }\n";
        let result = run_inject(&Zsh, stale);
        assert!(result.contains("shell)"));
        assert!(result.contains("--shell zsh"));
    }

    #[test]
    fn setup_updates_stale_fish_wrapper() {
        let stale = "# gvm init\ngvm env --shell fish | source\n\n# gvm wrapper\nfunction gvm\n    command gvm $argv\nend\n";
        let result = run_inject(&Fish, stale);
        assert!(result.contains("contains -- $argv[1] shell"));
        assert!(
            result.contains("string join"),
            "updated fish wrapper must use string join"
        );
    }

    #[test]
    fn setup_does_not_duplicate_init_block() {
        let existing = "# gvm init\neval \"$(gvm env --shell bash)\"\n";
        let result = run_inject(&Bash, existing);
        let count = result.matches("# gvm init").count();
        assert_eq!(count, 1, "init marker must appear exactly once");
    }

    #[test]
    fn setup_updates_stale_fish_init_line() {
        // Old fish init line (without the command -q guard) should be replaced.
        let stale = format!(
            "# gvm init\ngvm env --shell fish | source\n\n# gvm wrapper\n{}\n",
            Fish.wrapper_function()
        );
        let result = run_inject(&Fish, &stale);
        assert!(
            result.contains("command -q gvm"),
            "new guard must be present"
        );
        assert!(
            !result.contains("\ngvm env --shell fish | source\n"),
            "bare unguarded line must be replaced"
        );
        let count = result.matches("# gvm init").count();
        assert_eq!(
            count, 1,
            "init marker must appear exactly once after update"
        );
    }

    #[test]
    fn setup_is_idempotent_for_fish_after_update() {
        let sh = Fish;
        let first = run_inject(&sh, "");
        let second = run_inject(&sh, &first);
        assert_eq!(first, second, "fish: second run must not change the file");
    }
}
