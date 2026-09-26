use std::fs;
use std::io;
use std::path::{Path, PathBuf};

#[derive(Debug, Clone)]
pub struct HistoryStorageConfig {
    root: PathBuf,
    database_file: PathBuf,
}

impl Default for HistoryStorageConfig {
    fn default() -> Self {
        let root = default_history_directory();

        Self {
            database_file: root.join("history.db"),
            root,
        }
    }
}

impl HistoryStorageConfig {
    pub fn new(root: impl Into<PathBuf>) -> Self {
        let root = root.into();

        Self {
            database_file: root.join("history.db"),
            root,
        }
    }

    pub fn root(&self) -> &Path {
        &self.root
    }

    pub fn database_file(&self) -> &Path {
        &self.database_file
    }

    pub fn set_database_file(&mut self, path: impl Into<PathBuf>) {
        self.database_file = path.into();
    }
}

#[derive(Debug)]
pub enum HistoryStorageError {
    Io(io::Error),
    InvalidFormat(String),
}

impl std::fmt::Display for HistoryStorageError {
    fn fmt(
        &self,
        formatter: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
        match self {
            Self::Io(error) => write!(formatter, "I/O error: {error}"),
            Self::InvalidFormat(error) => {
                write!(formatter, "invalid history format: {error}")
            }
        }
    }
}

impl std::error::Error for HistoryStorageError {}

impl From<io::Error> for HistoryStorageError {
    fn from(error: io::Error) -> Self {
        Self::Io(error)
    }
}

#[derive(Debug, Clone)]
pub struct HistoryStorage {
    config: HistoryStorageConfig,
}

impl Default for HistoryStorage {
    fn default() -> Self {
        Self::new(HistoryStorageConfig::default())
    }
}

impl HistoryStorage {
    pub fn new(config: HistoryStorageConfig) -> Self {
        Self { config }
    }

    pub fn config(&self) -> &HistoryStorageConfig {
        &self.config
    }

    pub fn initialize(&self) -> Result<(), HistoryStorageError> {
        fs::create_dir_all(self.config.root())?;
        Ok(())
    }

    pub fn write(
        &self,
        data: &[u8],
    ) -> Result<(), HistoryStorageError> {
        self.initialize()?;
        fs::write(self.config.database_file(), data)?;
        Ok(())
    }

    pub fn read(&self) -> Result<Vec<u8>, HistoryStorageError> {
        if !self.config.database_file().exists() {
            return Ok(Vec::new());
        }

        Ok(fs::read(self.config.database_file())?)
    }

    pub fn exists(&self) -> bool {
        self.config.database_file().exists()
    }

    pub fn remove(&self) -> Result<(), HistoryStorageError> {
        if self.exists() {
            fs::remove_file(self.config.database_file())?;
        }

        Ok(())
    }
}

fn default_history_directory() -> PathBuf {
    if let Some(home) = std::env::var_os("HOME") {
        return PathBuf::from(home)
            .join(".local")
            .join("share")
            .join("conduit")
            .join("history");
    }

    PathBuf::from(".conduit").join("history")
}
