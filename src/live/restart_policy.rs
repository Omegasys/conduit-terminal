use crate::resources::ResourceKind;

use super::hot_reload::HotReloadAction;

/// Determines how aggressively a component should be restarted.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RestartLevel {
    /// No restart is necessary.
    None,

    /// Reload the affected resource in place.
    Reload,

    /// Restart only the component directly associated with the resource.
    Component,

    /// Restart the component and its dependents.
    Dependents,

    /// Restart the complete live runtime.
    Full,
}

/// Policy used to determine restart behavior.
#[derive(Debug, Clone)]
pub struct RestartPolicy {
    restart_plugins: RestartLevel,
    restart_extensions: RestartLevel,
    restart_scripts: RestartLevel,
    restart_configuration: RestartLevel,
    restart_themes: RestartLevel,
    restart_workspaces: RestartLevel,
    restart_layouts: RestartLevel,
    restart_keybindings: RestartLevel,
    restart_unknown: RestartLevel,
}

impl Default for RestartPolicy {
    fn default() -> Self {
        Self {
            restart_plugins: RestartLevel::Component,
            restart_extensions: RestartLevel::Component,
            restart_scripts: RestartLevel::Dependents,
            restart_configuration: RestartLevel::Reload,
            restart_themes: RestartLevel::Reload,
            restart_workspaces: RestartLevel::Reload,
            restart_layouts: RestartLevel::Reload,
            restart_keybindings: RestartLevel::Reload,
            restart_unknown: RestartLevel::None,
        }
    }
}

impl RestartPolicy {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn level_for(&self, kind: ResourceKind) -> RestartLevel {
        match kind {
            ResourceKind::Plugin => self.restart_plugins,
            ResourceKind::Extension => self.restart_extensions,
            ResourceKind::Script => self.restart_scripts,
            ResourceKind::Configuration => self.restart_configuration,
            ResourceKind::Theme => self.restart_themes,
            ResourceKind::Workspace => self.restart_workspaces,
            ResourceKind::Layout => self.restart_layouts,
            ResourceKind::Keybinding => self.restart_keybindings,
            ResourceKind::Profile => RestartLevel::Reload,
            ResourceKind::Unknown => self.restart_unknown,
        }
    }

    pub fn action_for(&self, kind: ResourceKind) -> HotReloadAction {
        match self.level_for(kind) {
            RestartLevel::None => HotReloadAction::Ignore,
            RestartLevel::Reload => HotReloadAction::Reload,
            RestartLevel::Component => HotReloadAction::RestartComponent,
            RestartLevel::Dependents => HotReloadAction::ReloadDependents,
            RestartLevel::Full => HotReloadAction::RestartComponent,
        }
    }

    pub fn set_plugin_level(&mut self, level: RestartLevel) {
        self.restart_plugins = level;
    }

    pub fn set_extension_level(&mut self, level: RestartLevel) {
        self.restart_extensions = level;
    }

    pub fn set_script_level(&mut self, level: RestartLevel) {
        self.restart_scripts = level;
    }

    pub fn set_configuration_level(&mut self, level: RestartLevel) {
        self.restart_configuration = level;
    }

    pub fn set_theme_level(&mut self, level: RestartLevel) {
        self.restart_themes = level;
    }

    pub fn set_workspace_level(&mut self, level: RestartLevel) {
        self.restart_workspaces = level;
    }

    pub fn set_layout_level(&mut self, level: RestartLevel) {
        self.restart_layouts = level;
    }

    pub fn set_keybinding_level(&mut self, level: RestartLevel) {
        self.restart_keybindings = level;
    }

    pub fn set_unknown_level(&mut self, level: RestartLevel) {
        self.restart_unknown = level;
    }

    pub fn is_reload_only(&self, kind: ResourceKind) -> bool {
        self.level_for(kind) == RestartLevel::Reload
    }

    pub fn requires_restart(&self, kind: ResourceKind) -> bool {
        matches!(
            self.level_for(kind),
            RestartLevel::Component | RestartLevel::Dependents | RestartLevel::Full
        )
    }
}
