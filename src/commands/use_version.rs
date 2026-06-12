//! `gvm use` - set the global default Go version.
//!
//! Resolves the supplied spec against the local toolchain store, writes the
//! version tag to `~/.gvm/version`, and prints the shell-appropriate command
//! the user should run to activate the change in their current session.

use anyhow::Result;
use colored::Colorize;

use crate::{config::Config, shell, toolchain, user_version::VersionSpec};

/// Sets the global default Go version to the version described by `spec_str`.
///
/// The version must already be installed. The hint printed after a successful
/// switch is tailored to the detected shell so the user can copy-paste it
/// directly.
///
/// # Errors
///
/// Returns an error if `spec_str` is not a valid version spec or if no
/// installed version matches the spec.
pub fn run(config: &Config, spec_str: &str) -> Result<()> {
    let spec = VersionSpec::parse(spec_str)?;
    let version = toolchain::resolve_installed(config, &spec)?;

    toolchain::set_global_version(config, &version)?;

    println!(
        "{} Now using Go {} (global).",
        "✓".green(),
        version.tag().bold()
    );

    // When the gvm wrapper function is active (injected by `gvm setup`) the
    // shell refreshes automatically. Print a fallback hint only for sessions
    // where the wrapper is not loaded (e.g. scripts, CI, or before setup).
    let hint = match shell::detect() {
        Some(sh) if sh.name() == "powershell" => {
            format!(
                "gvm env --shell powershell | Out-String | {}",
                "Invoke-Expression".cyan()
            )
        }
        Some(sh) => format!("eval \"$(gvm env --shell {})\"", sh.name().cyan()),
        None => "eval \"$(gvm env)\"".to_string(),
    };
    println!("  Active immediately in this session (or run: {hint})");
    Ok(())
}
