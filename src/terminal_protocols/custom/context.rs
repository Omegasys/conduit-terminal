use std::collections::BTreeMap;

#[derive(Clone, Debug)]
pub struct ProtocolSecurityPolicy {
    pub allow_clipboard_read: bool,
    pub allow_clipboard_write: bool,
    pub allow_external_commands: bool,
    pub allow_network_access: bool,
    pub allow_file_access: bool,
    pub allow_notifications: bool,
    pub allow_title_changes: bool,
}

impl Default for ProtocolSecurityPolicy {
    fn default() -> Self {
        Self {
            allow_clipboard_read: false,
            allow_clipboard_write: true,
            allow_external_commands: false,
            allow_network_access: false,
            allow_file_access: false,
            allow_notifications: true,
            allow_title_changes: true,
        }
    }
}

impl ProtocolSecurityPolicy {
    pub fn restrictive() -> Self {
        Self {
            allow_clipboard_read: false,
            allow_clipboard_write: false,
            allow_external_commands: false,
            allow_network_access: false,
            allow_file_access: false,
            allow_notifications: false,
            allow_title_changes: false,
        }
    }
}

pub struct ProtocolContext {
    security: ProtocolSecurityPolicy,
    values: BTreeMap<String, String>,
    responses: Vec<Vec<u8>>,
}

impl ProtocolContext {
    pub fn new() -> Self {
        Self {
            security: ProtocolSecurityPolicy::default(),
            values: BTreeMap::new(),
            responses: Vec::new(),
        }
    }

    pub fn with_security(security: ProtocolSecurityPolicy) -> Self {
        Self {
            security,
            values: BTreeMap::new(),
            responses: Vec::new(),
        }
    }

    pub fn security(&self) -> &ProtocolSecurityPolicy {
        &self.security
    }

    pub fn security_mut(&mut self) -> &mut ProtocolSecurityPolicy {
        &mut self.security
    }

    pub fn set_value<S1, S2>(&mut self, key: S1, value: S2)
    where
        S1: Into<String>,
        S2: Into<String>,
    {
        self.values.insert(key.into(), value.into());
    }

    pub fn value(&self, key: &str) -> Option<&str> {
        self.values.get(key).map(String::as_str)
    }

    pub fn queue_response(&mut self, response: Vec<u8>) {
        self.responses.push(response);
    }

    pub fn take_responses(&mut self) -> Vec<Vec<u8>> {
        std::mem::take(&mut self.responses)
    }
}

impl Default for ProtocolContext {
    fn default() -> Self {
        Self::new()
    }
}
