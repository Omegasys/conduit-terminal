use std::collections::HashSet;

/// Permissions used by terminal-facing security controls.
///
/// These are separate from plugin permissions because they describe
/// actions performed by terminal content, shell integrations, or
/// terminal protocol handlers.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SecurityPermission {
    ClipboardRead,
    ClipboardWrite,
    ClipboardSensitiveRead,
    ClipboardSensitiveWrite,

    OpenHyperlink,
    OpenFile,
    OpenDirectory,

    ExecuteEscapeAction,
    ChangeWindowTitle,
    ChangeWorkingDirectory,

    SetTerminalProperty,
    AccessSystemClipboard,
}

#[derive(Debug, Clone, Default)]
pub struct SecurityPermissionSet {
    permissions: HashSet<SecurityPermission>,
}

impl SecurityPermissionSet {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn allow(
        &mut self,
        permission: SecurityPermission,
    ) {
        self.permissions.insert(permission);
    }

    pub fn deny(
        &mut self,
        permission: SecurityPermission,
    ) {
        self.permissions.remove(&permission);
    }

    pub fn allows(
        &self,
        permission: SecurityPermission,
    ) -> bool {
        self.permissions.contains(&permission)
    }

    pub fn clear(&mut self) {
        self.permissions.clear();
    }

    pub fn len(&self) -> usize {
        self.permissions.len()
    }

    pub fn is_empty(&self) -> bool {
        self.permissions.is_empty()
    }

    pub fn all(&self) -> impl Iterator<Item = &SecurityPermission> {
        self.permissions.iter()
    }

    /// Conservative defaults for terminal content.
    pub fn terminal_defaults() -> Self {
        let mut set = Self::new();

        set.allow(SecurityPermission::ClipboardRead);
        set.allow(SecurityPermission::OpenHyperlink);

        set
    }

    /// Permissions appropriate for a trusted local session.
    pub fn trusted_defaults() -> Self {
        let mut set = Self::terminal_defaults();

        set.allow(SecurityPermission::ClipboardWrite);
        set.allow(SecurityPermission::OpenFile);
        set.allow(SecurityPermission::OpenDirectory);
        set.allow(SecurityPermission::ChangeWindowTitle);
        set.allow(SecurityPermission::ChangeWorkingDirectory);
        set.allow(SecurityPermission::SetTerminalProperty);

        set
    }
}
