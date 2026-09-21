use std::path::PathBuf;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ScpDirection {
    Upload,
    Download,
}

#[derive(Debug, Clone)]
pub struct ScpOptions {
    pub recursive: bool,
    pub preserve_permissions: bool,
    pub preserve_timestamps: bool,
    pub overwrite: bool,
}

impl Default for ScpOptions {
    fn default() -> Self {
        Self {
            recursive: false,
            preserve_permissions: true,
            preserve_timestamps: true,
            overwrite: false,
        }
    }
}

#[derive(Debug, Clone)]
pub struct ScpTransfer {
    pub direction: ScpDirection,
    pub local_path: PathBuf,
    pub remote_path: PathBuf,
    pub options: ScpOptions,
    pub bytes_transferred: u64,
    pub total_bytes: Option<u64>,
}

impl ScpTransfer {
    pub fn new(
        direction: ScpDirection,
        local_path: impl Into<PathBuf>,
        remote_path: impl Into<PathBuf>,
    ) -> Self {
        Self {
            direction,
            local_path: local_path.into(),
            remote_path: remote_path.into(),
            options: ScpOptions::default(),
            bytes_transferred: 0,
            total_bytes: None,
        }
    }

    pub fn progress(&self) -> Option<f64> {
        let total = self.total_bytes?;

        if total == 0 {
            return Some(1.0);
        }

        Some(
            self.bytes_transferred as f64
                / total as f64,
        )
    }

    pub fn is_complete(&self) -> bool {
        match self.total_bytes {
            Some(total) => self.bytes_transferred >= total,
            None => false,
        }
    }
}
