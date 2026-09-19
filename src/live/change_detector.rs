use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};
use std::time::SystemTime;

use super::watcher::{
    WatchEvent,
    WatchEventKind,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ResourceChangeKind {
    Added,
    Modified,
    Removed,
    Renamed,
}

#[derive(Debug, Clone)]
pub struct ResourceChange {
    pub kind: ResourceChangeKind,
    pub path: PathBuf,
    pub previous_path: Option<PathBuf>,
    pub previous_modified: Option<SystemTime>,
    pub modified: Option<SystemTime>,
}

impl ResourceChange {
    pub fn from_event(event: &WatchEvent) -> Self {
        let kind = match event.kind {
            WatchEventKind::Created => ResourceChangeKind::Added,
            WatchEventKind::Modified => ResourceChangeKind::Modified,
            WatchEventKind::Removed => ResourceChangeKind::Removed,
            WatchEventKind::Renamed => ResourceChangeKind::Renamed,
        };

        Self {
            kind,
            path: event.path.clone(),
            previous_path: event.previous_path.clone(),
            previous_modified: None,
            modified: Self::modified_time(&event.path),
        }
    }

    fn modified_time(path: &Path) -> Option<SystemTime> {
        fs::metadata(path)
            .ok()
            .and_then(|metadata| metadata.modified().ok())
    }
}

#[derive(Debug, Default)]
pub struct ChangeDetector {
    known: HashMap<PathBuf, SystemTime>,
}

impl ChangeDetector {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn track(
        &mut self,
        path: impl Into<PathBuf>,
    ) {
        let path = path.into();

        if let Some(modified) = Self::modified_time(&path) {
            self.known.insert(path, modified);
        }
    }

    pub fn untrack(
        &mut self,
        path: &Path,
    ) {
        self.known.remove(path);
    }

    pub fn detect(
        &mut self,
        event: &WatchEvent,
    ) -> Option<ResourceChange> {
        let path = &event.path;

        let previous_modified = self.known.get(path).copied();

        let change = match event.kind {
            WatchEventKind::Created => {
                if self.known.contains_key(path) {
                    None
                } else {
                    Some(ResourceChange {
                        kind: ResourceChangeKind::Added,
                        path: path.clone(),
                        previous_path: None,
                        previous_modified: None,
                        modified: Self::modified_time(path),
                    })
                }
            }

            WatchEventKind::Modified => {
                let modified = Self::modified_time(path);

                if modified == previous_modified {
                    None
                } else {
                    Some(ResourceChange {
                        kind: ResourceChangeKind::Modified,
                        path: path.clone(),
                        previous_path: None,
                        previous_modified,
                        modified,
                    })
                }
            }

            WatchEventKind::Removed => {
                Some(ResourceChange {
                    kind: ResourceChangeKind::Removed,
                    path: path.clone(),
                    previous_path: None,
                    previous_modified,
                    modified: None,
                })
            }

            WatchEventKind::Renamed => {
                Some(ResourceChange {
                    kind: ResourceChangeKind::Renamed,
                    path: path.clone(),
                    previous_path: event.previous_path.clone(),
                    previous_modified,
                    modified: Self::modified_time(path),
                })
            }
        };

        match &change {
            Some(change) => {
                match change.kind {
                    ResourceChangeKind::Removed => {
                        self.known.remove(path);
                    }

                    ResourceChangeKind::Renamed => {
                        if let Some(previous) = &change.previous_path {
                            self.known.remove(previous);
                        }

                        if let Some(modified) = change.modified {
                            self.known.insert(path.clone(), modified);
                        }
                    }

                    ResourceChangeKind::Added
                    | ResourceChangeKind::Modified => {
                        if let Some(modified) = change.modified {
                            self.known.insert(path.clone(), modified);
                        }
                    }
                }
            }

            None => {}
        }

        change
    }

    pub fn scan(
        &mut self,
        paths: &[PathBuf],
    ) -> Vec<ResourceChange> {
        let mut changes = Vec::new();

        for path in paths {
            let exists = path.exists();
            let known = self.known.get(path).copied();

            if exists {
                let modified = Self::modified_time(path);

                match known {
                    None => {
                        self.known
                            .insert(path.clone(), modified.unwrap_or(SystemTime::UNIX_EPOCH));

                        changes.push(ResourceChange {
                            kind: ResourceChangeKind::Added,
                            path: path.clone(),
                            previous_path: None,
                            previous_modified: None,
                            modified,
                        });
                    }

                    Some(previous) if Some(previous) != modified => {
                        self.known.insert(
                            path.clone(),
                            modified.unwrap_or(SystemTime::UNIX_EPOCH),
                        );

                        changes.push(ResourceChange {
                            kind: ResourceChangeKind::Modified,
                            path: path.clone(),
                            previous_path: None,
                            previous_modified: Some(previous),
                            modified,
                        });
                    }

                    _ => {}
                }
            } else if known.is_some() {
                self.known.remove(path);

                changes.push(ResourceChange {
                    kind: ResourceChangeKind::Removed,
                    path: path.clone(),
                    previous_path: None,
                    previous_modified: known,
                    modified: None,
                });
            }
        }

        changes
    }

    fn modified_time(path: &Path) -> Option<SystemTime> {
        fs::metadata(path)
            .ok()
            .and_then(|metadata| metadata.modified().ok())
    }
}
