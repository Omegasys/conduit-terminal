use std::fmt;
use std::fs;
use std::path::{Path, PathBuf};
use std::time::SystemTime;

/// Basic filesystem metadata exposed to Conduit.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FileMetadata {
    pub path: PathBuf,
    pub size: u64,
    pub is_file: bool,
    pub is_directory: bool,
    pub is_symlink: bool,
    pub modified: Option<SystemTime>,
}

impl FileMetadata {
    pub fn from_path(path: impl AsRef<Path>) -> Result<Self, LinuxFilesystemError> {
        let path = path.as_ref().to_path_buf();
        let metadata = fs::symlink_metadata(&path)?;

        Ok(Self {
            path,
            size: metadata.len(),
            is_file: metadata.is_file(),
            is_directory: metadata.is_dir(),
            is_symlink: metadata.file_type().is_symlink(),
            modified: metadata.modified().ok(),
        })
    }
}

/// Linux filesystem integration.
#[derive(Debug, Clone, Copy, Default)]
pub struct LinuxFilesystem;

impl LinuxFilesystem {
    pub fn new() -> Self {
        Self
    }

    pub fn metadata(
        &self,
        path: impl AsRef<Path>,
    ) -> Result<FileMetadata, LinuxFilesystemError> {
        FileMetadata::from_path(path)
    }

    pub fn exists(&self, path: impl AsRef<Path>) -> bool {
        path.as_ref().exists()
    }

    pub fn is_file(&self, path: impl AsRef<Path>) -> bool {
        path.as_ref().is_file()
    }

    pub fn is_directory(&self, path: impl AsRef<Path>) -> bool {
        path.as_ref().is_dir()
    }

    pub fn canonicalize(
        &self,
        path: impl AsRef<Path>,
    ) -> Result<PathBuf, LinuxFilesystemError> {
        Ok(fs::canonicalize(path)?)
    }

    pub fn read(
        &self,
        path: impl AsRef<Path>,
    ) -> Result<Vec<u8>, LinuxFilesystemError> {
        Ok(fs::read(path)?)
    }

    pub fn read_to_string(
        &self,
        path: impl AsRef<Path>,
    ) -> Result<String, LinuxFilesystemError> {
        Ok(fs::read_to_string(path)?)
    }

    pub fn create_directory(
        &self,
        path: impl AsRef<Path>,
    ) -> Result<(), LinuxFilesystemError> {
        fs::create_dir_all(path)?;
        Ok(())
    }

    pub fn remove_file(
        &self,
        path: impl AsRef<Path>,
    ) -> Result<(), LinuxFilesystemError> {
        fs::remove_file(path)?;
        Ok(())
    }
}

/// Linux filesystem errors.
#[derive(Debug)]
pub enum LinuxFilesystemError {
    Io(std::io::Error),
    PermissionDenied(PathBuf),
    InvalidPath(PathBuf),
}

impl fmt::Display for LinuxFilesystemError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Io(error) => write!(formatter, "filesystem error: {error}"),
            Self::PermissionDenied(path) => {
                write!(formatter, "permission denied: {}", path.display())
            }
            Self::InvalidPath(path) => {
                write!(formatter, "invalid path: {}", path.display())
            }
        }
    }
}

impl std::error::Error for LinuxFilesystemError {}

impl From<std::io::Error> for LinuxFilesystemError {
    fn from(error: std::io::Error) -> Self {
        if error.kind() == std::io::ErrorKind::PermissionDenied {
            Self::PermissionDenied(PathBuf::from(
                error.to_string(),
            ))
        } else {
            Self::Io(error)
        }
    }
}
