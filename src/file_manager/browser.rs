use std::fs;
use std::io;
use std::path::{Path, PathBuf};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EntryKind {
    File,
    Directory,
    Symlink,
    Other,
}

#[derive(Debug, Clone)]
pub struct DirectoryEntry {
    name: String,
    path: PathBuf,
    kind: EntryKind,
    size: Option<u64>,
    hidden: bool,
}

impl DirectoryEntry {
    pub fn new(path: PathBuf, metadata: &fs::Metadata) -> Self {
        let kind = if metadata.is_dir() {
            EntryKind::Directory
        } else if metadata.is_file() {
            EntryKind::File
        } else {
            EntryKind::Other
        };

        let name = path
            .file_name()
            .map(|value| value.to_string_lossy().into_owned())
            .unwrap_or_default();

        Self {
            hidden: name.starts_with('.'),
            name,
            path,
            kind,
            size: if metadata.is_file() {
                Some(metadata.len())
            } else {
                None
            },
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn path(&self) -> &Path {
        &self.path
    }

    pub fn kind(&self) -> EntryKind {
        self.kind
    }

    pub fn size(&self) -> Option<u64> {
        self.size
    }

    pub fn is_hidden(&self) -> bool {
        self.hidden
    }

    pub fn is_directory(&self) -> bool {
        self.kind == EntryKind::Directory
    }

    pub fn is_file(&self) -> bool {
        self.kind == EntryKind::File
    }
}

#[derive(Debug, Clone)]
pub struct FileBrowserState {
    current_directory: PathBuf,
    selected: Option<usize>,
    show_hidden: bool,
}

impl FileBrowserState {
    pub fn new(path: impl Into<PathBuf>) -> Self {
        Self {
            current_directory: path.into(),
            selected: None,
            show_hidden: false,
        }
    }

    pub fn current_directory(&self) -> &Path {
        &self.current_directory
    }

    pub fn selected(&self) -> Option<usize> {
        self.selected
    }

    pub fn show_hidden(&self) -> bool {
        self.show_hidden
    }

    pub fn set_directory(&mut self, path: impl Into<PathBuf>) {
        self.current_directory = path.into();
        self.selected = None;
    }

    pub fn set_selected(&mut self, selected: Option<usize>) {
        self.selected = selected;
    }

    pub fn set_show_hidden(&mut self, show: bool) {
        self.show_hidden = show;
    }
}

#[derive(Debug)]
pub struct FileBrowser {
    state: FileBrowserState,
    entries: Vec<DirectoryEntry>,
}

impl FileBrowser {
    pub fn new(path: impl Into<PathBuf>) -> io::Result<Self> {
        let mut browser = Self {
            state: FileBrowserState::new(path),
            entries: Vec::new(),
        };

        browser.refresh()?;
        Ok(browser)
    }

    pub fn refresh(&mut self) -> io::Result<()> {
        self.entries.clear();

        for result in fs::read_dir(self.state.current_directory())? {
            let entry = result?;
            let path = entry.path();
            let metadata = fs::symlink_metadata(&path)?;

            let directory_entry = DirectoryEntry::new(path, &metadata);

            if !self.state.show_hidden() && directory_entry.is_hidden() {
                continue;
            }

            self.entries.push(directory_entry);
        }

        self.entries.sort_by(|a, b| {
            match (a.is_directory(), b.is_directory()) {
                (true, false) => std::cmp::Ordering::Less,
                (false, true) => std::cmp::Ordering::Greater,
                _ => a.name().cmp(b.name()),
            }
        });

        if self.entries.is_empty() {
            self.state.set_selected(None);
        } else if self.state.selected().unwrap_or(0) >= self.entries.len() {
            self.state.set_selected(Some(self.entries.len() - 1));
        }

        Ok(())
    }

    pub fn entries(&self) -> &[DirectoryEntry] {
        &self.entries
    }

    pub fn state(&self) -> &FileBrowserState {
        &self.state
    }

    pub fn state_mut(&mut self) -> &mut FileBrowserState {
        &mut self.state
    }

    pub fn selected_entry(&self) -> Option<&DirectoryEntry> {
        self.state
            .selected()
            .and_then(|index| self.entries.get(index))
    }

    pub fn select(&mut self, index: usize) -> bool {
        if index < self.entries.len() {
            self.state.set_selected(Some(index));
            true
        } else {
            false
        }
    }

    pub fn enter_selected(&mut self) -> io::Result<bool> {
        let path = match self.selected_entry() {
            Some(entry) if entry.is_directory() => entry.path().to_path_buf(),
            _ => return Ok(false),
        };

        self.state.set_directory(path);
        self.refresh()?;

        Ok(true)
    }

    pub fn parent(&mut self) -> io::Result<bool> {
        let parent = match self.state.current_directory().parent() {
            Some(path) => path.to_path_buf(),
            None => return Ok(false),
        };

        self.state.set_directory(parent);
        self.refresh()?;

        Ok(true)
    }

    pub fn toggle_hidden(&mut self) -> io::Result<()> {
        let show = !self.state.show_hidden();
        self.state.set_show_hidden(show);
        self.refresh()
    }
}
