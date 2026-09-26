use std::path::PathBuf;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FileManagerAction {
    Open(PathBuf),
    OpenDirectory(PathBuf),
    Preview(PathBuf),
    CopyPath(PathBuf),
    ChangeDirectory(PathBuf),
}

#[derive(Debug, Default)]
pub struct FileManagerIntegration {
    pending: Vec<FileManagerAction>,
}

impl FileManagerIntegration {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn emit(&mut self, action: FileManagerAction) {
        self.pending.push(action);
    }

    pub fn open(&mut self, path: impl Into<PathBuf>) {
        self.emit(FileManagerAction::Open(path.into()));
    }

    pub fn open_directory(&mut self, path: impl Into<PathBuf>) {
        self.emit(FileManagerAction::OpenDirectory(path.into()));
    }

    pub fn preview(&mut self, path: impl Into<PathBuf>) {
        self.emit(FileManagerAction::Preview(path.into()));
    }

    pub fn copy_path(&mut self, path: impl Into<PathBuf>) {
        self.emit(FileManagerAction::CopyPath(path.into()));
    }

    pub fn change_directory(&mut self, path: impl Into<PathBuf>) {
        self.emit(FileManagerAction::ChangeDirectory(path.into()));
    }

    pub fn next(&mut self) -> Option<FileManagerAction> {
        if self.pending.is_empty() {
            None
        } else {
            Some(self.pending.remove(0))
        }
    }

    pub fn pending(&self) -> &[FileManagerAction] {
        &self.pending
    }

    pub fn clear(&mut self) {
        self.pending.clear();
    }
}
