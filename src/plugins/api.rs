use std::collections::HashMap;
use std::fmt;

use super::permissions::PermissionSet;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct PluginId(String);

impl PluginId {
    pub fn new<S: Into<String>>(id: S) -> Self {
        Self(id.into())
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl From<&str> for PluginId {
    fn from(value: &str) -> Self {
        Self::new(value)
    }
}

impl From<String> for PluginId {
    fn from(value: String) -> Self {
        Self::new(value)
    }
}

impl fmt::Display for PluginId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}

/// Events delivered to plugins by Conduit.
#[derive(Debug, Clone)]
pub enum PluginEvent {
    Startup,
    Shutdown,
    WindowCreated,
    WindowClosed,
    TabCreated,
    TabClosed,
    PaneCreated,
    PaneClosed,
    SessionCreated,
    SessionClosed,
    ConfigurationChanged,
    ThemeChanged,
    Custom {
        name: String,
        data: Vec<u8>,
    },
}

/// Shared context exposed to a plugin.
///
/// The context intentionally exposes only controlled capabilities rather
/// than giving plugins unrestricted access to Conduit's internals.
#[derive(Debug, Clone)]
pub struct PluginContext {
    plugin_id: PluginId,
    permissions: PermissionSet,
    configuration: HashMap<String, String>,
}

impl PluginContext {
    pub fn new(plugin_id: PluginId, permissions: PermissionSet) -> Self {
        Self {
            plugin_id,
            permissions,
            configuration: HashMap::new(),
        }
    }

    pub fn plugin_id(&self) -> &PluginId {
        &self.plugin_id
    }

    pub fn permissions(&self) -> &PermissionSet {
        &self.permissions
    }

    pub fn configuration(&self) -> &HashMap<String, String> {
        &self.configuration
    }

    pub fn configuration_mut(&mut self) -> &mut HashMap<String, String> {
        &mut self.configuration
    }

    pub fn has_permission(
        &self,
        permission: super::permissions::Permission,
    ) -> bool {
        self.permissions.contains(permission)
    }
}

pub type PluginResult = Result<(), String>;

/// Main interface implemented by native or embedded Conduit plugins.
pub trait Plugin: Send {
    fn id(&self) -> &PluginId;

    fn initialize(&mut self, _context: &mut PluginContext) -> PluginResult {
        Ok(())
    }

    fn handle_event(
        &mut self,
        _event: &PluginEvent,
        _context: &mut PluginContext,
    ) -> PluginResult {
        Ok(())
    }

    fn shutdown(&mut self, _context: &mut PluginContext) -> PluginResult {
        Ok(())
    }
}
