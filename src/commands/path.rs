//! `gvm path` - print the `bin/` directory of a Go version.
//!
//! The output is a plain, undecorated path designed to be captured by shell
//! hooks and scripts:
//!
//! ```sh
//! export PATH="$(gvm path):$PATH"
//! export GOROOT="$(dirname "$(gvm path)")"
//! ```

use anyhow::Result;

use crate::{config::Config, toolchain, user_version::VersionSpec};

/// Prints the `bin/` directory path for the active or specified Go version.
///
/// When `spec_str` is `None`, the active version is resolved via
/// [`toolchain::active_version`] (`.go-version` lookup followed by global
/// default). When a spec is provided it must refer to an installed version.
///
/// Output is a single line containing only the path, with no decorations, so
/// it can be used directly in shell command substitution.
///
/// # Errors
///
/// Returns an error if the spec is invalid, no matching version is installed,
/// or no active version can be determined.
pub fn run(config: &Config, spec_str: Option<&str>) -> Result<()> {
    let version = match spec_str {
        Some(s) => toolchain::resolve_installed(config, &VersionSpec::parse(s)?)?,
        None => toolchain::active_version(config)?.0,
    };

    let bin = toolchain::version_bin_path(config, &version)?;
    println!("{}", bin.display());
    Ok(())
}
