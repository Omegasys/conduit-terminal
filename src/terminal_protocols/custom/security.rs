#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ProtocolPermission {
    AccessTerminal,
    ReadTerminalInput,
    WriteTerminalOutput,

    Graphics,
    ClipboardRead,
    ClipboardWrite,

    Hyperlinks,
    MouseInput,
    KeyboardInput,

    AlternateScreen,
    SynchronizedOutput,

    BinaryData,
    LoadPlugins,
}

#[derive(Debug, Clone)]
pub struct CustomProtocolSecurityPolicy {
    permissions: Vec<ProtocolPermission>,
    allow_unknown_operations: bool,
}

impl Default for CustomProtocolSecurityPolicy {
    fn default() -> Self {
        Self {
            permissions: vec![
                ProtocolPermission::AccessTerminal,
                ProtocolPermission::ReadTerminalInput,
                ProtocolPermission::WriteTerminalOutput,
            ],
            allow_unknown_operations: false,
        }
    }
}

impl CustomProtocolSecurityPolicy {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn allow(
        &mut self,
        permission: ProtocolPermission,
    ) {
        if !self.permissions.contains(&permission) {
            self.permissions.push(permission);
        }
    }

    pub fn deny(
        &mut self,
        permission: ProtocolPermission,
    ) {
        self.permissions
            .retain(|item| *item != permission);
    }

    pub fn is_allowed(
        &self,
        permission: ProtocolPermission,
    ) -> bool {
        self.permissions.contains(&permission)
    }

    pub fn permissions(
        &self,
    ) -> &[ProtocolPermission] {
        &self.permissions
    }

    pub fn set_allow_unknown_operations(
        &mut self,
        allowed: bool,
    ) {
        self.allow_unknown_operations = allowed;
    }

    pub fn allow_unknown_operations(&self) -> bool {
        self.allow_unknown_operations
    }

    pub fn allow_all_for_development() -> Self {
        let mut policy = Self::new();

        let permissions = [
            ProtocolPermission::Graphics,
            ProtocolPermission::ClipboardRead,
            ProtocolPermission::ClipboardWrite,
            ProtocolPermission::Hyperlinks,
            ProtocolPermission::MouseInput,
            ProtocolPermission::KeyboardInput,
            ProtocolPermission::AlternateScreen,
            ProtocolPermission::SynchronizedOutput,
            ProtocolPermission::BinaryData,
            ProtocolPermission::LoadPlugins,
        ];

        for permission in permissions {
            policy.allow(permission);
        }

        policy.allow_unknown_operations = true;

        policy
    }
}
