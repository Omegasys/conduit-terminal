use std::path::PathBuf;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SftpFileType {
    File,
    Directory,
    Symlink,
    Socket,
    BlockDevice,
    CharacterDevice,
    Unknown,
}

#[derive(Debug, Clone)]
pub struct SftpEntry {
    pub path: PathBuf,
    pub name: String,
    pub file_type: SftpFileType,
    pub size: u64,
    pub permissions: u32,
    pub modified: Option<u64>,
}

impl SftpEntry {
    pub fn new(
        path: impl Into<PathBuf>,
        file_type: SftpFileType,
    ) -> Self {
        let path = path.into();

        let name = path
            .file_name()
            .map(|name| name.to_string_lossy().into_owned())
            .unwrap_or_default();

        Self {
            path,
            name,
            file_type,
            size: 0,
            permissions: 0,
            modified: None,
        }
    }

    pub fn is_directory(&self) -> bool {
        self.file_type == SftpFileType::Directory
    }

    pub fn is_file(&self) -> bool {
        self.file_type == SftpFileType::File
    }
}

#[derive(Debug, Clone)]
pub struct SftpOptions {
    pub preserve_permissions: bool,
    pub preserve_timestamps: bool,
    pub overwrite: bool,
    pub recursive: bool,
}

impl Default for SftpOptions {
    fn default() -> Self {
        Self {
            preserve_permissions: true,
            preserve_timestamps: true,
            overwrite: false,
            recursive: true,
        }
    }
}

#[derive(Debug)]
pub struct SftpSession {
    connected: bool,
    current_directory: PathBuf,
}

impl Default for SftpSession {
    fn default() -> Self {
        Self::new()
    }
}

impl SftpSession {
    pub fn new() -> Self {
        Self {
            connected: false,
            current_directory: PathBuf::from("."),
        }
    }

    pub fn connect(&mut self) {
        self.connected = true;
    }

    pub fn disconnect(&mut self) {
        self.connected = false;
    }

    pub fn is_connected(&self) -> bool {
        self.connected
    }

    pub fn current_directory(&self) -> &PathBuf {
        &self.current_directory
    }

    pub fn set_current_directory(
        &mut self,
        path: impl Into<PathBuf>,
    ) {
        self.current_directory = path.into();
    }

    pub fn list(&self, _path: impl Into<PathBuf>) -> Result<Vec<SftpEntry>, String> {
        if !self.connected {
            return Err("SFTP session is not connected".into());
        }

        Ok(Vec::new())
    }

    pub fn upload(
        &self,
        _local: impl Into<PathBuf>,
        _remote: impl Into<PathBuf>,
        _options: &SftpOptions,
    ) -> Result<(), String> {
        if !self.connected {
            return Err("SFTP session is not connected".into());
        }

        Ok(())
    }

    pub fn download(
        &self,
        _remote: impl Into<PathBuf>,
        _local: impl Into<PathBuf>,
        _options: &SftpOptions,
    ) -> Result<(), String> {
        if !self.connected {
            return Err("SFTP session is not connected".into());
        }

        Ok(())
    }
}
