use std::fs;
use std::path::{Path, PathBuf};

use super::{
    loader::{
        ProtocolResourceLoadResult,
        ProtocolResourceLoader,
    },
    manifest::ProtocolResourceManifest,
};

#[derive(Debug, Default)]
pub struct ProtocolDiscoveryResult {
    pub discovered: Vec<ProtocolResourceManifest>,
    pub skipped: Vec<String>,
    pub errors: Vec<String>,
}

pub struct ProtocolDiscovery {
    loader: ProtocolResourceLoader,
}

impl ProtocolDiscovery {
    pub fn new() -> Self {
        Self {
            loader: ProtocolResourceLoader::new(),
        }
    }

    pub fn discover(
        &self,
        root: impl AsRef<Path>,
    ) -> ProtocolDiscoveryResult {
        let root = root.as_ref();
        let mut result =
            ProtocolDiscoveryResult::default();

        if !root.exists() {
            return result;
        }

        let entries =
            match fs::read_dir(root) {
                Ok(entries) => entries,

                Err(error) => {
                    result.errors.push(format!(
                        "Failed to read protocol directory {}: {error}",
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
                ProtocolResourceLoadResult::Loaded(
                    manifest,
                ) => {
                    result.discovered.push(manifest);
                }

                ProtocolResourceLoadResult::Skipped(
                    reason,
                ) => {
                    result.skipped.push(format!(
                        "{}: {reason}",
                        path.display()
                    ));
                }

                ProtocolResourceLoadResult::Failed(
                    error,
                ) => {
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

        let Ok(entries) =
            fs::read_dir(root)
        else {
            return Vec::new();
        };

        entries
            .flatten()
            .map(|entry| entry.path())
            .filter(|path| path.is_dir())
            .filter(|path| {
                path.join("protocol.toml").is_file()
            })
            .collect()
    }
}

impl Default for ProtocolDiscovery {
    fn default() -> Self {
        Self::new()
    }
}
