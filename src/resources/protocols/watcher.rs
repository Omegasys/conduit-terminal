use std::collections::BTreeMap;
use std::path::{Path, PathBuf};
use std::time::SystemTime;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ProtocolResourceChange {
    Added(PathBuf),
    Modified(PathBuf),
    Removed(PathBuf),
}

#[derive(Debug, Default)]
pub struct ProtocolResourceWatcher {
    known_manifests:
        BTreeMap<PathBuf, SystemTime>,
}

impl ProtocolResourceWatcher {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn scan(
        &mut self,
        root: impl AsRef<Path>,
    ) -> Vec<ProtocolResourceChange> {
        let root = root.as_ref();

        let mut changes = Vec::new();

        let mut current =
            BTreeMap::new();

        let Ok(entries) =
            std::fs::read_dir(root)
        else {
            for path in
                self.known_manifests.keys()
            {
                changes.push(
                    ProtocolResourceChange::Removed(
                        path.clone(),
                    ),
                );
            }

            self.known_manifests.clear();

            return changes;
        };

        for entry in entries.flatten() {
            let directory =
                entry.path();

            if !directory.is_dir() {
                continue;
            }

            let manifest =
                directory.join("protocol.toml");

            if !manifest.is_file() {
                continue;
            }

            let Ok(metadata) =
                std::fs::metadata(&manifest)
            else {
                continue;
            };

            let modified = metadata
                .modified()
                .unwrap_or(
                    SystemTime::UNIX_EPOCH,
                );

            current.insert(
                manifest.clone(),
                modified,
            );

            match self
                .known_manifests
                .get(&manifest)
            {
                None => {
                    changes.push(
                        ProtocolResourceChange::Added(
                            manifest,
                        ),
                    );
                }

                Some(previous)
                    if *previous != modified =>
                {
                    changes.push(
                        ProtocolResourceChange::Modified(
                            manifest,
                        ),
                    );
                }

                Some(_) => {}
            }
        }

        for path in
            self.known_manifests.keys()
        {
            if !current.contains_key(path) {
                changes.push(
                    ProtocolResourceChange::Removed(
                        path.clone(),
                    ),
                );
            }
        }

        self.known_manifests = current;

        changes
    }

    pub fn clear(&mut self) {
        self.known_manifests.clear();
    }
}
