//! `gvm setup` - inject the shell hook into the user's profile.
//!
//! Appends the appropriate `gvm env` init line to the detected (or specified)
//! shell's startup file. The operation is idempotent: if the `# gvm init`
//! marker is already present in the profile, the file is left unchanged.
//!
//! The install scripts call this automatically, so most users will never need
//! to run `gvm setup` manually.

use anyhow::Result;
use colored::Colorize;

use crate::shell::{self, inject_profile};

/// Injects the `gvm env` hook into the current shell's profile.
///
/// The shell is determined from `shell_str` when provided; otherwise it is
/// auto-detected via [`shell::detect`]. After injecting the hook, a warning
/// is printed if the `gvm` binary itself is not on `PATH` yet, since the
/// hook cannot work unless `gvm path` is callable from the shell.
///
/// # Errors
///
/// Returns an error if the shell cannot be detected (when `shell_str` is
/// `None`), the shell name is unknown, or the profile file cannot be written.
pub fn run(shell_str: Option<&str>) -> Result<()> {
    let sh = match shell_str {
        Some(s) => shell::from_str(s)?,
        None => shell::detect()
            .ok_or_else(|| anyhow::anyhow!("Could not detect shell. Use --shell <name>."))?,
    };

    println!("Setting up gvm for {}...", sh.name().bold());
    inject_profile(sh.as_ref())?;

    // A chicken-and-egg situation on first install: the hook calls `gvm path`,
    // but that only works if the gvm binary is already on PATH.
    if !shell::gvm_in_path() {
        if let Ok(exe) = std::env::current_exe() {
            let dir = exe
                .parent()
                .map(|p| p.display().to_string())
                .unwrap_or_default();
            println!("\n{} gvm is not in PATH yet.", "!".yellow());
            println!(
                "  Add {} to your PATH so the shell hook can call 'gvm path'.",
                dir.cyan()
            );
        }
    }

    println!(
        "\n{} Restart your shell or run: {}",
        "✓".green(),
        sh.init_line().cyan()
    );
    Ok(())
}
