use std::fs;
use std::path::{Path, PathBuf};

use super::loader::{
    ShellResourceLoadResult,
    ShellResourceLoader,
};
use super::manifest::ShellResourceManifest;

#[derive(Debug, Default)]
pub struct ShellDiscoveryResult {
    pub discovered: Vec<ShellResourceManifest>,
    pub skipped: Vec<String>,
    pub errors: Vec<String>,
}

pub struct ShellDiscovery {
    loader: ShellResourceLoader,
}

impl ShellDiscovery {
    pub fn new() -> Self {
        Self {
            loader: ShellResourceLoader::new(),
        }
    }

    pub fn discover(
        &self,
        root: impl AsRef<Path>,
    ) -> ShellDiscoveryResult {
        let root = root.as_ref();
        let mut result = ShellDiscoveryResult::default();

        if !root.exists() {
            return result;
        }

        let entries = match fs::read_dir(root) {
            Ok(entries) => entries,
            Err(error) => {
                result.errors.push(format!(
                    "Failed to read shell directory {}: {error}",
                    root.display()
                ));

                return result;
            }
        };

        for entry in entries.flatten() {
            let path = entry.path();

            if !path.is_dir() {
                continue;
            }

            match self.loader.load_directory(&path) {
                ShellResourceLoadResult::Loaded(manifest) => {
                    result.discovered.push(manifest);
                }

                ShellResourceLoadResult::Skipped(reason) => {
                    result.skipped.push(format!(
                        "{}: {reason}",
                        path.display()
                    ));
                }

                ShellResourceLoadResult::Failed(error) => {
                    result.errors.push(error);
                }
            }
        }

        result
    }

    pub fn discover_paths(
        &self,
        root: impl AsRef<Path>,
    ) -> Vec<PathBuf> {
        let root = root.as_ref();

        let Ok(entries) = fs::read_dir(root) else {
            return Vec::new();
        };

        entries
            .flatten()
            .map(|entry| entry.path())
            .filter(|path| path.is_dir())
            .filter(|path| path.join("shell.toml").is_file())
            .collect()
    }
}

impl Default for ShellDiscovery {
    fn default() -> Self {
        Self::new()
    }
}
