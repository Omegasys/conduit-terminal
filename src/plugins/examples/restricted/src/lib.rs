use conduit_plugin_api::{
    Permission,
    Plugin,
    PluginContext,
    PluginEvent,
    PluginId,
    PluginResult,
};

pub struct RestrictedPlugin {
    id: PluginId,
    enabled: bool,
}

impl RestrictedPlugin {
    pub fn new() -> Self {
        Self {
            id: PluginId::new("example.restricted"),
            enabled: true,
        }
    }
}

impl Default for RestrictedPlugin {
    fn default() -> Self {
        Self::new()
    }
}

impl Plugin for RestrictedPlugin {
    fn id(&self) -> &PluginId {
        &self.id
    }

    fn initialize(&mut self, context: &mut PluginContext) -> PluginResult {
        self.enabled = context
            .configuration()
            .get("enabled")
            .map(|value| value != "false")
            .unwrap_or(true);

        if !context.has_permission(Permission::AccessTerminal) {
            return Err(
                "Restricted plugin requires AccessTerminal permission"
                    .to_string(),
            );
        }

        if context.has_permission(Permission::ReadFilesystem) {
            return Err(
                "Restricted plugin unexpectedly has filesystem access"
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
            let filesystem_access = context.has_permission(
                Permission::ReadFilesystem,
            );

            let message = if filesystem_access {
                b"unexpected filesystem access".to_vec()
            } else {
                b"running with restricted permissions".to_vec()
            };

            context.emit_event(PluginEvent::Custom {
                name: "example.restricted.status".to_string(),
                data: message,
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
    Box::into_raw(Box::new(RestrictedPlugin::new()))
}
