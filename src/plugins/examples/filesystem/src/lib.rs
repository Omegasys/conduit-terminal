use conduit_plugin_api::{
    Plugin,
    PluginContext,
    PluginEvent,
    PluginId,
    PluginResult,
};

pub struct FilesystemPlugin {
    id: PluginId,
    enabled: bool,
}

impl FilesystemPlugin {
    pub fn new() -> Self {
        Self {
            id: PluginId::new("example.filesystem"),
            enabled: true,
        }
    }
}

impl Default for FilesystemPlugin {
    fn default() -> Self {
        Self::new()
    }
}

impl Plugin for FilesystemPlugin {
    fn id(&self) -> &PluginId {
        &self.id
    }

    fn initialize(&mut self, context: &mut PluginContext) -> PluginResult {
        self.enabled = context
            .configuration()
            .get("enabled")
            .map(|value| value != "false")
            .unwrap_or(true);

        if !context.has_permission(
            conduit_plugin_api::Permission::ReadFilesystem,
        ) {
            return Err(
                "Filesystem example requires ReadFilesystem permission"
                    .to_string(),
            );
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
            context.emit_event(PluginEvent::Custom {
                name: "example.filesystem.ready".to_string(),
                data: Vec::new(),
            })?;
        }

        Ok(())
    }

    fn shutdown(&mut self, _context: &mut PluginContext) -> PluginResult {
        Ok(())
    }
}

#[no_mangle]
pub extern "C" fn conduit_plugin_create() -> *mut dyn Plugin {
    Box::into_raw(Box::new(FilesystemPlugin::new()))
}
