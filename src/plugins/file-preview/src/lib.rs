use conduit_plugin_api::{
    Plugin,
    PluginContext,
    PluginEvent,
    PluginId,
    PluginResult,
};

/// File preview plugin for Conduit.
pub struct FilePreviewPlugin {
    id: PluginId,
    enabled: bool,
}

impl Default for FilePreviewPlugin {
    fn default() -> Self {
        Self {
            id: PluginId::new("file-preview"),
            enabled: true,
        }
    }
}

impl FilePreviewPlugin {
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
            "file.preview.request" => Ok(()),
            "file.preview.refresh" => Ok(()),
            "file.preview.close" => Ok(()),
            _ => Ok(()),
        }
    }
}

impl Plugin for FilePreviewPlugin {
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

#[no_mangle]
pub extern "C" fn conduit_plugin_create() -> *mut dyn Plugin {
    Box::into_raw(Box::new(FilePreviewPlugin::new()))
}
