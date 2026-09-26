use conduit_plugin_api::{
    Plugin,
    PluginContext,
    PluginEvent,
    PluginId,
    PluginResult,
};

/// Git integration plugin for Conduit.
pub struct GitPlugin {
    id: PluginId,
    enabled: bool,
}

impl Default for GitPlugin {
    fn default() -> Self {
        Self {
            id: PluginId::new("git"),
            enabled: true,
        }
    }
}

impl GitPlugin {
    pub fn new() -> Self {
        Self::default()
    }

    fn handle_custom_event(
        &mut self,
        name: &str,
        _data: &[u8],
        _context: &mut PluginContext,
    ) -> PluginResult {
        match name {
            "git.repository.changed" => Ok(()),
            "git.branch.changed" => Ok(()),
            "git.status.changed" => Ok(()),
            _ => Ok(()),
        }
    }
}

impl Plugin for GitPlugin {
    fn id(&self) -> &PluginId {
        &self.id
    }

    fn initialize(
        &mut self,
        context: &mut PluginContext,
    ) -> PluginResult {
        self.enabled = context
            .configuration()
            .get("enabled")
            .map(|value| value != "false")
            .unwrap_or(true);

        Ok(())
    }

    fn handle_event(
        &mut self,
        event: &PluginEvent,
        context: &mut PluginContext,
    ) -> PluginResult {
        if !self.enabled {
            return Ok(());
        }

        match event {
            PluginEvent::Custom { name, data } => {
                self.handle_custom_event(name, data, context)
            }

            PluginEvent::PaneCreated { .. }
            | PluginEvent::SessionCreated { .. }
            | PluginEvent::ConfigurationChanged => Ok(()),

            _ => Ok(()),
        }
    }

    fn shutdown(
        &mut self,
        _context: &mut PluginContext,
    ) -> PluginResult {
        self.enabled = false;
        Ok(())
    }
}

/// Native plugin entry point.
#[no_mangle]
pub extern "C" fn conduit_plugin_create() -> *mut dyn Plugin {
    Box::into_raw(Box::new(GitPlugin::new()))
}
