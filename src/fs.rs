//! File-system utilities.
//!
//! Supplements the standard library with helpers needed for cross-platform
//! toolchain management. The primary concern is safe directory moves across
//! different drives or mount points, which `std::fs::rename` does not support.

use anyhow::{Context, Result};
use std::path::Path;

/// Moves the directory at `src` to `dst`.
///
/// Attempts an atomic rename first. If the rename fails - which happens when
/// `src` and `dst` reside on different file-system volumes (common on Windows
/// when the temp directory is on a different drive than `~/.gvm`) - it falls
/// back to a recursive copy followed by removal of the source.
///
/// # Errors
///
/// Returns an error if:
/// - Both the rename and the copy-then-delete fallback fail.
/// - Any file in `src` cannot be copied to `dst`.
/// - `src` cannot be removed after a successful copy.
pub fn move_dir(src: &Path, dst: &Path) -> Result<()> {
    match std::fs::rename(src, dst) {
        Ok(()) => Ok(()),
        Err(_) => {
            // Cross-device move: copy every entry then delete the source tree.
            copy_dir_all(src, dst).with_context(|| {
                format!("Failed to copy {} to {}", src.display(), dst.display())
            })?;
            std::fs::remove_dir_all(src)
                .with_context(|| format!("Failed to remove {}", src.display()))
        }
    }
}

/// Recursively copies the directory tree rooted at `src` into `dst`.
///
/// `dst` is created if it does not exist. Files are copied individually;
/// symbolic links are not followed - they are copied as regular files
/// pointing to the same content.
///
/// # Errors
///
/// Returns an error if any entry cannot be read, created, or copied.
fn copy_dir_all(src: &Path, dst: &Path) -> Result<()> {
    std::fs::create_dir_all(dst)?;
    for entry in std::fs::read_dir(src)? {
        let entry = entry?;
        let dst_path = dst.join(entry.file_name());
        if entry.file_type()?.is_dir() {
            copy_dir_all(&entry.path(), &dst_path)?;
        } else {
            std::fs::copy(entry.path(), &dst_path)?;
        }
    }
    Ok(())
}
