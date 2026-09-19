use std::path::{Path, PathBuf};
use std::sync::mpsc::{
    self,
    Receiver,
    Sender,
};
use std::time::SystemTime;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WatchEventKind {
    Created,
    Modified,
    Removed,
    Renamed,
}

#[derive(Debug, Clone)]
pub struct WatchEvent {
    pub kind: WatchEventKind,
    pub path: PathBuf,
    pub previous_path: Option<PathBuf>,
    pub timestamp: SystemTime,
}

impl WatchEvent {
    pub fn new(
        kind: WatchEventKind,
        path: impl Into<PathBuf>,
    ) -> Self {
        Self {
            kind,
            path: path.into(),
            previous_path: None,
            timestamp: SystemTime::now(),
        }
    }

    pub fn renamed(
        old_path: impl Into<PathBuf>,
        new_path: impl Into<PathBuf>,
    ) -> Self {
        Self {
            kind: WatchEventKind::Renamed,
            path: new_path.into(),
            previous_path: Some(old_path.into()),
            timestamp: SystemTime::now(),
        }
    }
}

pub struct ResourceWatcher {
    roots: Vec<PathBuf>,
    sender: Sender<WatchEvent>,
    receiver: Receiver<WatchEvent>,
}

impl ResourceWatcher {
    pub fn new() -> Self {
        let (sender, receiver) = mpsc::channel();

        Self {
            roots: Vec::new(),
            sender,
            receiver,
        }
    }

    pub fn add_root(
        &mut self,
        path: impl Into<PathBuf>,
    ) {
        let path = path.into();

        if !self.roots.contains(&path) {
            self.roots.push(path);
        }
    }

    pub fn remove_root(
        &mut self,
        path: &Path,
    ) {
        self.roots.retain(|root| root != path);
    }

    pub fn roots(&self) -> &[PathBuf] {
        &self.roots
    }

    pub fn sender(&self) -> Sender<WatchEvent> {
        self.sender.clone()
    }

    pub fn poll(&self) -> Vec<WatchEvent> {
        let mut events = Vec::new();

        while let Ok(event) = self.receiver.try_recv() {
            events.push(event);
        }

        events
    }

    pub fn emit(
        &self,
        event: WatchEvent,
    ) -> Result<(), mpsc::SendError<WatchEvent>> {
        self.sender.send(event)
    }

    pub fn is_watched(&self, path: &Path) -> bool {
        self.roots.iter().any(|root| {
            path == root || path.starts_with(root)
        })
    }
}

impl Default for ResourceWatcher {
    fn default() -> Self {
        Self::new()
    }
}
