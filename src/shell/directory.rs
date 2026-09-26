//! Working-directory tracking for shell sessions.

use std::{
    fs,
    path::{Path, PathBuf},
};

/// A validated shell working directory.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WorkingDirectory {
    path: PathBuf,
}

impl WorkingDirectory {
    pub fn new(path: impl Into<PathBuf>) -> Option<Self> {
        let path = path.into();

        if path.is_dir() {
            Some(Self { path })
        } else {
            None
        }
    }

    pub fn from_current() -> Option<Self> {
        std::env::current_dir().ok().and_then(Self::new)
    }

    pub fn path(&self) -> &Path {
        &self.path
    }

    pub fn into_path(self) -> PathBuf {
        self.path
    }

    pub fn display(&self) -> String {
        self.path.to_string_lossy().into_owned()
    }

    pub fn exists(&self) -> bool {
        self.path.exists()
    }

    pub fn is_dir(&self) -> bool {
        self.path.is_dir()
    }

    pub fn parent(&self) -> Option<Self> {
        self.path.parent().and_then(Self::new)
    }

    pub fn canonicalize(&self) -> std::io::Result<Self> {
        let path = fs::canonicalize(&self.path)?;

        Ok(Self { path })
    }
}

/// Tracks the current working directory of a shell session.
#[derive(Debug, Clone, Default)]
pub struct DirectoryTracker {
    current: Option<WorkingDirectory>,
    previous: Option<WorkingDirectory>,
    change_count: u64,
}

impl DirectoryTracker {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn from_current() -> Self {
        Self {
            current: WorkingDirectory::from_current(),
            previous: None,
            change_count: 0,
        }
    }

    pub fn current(&self) -> Option<&WorkingDirectory> {
        self.current.as_ref()
    }

    pub fn previous(&self) -> Option<&WorkingDirectory> {
        self.previous.as_ref()
    }

    pub fn change_count(&self) -> u64 {
        self.change_count
    }

    /// Update the tracked directory.
    ///
    /// Returns `true` when the directory actually changed.
    pub fn update(
        &mut self,
        path: impl Into<PathBuf>,
    ) -> bool {
        let Some(next) = WorkingDirectory::new(path) else {
            return false;
        };

        if self.current.as_ref() == Some(&next) {
            return false;
        }

        self.previous = self.current.take();
        self.current = Some(next);
        self.change_count += 1;

        true
    }

    pub fn refresh(&mut self) -> bool {
        let Ok(path) = std::env::current_dir() else {
            return false;
        };

        self.update(path)
    }

    pub fn swap_previous(&mut self) -> bool {
        let Some(previous) = self.previous.take() else {
            return false;
        };

        if let Some(current) = self.current.take() {
            self.previous = Some(current);
        }

        self.current = Some(previous);
        self.change_count += 1;

        true
    }

    pub fn clear(&mut self) {
        self.current = None;
        self.previous = None;
    }
}
