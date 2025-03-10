#[cfg(feature = "file_watcher")]
mod file_watcher;

#[cfg(feature = "multi_threaded")]
mod file_asset;
#[cfg(not(feature = "multi_threaded"))]
mod sync_file_asset;

#[cfg(feature = "file_watcher")]
pub use file_watcher::*;
use tracing::{debug, error};

use alloc::borrow::ToOwned;
use std::{
    env,
    path::{Path, PathBuf},
};

pub(crate) fn get_base_path() -> PathBuf {
    if let Ok(manifest_dir) = env::var("BEVY_ASSET_ROOT") {
        PathBuf::from(manifest_dir)
    } else if let Ok(manifest_dir) = env::var("CARGO_MANIFEST_DIR") {
        PathBuf::from(manifest_dir)
    } else {
        env::current_exe()
            .map(|path| path.parent().map(ToOwned::to_owned).unwrap())
            .unwrap()
    }
}

/// I/O implementation for the local filesystem.
///
/// This asset I/O is fully featured but it's not available on `android` and `wasm` targets.
pub struct FileAssetReader {
    root_path: PathBuf,
}

impl FileAssetReader {
    /// Creates a new `FileAssetIo` at a path relative to the executable's directory, optionally
    /// watching for changes.
    ///
    /// See `get_base_path` below.
    pub fn new<P: AsRef<Path>>(path: P) -> Self {
        let root_path = Self::get_base_path().join(path.as_ref());
        debug!(
            "Asset Server using {} as its base path.",
            root_path.display()
        );
        Self { root_path }
    }

    /// Returns the base path of the assets directory, which is normally the executable's parent
    /// directory.
    ///
    /// To change this, set [`AssetPlugin.file_path`].
    pub fn get_base_path() -> PathBuf {
        get_base_path()
    }

    /// Returns the root directory where assets are loaded from.
    ///
    /// See `get_base_path`.
    pub fn root_path(&self) -> &PathBuf {
        &self.root_path
    }
}

/// A writer for the local filesystem.
pub struct FileAssetWriter {
    root_path: PathBuf,
}

impl FileAssetWriter {
    /// Creates a new [`FileAssetWriter`] at a path relative to the executable's directory, optionally
    /// watching for changes.
    pub fn new<P: AsRef<Path> + core::fmt::Debug>(path: P, create_root: bool) -> Self {
        let root_path = get_base_path().join(path.as_ref());
        if create_root {
            if let Err(e) = std::fs::create_dir_all(&root_path) {
                error!(
                    "Failed to create root directory {} for file asset writer: {}",
                    root_path.display(),
                    e
                );
            }
        }
        Self { root_path }
    }
}
