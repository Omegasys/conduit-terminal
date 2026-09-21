use std::path::{Path, PathBuf};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DirectoryState {
    current: Option<PathBuf>,
    previous: Option<PathBuf>,
}

impl DirectoryState {
    pub fn new() -> Self {
        Self {
            current: None,
            previous: None,
        }
    }

    pub fn current(&self) -> Option<&Path> {
        self.current.as_deref()
    }

    pub fn previous(&self) -> Option<&Path> {
        self.previous.as_deref()
    }

    pub fn set<P: Into<PathBuf>>(&mut self, directory: P) {
        let directory = directory.into();

        if self.current.as_ref() != Some(&directory) {
            self.previous = self.current.take();
            self.current = Some(directory);
        }
    }

    pub fn clear(&mut self) {
        self.current = None;
        self.previous = None;
    }

    pub fn changed(&self) -> bool {
        self.current != self.previous
    }

    pub fn is_known(&self) -> bool {
        self.current.is_some()
    }
}

impl Default for DirectoryState {
    fn default() -> Self {
        Self::new()
    }
}

pub fn normalize_directory(path: impl AsRef<Path>) -> PathBuf {
    let path = path.as_ref();

    if let Ok(canonical) = path.canonicalize() {
        return canonical;
    }

    path.to_path_buf()
}
