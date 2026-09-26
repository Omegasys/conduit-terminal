use conduit_plugin_api::{
    Plugin,
    PluginContext,
    PluginEvent,
    PluginId,
    PluginResult,
};

pub struct CommandPlugin {
    id: PluginId,
    enabled: bool,
}

impl CommandPlugin {
    pub fn new() -> Self {
        Self {
            id: PluginId::new("example.command"),
            enabled: true,
        }
    }
}

impl Default for CommandPlugin {
    fn default() -> Self {
        Self::new()
    }
}

impl Plugin for CommandPlugin {
    fn id(&self) -> &PluginId {
        &self.id
    }

    fn initialize(&mut self, context: &mut PluginContext) -> PluginResult {
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

        if let PluginEvent::Custom { name, data } = event {
            if name == "command.execute" {
                context.emit_event(PluginEvent::Custom {
                    name: "example.command.received".to_string(),
                    data: data.clone(),
                })?;
            }
        }

        Ok(())
    }

    fn shutdown(&mut self, _context: &mut PluginContext) -> PluginResult {
        Ok(())
    }
}

#[no_mangle]
pub extern "C" fn conduit_plugin_create() -> *mut dyn Plugin {
    Box::into_raw(Box::new(CommandPlugin::new()))
}
