use std::fs;
use std::io;
use std::path::Path;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PermissionKind {
    Read,
    Write,
    Execute,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FilePermission {
    pub kind: PermissionKind,
    pub owner: bool,
    pub group: bool,
    pub other: bool,
}

#[derive(Debug, Clone, Copy, Default)]
pub struct FilePermissions {
    readonly: bool,
    mode: Option<u32>,
}

impl FilePermissions {
    pub fn inspect(path: impl AsRef<Path>) -> io::Result<Self> {
        let metadata = fs::symlink_metadata(path)?;
        let readonly = metadata.permissions().readonly();

        #[cfg(unix)]
        let mode = {
            use std::os::unix::fs::PermissionsExt;
            Some(metadata.permissions().mode())
        };

        #[cfg(not(unix))]
        let mode = None;

        Ok(Self {
            readonly,
            mode,
        })
    }

    pub fn readonly(&self) -> bool {
        self.readonly
    }

    pub fn mode(&self) -> Option<u32> {
        self.mode
    }

    pub fn allows(
        &self,
        permission: PermissionKind,
    ) -> bool {
        match permission {
            PermissionKind::Read => self.mode.map(|mode| mode & 0o444 != 0).unwrap_or(true),
            PermissionKind::Write => {
                self.mode.map(|mode| mode & 0o222 != 0).unwrap_or(!self.readonly)
            }
            PermissionKind::Execute => {
                self.mode.map(|mode| mode & 0o111 != 0).unwrap_or(false)
            }
        }
    }

    pub fn owner_permissions(&self) -> Option<[bool; 3]> {
        self.mode.map(|mode| {
            [
                mode & 0o400 != 0,
                mode & 0o200 != 0,
                mode & 0o100 != 0,
            ]
        })
    }

    pub fn group_permissions(&self) -> Option<[bool; 3]> {
        self.mode.map(|mode| {
            [
                mode & 0o040 != 0,
                mode & 0o020 != 0,
                mode & 0o010 != 0,
            ]
        })
    }

    pub fn other_permissions(&self) -> Option<[bool; 3]> {
        self.mode.map(|mode| {
            [
                mode & 0o004 != 0,
                mode & 0o002 != 0,
                mode & 0o001 != 0,
            ]
        })
    }
}
