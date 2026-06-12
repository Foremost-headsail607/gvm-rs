//! `gvm exec` - run a command with a specific Go version.
//!
//! Resolves the requested version, prepends its `bin/` directory to `PATH`,
//! sets `GOROOT`, and spawns the command as a child process. The global
//! default version is not modified.
//!
//! The exit code of the child process is forwarded to the parent so that
//! `gvm exec` is transparent to callers that inspect `$?` / `$LASTEXITCODE`.

use anyhow::{bail, Result};
use colored::Colorize;

use crate::{config::Config, toolchain, user_version::VersionSpec};

/// Runs `args[0]` with the Go version described by `spec_str` injected into
/// the environment, without changing the global default.
///
/// The version's `bin/` directory is prepended to a copy of the current
/// `PATH`, and `GOROOT` is set to the version's root directory. Both
/// modifications are scoped to the child process only.
///
/// The function does not return on success: it calls
/// [`std::process::exit`] with the child's exit code so the status is
/// forwarded correctly.
///
/// # Errors
///
/// Returns an error if:
/// - `args` is empty.
/// - `spec_str` is not a valid version spec.
/// - No installed version matches the spec.
/// - The command cannot be spawned (e.g. not found on `PATH`).
pub fn run(config: &Config, spec_str: &str, args: &[String]) -> Result<()> {
    if args.is_empty() {
        bail!("No command specified. Usage: gvm exec <version> <command> [args...]");
    }

    let spec = VersionSpec::parse(spec_str)?;
    let version = toolchain::resolve_installed(config, &spec)?;
    let bin = toolchain::version_bin_path(config, &version)?;
    let root = config.version_dir(&version.tag());

    let cmd_name = &args[0];
    let cmd_args = &args[1..];

    // Build a modified PATH that has the target version's bin/ first, with all
    // previously gvm-managed entries left in place (they come after anyway).
    let path_var = std::env::var("PATH").unwrap_or_default();
    let sep = if cfg!(windows) { ";" } else { ":" };
    let new_path = format!("{}{sep}{path_var}", bin.display());

    println!("{} Running with Go {}", "->".cyan(), version.tag().dimmed());

    let status = std::process::Command::new(cmd_name)
        .args(cmd_args)
        .env("GOROOT", &root)
        .env("PATH", &new_path)
        .status()
        .map_err(|e| anyhow::anyhow!("Failed to run '{}': {e}", cmd_name))?;

    std::process::exit(status.code().unwrap_or(1));
}
