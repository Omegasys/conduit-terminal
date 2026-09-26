use conduit_plugin_api::{
    Plugin,
    PluginContext,
    PluginEvent,
    PluginId,
    PluginResult,
};

pub struct BasicPlugin {
    id: PluginId,
    enabled: bool,
}

impl BasicPlugin {
    pub fn new() -> Self {
        Self {
            id: PluginId::new("example.basic"),
            enabled: true,
        }
    }
}

impl Default for BasicPlugin {
    fn default() -> Self {
        Self::new()
    }
}

impl Plugin for BasicPlugin {
    fn id(&self) -> &PluginId {
        &self.id
    }

    fn initialize(&mut self, context: &mut PluginContext) -> PluginResult {
        self.enabled = context
            .configuration()
            .get("enabled")
            .map(|value| value != "false")
            .unwrap_or(true);

        if self.enabled {
            context.emit_event(PluginEvent::Custom {
                name: "example.basic.started".to_string(),
                data: b"Basic plugin initialized".to_vec(),
            })?;
        }

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

        if matches!(event, PluginEvent::Startup) {
            let message = context
                .configuration()
                .get("message")
                .cloned()
                .unwrap_or_else(|| "Hello from Conduit".to_string());

            context.emit_event(PluginEvent::Custom {
                name: "example.basic.message".to_string(),
                data: message.into_bytes(),
            })?;
        }

        Ok(())
    }

    fn shutdown(&mut self, context: &mut PluginContext) -> PluginResult {
        if self.enabled {
            context.emit_event(PluginEvent::Custom {
                name: "example.basic.stopped".to_string(),
                data: Vec::new(),
            })?;
        }

        Ok(())
    }
}

#[no_mangle]
pub extern "C" fn conduit_plugin_create() -> *mut dyn Plugin {
    Box::into_raw(Box::new(BasicPlugin::new()))
}
