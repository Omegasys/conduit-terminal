#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PluginCommandAction {
    List,
    Show,
    Enable,
    Disable,
    Load,
    Unload,
    Reload,
    Install,
    Remove,
    Update,
    Permissions,
}

impl PluginCommandAction {
    pub fn name(&self) -> &'static str {
        match self {
            Self::List => "plugin-list",
            Self::Show => "plugin-show",
            Self::Enable => "plugin-enable",
            Self::Disable => "plugin-disable",
            Self::Load => "plugin-load",
            Self::Unload => "plugin-unload",
            Self::Reload => "plugin-reload",
            Self::Install => "plugin-install",
            Self::Remove => "plugin-remove",
            Self::Update => "plugin-update",
            Self::Permissions => "plugin-permissions",
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PluginCommand {
    action: PluginCommandAction,
    plugin: Option<String>,
    source: Option<String>,
}

impl PluginCommand {
    pub fn new(action: PluginCommandAction) -> Self {
        Self {
            action,
            plugin: None,
            source: None,
        }
    }

    pub fn with_plugin(
        action: PluginCommandAction,
        plugin: impl Into<String>,
    ) -> Self {
        Self {
            action,
            plugin: Some(plugin.into()),
            source: None,
        }
    }

    pub fn with_source(
        action: PluginCommandAction,
        source: impl Into<String>,
    ) -> Self {
        Self {
            action,
            plugin: None,
            source: Some(source.into()),
        }
    }

    pub fn action(&self) -> PluginCommandAction {
        self.action
    }

    pub fn plugin(&self) -> Option<&str> {
        self.plugin.as_deref()
    }

    pub fn source(&self) -> Option<&str> {
        self.source.as_deref()
    }

    pub fn set_plugin(&mut self, plugin: impl Into<String>) {
        self.plugin = Some(plugin.into());
    }

    pub fn set_source(&mut self, source: impl Into<String>) {
        self.source = Some(source.into());
    }

    pub fn is_targeted(&self) -> bool {
        self.plugin.is_some()
    }

    pub fn requires_plugin(&self) -> bool {
        matches!(
            self.action,
            PluginCommandAction::Show
                | PluginCommandAction::Enable
                | PluginCommandAction::Disable
                | PluginCommandAction::Load
                | PluginCommandAction::Unload
                | PluginCommandAction::Reload
                | PluginCommandAction::Remove
                | PluginCommandAction::Update
                | PluginCommandAction::Permissions
        )
    }
}
