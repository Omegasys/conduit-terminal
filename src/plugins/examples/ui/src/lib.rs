use conduit_plugin_api::{
    Plugin,
    PluginContext,
    PluginEvent,
    PluginId,
    PluginResult,
};

pub struct UiPlugin {
    id: PluginId,
    enabled: bool,
}

impl UiPlugin {
    pub fn new() -> Self {
        Self {
            id: PluginId::new("example.ui"),
            enabled: true,
        }
    }
}

impl Default for UiPlugin {
    fn default() -> Self {
        Self::new()
    }
}

impl Plugin for UiPlugin {
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

        let event_name = match event {
            PluginEvent::WindowCreated => "window.created",
            PluginEvent::WindowClosed => "window.closed",
            PluginEvent::TabCreated => "tab.created",
            PluginEvent::TabClosed => "tab.closed",
            PluginEvent::PaneCreated => "pane.created",
            PluginEvent::PaneClosed => "pane.closed",
            _ => return Ok(()),
        };

        context.emit_event(PluginEvent::Custom {
            name: format!("example.ui.{event_name}"),
            data: Vec::new(),
        })?;

        Ok(())
    }

    fn shutdown(&mut self, _context: &mut PluginContext) -> PluginResult {
        Ok(())
    }
}

#[no_mangle]
pub extern "C" fn conduit_plugin_create() -> *mut dyn Plugin {
    Box::into_raw(Box::new(UiPlugin::new()))
}
