use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Permission {
    ReadConfiguration,
    WriteConfiguration,

    ReadClipboard,
    WriteClipboard,

    ReadFilesystem,
    WriteFilesystem,

    ExecuteCommands,

    Network,
    LocalNetwork,

    SpawnProcess,

    AccessEnvironment,

    AccessTerminal,

    AccessWindows,
    AccessTabs,
    AccessPanes,
    AccessSessions,

    LoadOtherPlugins,

    InstallPlugins,
    UpdatePlugins,

    SystemIntegration,
}

#[derive(Debug, Clone, Default)]
pub struct PermissionSet {
    permissions: Vec<Permission>,
}

impl PermissionSet {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn from_permissions(
        permissions: impl IntoIterator<Item = Permission>,
    ) -> Self {
        let mut set = Self::new();

        for permission in permissions {
            set.grant(permission);
        }

        set
    }

    pub fn grant(&mut self, permission: Permission) {
        if !self.permissions.contains(&permission) {
            self.permissions.push(permission);
        }
    }

    pub fn revoke(&mut self, permission: Permission) {
        self.permissions.retain(|item| *item != permission);
    }

    pub fn contains(&self, permission: Permission) -> bool {
        self.permissions.contains(&permission)
    }

    pub fn is_empty(&self) -> bool {
        self.permissions.is_empty()
    }

    pub fn len(&self) -> usize {
        self.permissions.len()
    }

    pub fn all(&self) -> &[Permission] {
        &self.permissions
    }

    pub fn clear(&mut self) {
        self.permissions.clear();
    }
}

#[derive(Debug, Clone)]
pub struct PermissionRequest {
    pub plugin_id: String,
    pub requested: PermissionSet,
    pub reason: Option<String>,
}

impl PermissionRequest {
    pub fn new<S: Into<String>>(
        plugin_id: S,
        requested: PermissionSet,
    ) -> Self {
        Self {
            plugin_id: plugin_id.into(),
            requested,
            reason: None,
        }
    }

    pub fn with_reason<S: Into<String>>(mut self, reason: S) -> Self {
        self.reason = Some(reason.into());
        self
    }
}
