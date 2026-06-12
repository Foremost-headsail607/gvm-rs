//! `gvm list` - show all locally installed Go versions.
//!
//! Installed versions are listed newest-first. The currently active version
//! is marked so the user can identify it at a glance.

use anyhow::Result;
use colored::Colorize;

use crate::{config::Config, toolchain};

/// Prints all installed Go versions, sorted newest-first.
///
/// The active version (determined from `.go-version` or the global default)
/// is highlighted with a check mark and an `(active)` label. If no version is
/// active - for example because no global default has been set yet - all
/// versions are shown without highlighting.
///
/// # Errors
///
/// Returns an error if the versions directory cannot be read.
pub fn run(config: &Config) -> Result<()> {
    let installed = toolchain::list_installed(config)?;

    if installed.is_empty() {
        println!("No Go versions installed. Run 'gvm install latest'.");
        return Ok(());
    }

    let active = toolchain::active_version(config).map(|(v, _)| v).ok();

    println!("Installed Go versions:");
    for v in &installed {
        if active.as_ref() == Some(v) {
            println!(
                "  {} {}  {}",
                "✓".green(),
                v.tag().bold(),
                "(active)".dimmed()
            );
        } else {
            println!("    {}", v.tag());
        }
    }
    Ok(())
}
