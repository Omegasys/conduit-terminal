#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PluginDisplayState {
    Discovered,
    Loading,
    Loaded,
    Disabled,
    Blocked,
    Error,
    Unloaded,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PluginTrustLevel {
    Unknown,
    Unverified,
    Verified,
    Trusted,
    Restricted,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PluginPermission {
    Filesystem,
    Network,
    Processes,
    Clipboard,
    TerminalControl,
    Configuration,
    UiExtension,
}

pub struct PluginDisplay {
    id: String,
    name: String,
    version: String,
    description: String,
    state: PluginDisplayState,
    trust: PluginTrustLevel,
    permissions: Vec<PluginPermission>,
    error: Option<String>,
    selected: bool,
}

impl PluginDisplay {
    pub fn new(
        id: impl Into<String>,
        name: impl Into<String>,
        version: impl Into<String>,
    ) -> Self {
        Self {
            id: id.into(),
            name: name.into(),
            version: version.into(),
            description: String::new(),
            state: PluginDisplayState::Discovered,
            trust: PluginTrustLevel::Unknown,
            permissions: Vec::new(),
            error: None,
            selected: false,
        }
    }

    pub fn id(&self) -> &str {
        &self.id
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn version(&self) -> &str {
        &self.version
    }

    pub fn description(&self) -> &str {
        &self.description
    }

    pub fn set_description(&mut self, description: impl Into<String>) {
        self.description = description.into();
    }

    pub fn state(&self) -> PluginDisplayState {
        self.state
    }

    pub fn set_state(&mut self, state: PluginDisplayState) {
        self.state = state;
    }

    pub fn trust(&self) -> PluginTrustLevel {
        self.trust
    }

    pub fn set_trust(&mut self, trust: PluginTrustLevel) {
        self.trust = trust;
    }

    pub fn permissions(&self) -> &[PluginPermission] {
        &self.permissions
    }

    pub fn grant_permission(&mut self, permission: PluginPermission) {
        if !self.permissions.contains(&permission) {
            self.permissions.push(permission);
        }
    }

    pub fn revoke_permission(&mut self, permission: PluginPermission) {
        self.permissions.retain(|p| *p != permission);
    }

    pub fn error(&self) -> Option<&str> {
        self.error.as_deref()
    }

    pub fn set_error(&mut self, error: Option<String>) {
        self.error = error;
    }

    pub fn selected(&self) -> bool {
        self.selected
    }

    pub fn set_selected(&mut self, selected: bool) {
        self.selected = selected;
    }
}

pub struct TuiPluginView {
    plugins: Vec<PluginDisplay>,
    selected: Option<String>,
    show_disabled: bool,
    show_blocked: bool,
    compact: bool,
}

impl Default for TuiPluginView {
    fn default() -> Self {
        Self::new()
    }
}

impl TuiPluginView {
    pub fn new() -> Self {
        Self {
            plugins: Vec::new(),
            selected: None,
            show_disabled: true,
            show_blocked: true,
            compact: false,
        }
    }

    pub fn add(&mut self, plugin: PluginDisplay) {
        let id = plugin.id().to_owned();

        if self.plugins.iter().any(|existing| existing.id() == id) {
            return;
        }

        self.plugins.push(plugin);

        if self.selected.is_none() {
            self.selected = Some(id);
        }

        self.refresh_selection();
    }

    pub fn remove(&mut self, id: &str) -> Option<PluginDisplay> {
        let index = self.plugins.iter().position(|plugin| plugin.id() == id)?;
        let removed = self.plugins.remove(index);

        if self.selected.as_deref() == Some(id) {
            self.selected = self.plugins.first().map(|plugin| plugin.id().to_owned());
        }

        self.refresh_selection();
        Some(removed)
    }

    pub fn get(&self, id: &str) -> Option<&PluginDisplay> {
        self.plugins.iter().find(|plugin| plugin.id() == id)
    }

    pub fn get_mut(&mut self, id: &str) -> Option<&mut PluginDisplay> {
        self.plugins.iter_mut().find(|plugin| plugin.id() == id)
    }

    pub fn select(&mut self, id: &str) -> bool {
        if self.get(id).is_none() {
            return false;
        }

        self.selected = Some(id.to_owned());
        self.refresh_selection();
        true
    }

    pub fn selected(&self) -> Option<&PluginDisplay> {
        self.selected.as_deref().and_then(|id| self.get(id))
    }

    pub fn plugins(&self) -> &[PluginDisplay] {
        &self.plugins
    }

    pub fn visible_plugins(&self) -> Vec<&PluginDisplay> {
        self.plugins
            .iter()
            .filter(|plugin| {
                match plugin.state() {
                    PluginDisplayState::Disabled => self.show_disabled,
                    PluginDisplayState::Blocked => self.show_blocked,
                    _ => true,
                }
            })
            .collect()
    }

    pub fn show_disabled(&self) -> bool {
        self.show_disabled
    }

    pub fn set_show_disabled(&mut self, show: bool) {
        self.show_disabled = show;
    }

    pub fn show_blocked(&self) -> bool {
        self.show_blocked
    }

    pub fn set_show_blocked(&mut self, show: bool) {
        self.show_blocked = show;
    }

    pub fn compact(&self) -> bool {
        self.compact
    }

    pub fn set_compact(&mut self, compact: bool) {
        self.compact = compact;
    }

    pub fn refresh_selection(&mut self) {
        let selected = self.selected.as_deref();

        for plugin in &mut self.plugins {
            plugin.set_selected(selected == Some(plugin.id()));
        }
    }

    pub fn loaded_count(&self) -> usize {
        self.plugins
            .iter()
            .filter(|plugin| plugin.state() == PluginDisplayState::Loaded)
            .count()
    }

    pub fn error_count(&self) -> usize {
        self.plugins
            .iter()
            .filter(|plugin| plugin.state() == PluginDisplayState::Error)
            .count()
    }

    pub fn len(&self) -> usize {
        self.plugins.len()
    }

    pub fn is_empty(&self) -> bool {
        self.plugins.is_empty()
    }

    pub fn clear(&mut self) {
        self.plugins.clear();
        self.selected = None;
    }
}
