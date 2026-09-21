#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum ShellPermission {
    ExecuteProcesses,
    ReadFiles,
    WriteFiles,
    ReadEnvironment,
    ModifyEnvironment,
    NetworkAccess,
    ClipboardRead,
    ClipboardWrite,
    SpawnShells,
    LoadPlugins,
    AccessTerminal,
    AccessSystemDevices,
}

#[derive(Clone, Debug)]
pub struct CustomShellSecurityPolicy {
    permissions:
        Vec<ShellPermission>,
    allow_unknown_operations: bool,
}

impl CustomShellSecurityPolicy {
    pub fn new() -> Self {
        Self {
            permissions: Vec::new(),
            allow_unknown_operations: false,
        }
    }

    pub fn restrictive() -> Self {
        Self::new()
    }

    pub fn allow(
        &mut self,
        permission: ShellPermission,
    ) {
        if !self.permissions.contains(&permission) {
            self.permissions.push(permission);
        }
    }

    pub fn deny(
        &mut self,
        permission: ShellPermission,
    ) {
        self.permissions
            .retain(|item| *item != permission);
    }

    pub fn is_allowed(
        &self,
        permission: ShellPermission,
    ) -> bool {
        self.permissions.contains(&permission)
    }

    pub fn permissions(
        &self,
    ) -> &[ShellPermission] {
        &self.permissions
    }

    pub fn set_allow_unknown_operations(
        &mut self,
        allowed: bool,
    ) {
        self.allow_unknown_operations =
            allowed;
    }

    pub fn allow_unknown_operations(
        &self,
    ) -> bool {
        self.allow_unknown_operations
    }

    pub fn allow_all_for_development() -> Self {
        let mut policy = Self::new();

        let permissions = [
            ShellPermission::ExecuteProcesses,
            ShellPermission::ReadFiles,
            ShellPermission::WriteFiles,
            ShellPermission::ReadEnvironment,
            ShellPermission::ModifyEnvironment,
            ShellPermission::NetworkAccess,
            ShellPermission::ClipboardRead,
            ShellPermission::ClipboardWrite,
            ShellPermission::SpawnShells,
            ShellPermission::LoadPlugins,
            ShellPermission::AccessTerminal,
            ShellPermission::AccessSystemDevices,
        ];

        for permission in permissions {
            policy.allow(permission);
        }

        policy.set_allow_unknown_operations(true);

        policy
    }
}

impl Default for CustomShellSecurityPolicy {
    fn default() -> Self {
        Self::restrictive()
    }
}
