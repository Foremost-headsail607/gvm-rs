//! Archive extraction for `.tar.gz` and `.zip` files.
//!
//! Go releases use `.tar.gz` on Linux and macOS and `.zip` on Windows.
//! [`unpack`] dispatches to the appropriate implementation based on the file
//! extension and displays a spinner while extraction is in progress.

use anyhow::{bail, Context, Result};
use indicatif::{ProgressBar, ProgressStyle};
use std::path::Path;

/// Extracts `archive` into the `dest` directory.
///
/// The archive format is determined from the file extension:
/// - `.tar.gz` - extracted with `flate2` + `tar`.
/// - `.zip` - extracted with the `zip` crate.
///
/// A spinner is shown for the duration of the extraction and cleared
/// afterwards, regardless of whether extraction succeeds or fails.
///
/// # Errors
///
/// Returns an error if:
/// - The archive file extension is not `.tar.gz` or `.zip`.
/// - The archive cannot be opened or is malformed.
/// - Any entry cannot be written to `dest`.
pub fn unpack(archive: &Path, dest: &Path) -> Result<()> {
    let name = archive
        .file_name()
        .and_then(|n| n.to_str())
        .unwrap_or_default();

    let pb = ProgressBar::new_spinner();
    pb.set_style(
        ProgressStyle::default_spinner()
            .template("  {spinner:.cyan} {msg}")
            .unwrap(),
    );
    pb.set_message(format!("Extracting {name}..."));
    pb.enable_steady_tick(std::time::Duration::from_millis(80));

    let result = if name.ends_with(".tar.gz") {
        unpack_tar_gz(archive, dest)
    } else if name.ends_with(".zip") {
        unpack_zip(archive, dest)
    } else {
        bail!("Unsupported archive format: {name}")
    };

    pb.finish_and_clear();
    result
}

/// Extracts a `.tar.gz` archive into `dest`.
///
/// The gzip stream is decoded on the fly; no temporary uncompressed file is
/// written to disk.
///
/// # Errors
///
/// Returns an error if the file cannot be opened or the archive is malformed.
fn unpack_tar_gz(archive: &Path, dest: &Path) -> Result<()> {
    let file = std::fs::File::open(archive)?;
    let gz = flate2::read::GzDecoder::new(file);
    tar::Archive::new(gz)
        .unpack(dest)
        .context("Failed to extract tar.gz")
}

/// Extracts a `.zip` archive into `dest`.
///
/// # Errors
///
/// Returns an error if the file cannot be opened, the central directory
/// cannot be read, or any entry cannot be extracted.
fn unpack_zip(archive: &Path, dest: &Path) -> Result<()> {
    let file = std::fs::File::open(archive)?;
    zip::ZipArchive::new(file)
        .context("Failed to read zip archive")?
        .extract(dest)
        .context("Failed to extract zip")
}
