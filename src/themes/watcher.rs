//! Theme file change tracking.
//!
//! This module provides polling-based change detection so the core does not
//! require a platform-specific filesystem watcher dependency.

use std::{
    collections::HashMap,
    fs,
    path::{Path, PathBuf},
    time::SystemTime,
};

/// A detected theme change.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ThemeChange {
    Created(PathBuf),
    Modified(PathBuf),
    Removed(PathBuf),
}

/// Tracks theme files for hot reload.
#[derive(Debug, Default)]
pub struct ThemeWatcher {
    files: HashMap<PathBuf, Option<SystemTime>>,
}

impl ThemeWatcher {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn watch(&mut self, path: impl Into<PathBuf>) {
        let path = path.into();
        let modified = modification_time(&path);
        self.files.insert(path, modified);
    }

    pub fn unwatch(&mut self, path: &Path) {
        self.files.remove(path);
    }

    pub fn clear(&mut self) {
        self.files.clear();
    }

    /// Check watched files for changes.
    pub fn poll(&mut self) -> Vec<ThemeChange> {
        let mut changes = Vec::new();

        for (path, previous) in &mut self.files {
            let current = modification_time(path);

            match (*previous, current) {
                (None, Some(_)) => {
                    changes.push(ThemeChange::Created(path.clone()));
                }
                (Some(_), None) => {
                    changes.push(ThemeChange::Removed(path.clone()));
                }
                (Some(old), Some(new)) if old != new => {
                    changes.push(ThemeChange::Modified(path.clone()));
                }
                _ => {}
            }

            *previous = current;
        }

        changes
    }

    pub fn watched(&self, path: &Path) -> bool {
        self.files.contains_key(path)
    }

    pub fn paths(&self) -> impl Iterator<Item = &Path> {
        self.files.keys().map(PathBuf::as_path)
    }

    pub fn len(&self) -> usize {
        self.files.len()
    }

    pub fn is_empty(&self) -> bool {
        self.files.is_empty()
    }
}

fn modification_time(path: &Path) -> Option<SystemTime> {
    fs::metadata(path).ok()?.modified().ok()
}
