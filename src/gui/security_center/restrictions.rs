use std::collections::BTreeMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum RestrictionKind {
    Clipboard,
    Hyperlinks,
    ExternalCommands,
    FileAccess,
    NetworkAccess,
    PluginAccess,
    PluginNetwork,
    PluginFilesystem,
    PluginProcesses,
    TerminalControl,
    ConfigurationChanges,
    UiExtensions,
}

#[derive(Debug, Clone)]
pub struct Restriction {
    kind: RestrictionKind,
    name: String,
    description: String,
    enabled: bool,
}

impl Restriction {
    pub fn new(
        kind: RestrictionKind,
        name: impl Into<String>,
        description: impl Into<String>,
    ) -> Self {
        Self {
            kind,
            name: name.into(),
            description: description.into(),
            enabled: false,
        }
    }

    pub fn kind(&self) -> RestrictionKind {
        self.kind
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn description(&self) -> &str {
        &self.description
    }

    pub fn enabled(&self) -> bool {
        self.enabled
    }

    pub fn set_enabled(&mut self, enabled: bool) {
        self.enabled = enabled;
    }

    pub fn enable(&mut self) {
        self.enabled = true;
    }

    pub fn disable(&mut self) {
        self.enabled = false;
    }
}

#[derive(Debug, Default)]
pub struct RestrictionManager {
    restrictions: BTreeMap<RestrictionKind, Restriction>,
}

impl RestrictionManager {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn register(&mut self, restriction: Restriction) {
        self.restrictions
            .insert(restriction.kind(), restriction);
    }

    pub fn get(&self, kind: RestrictionKind) -> Option<&Restriction> {
        self.restrictions.get(&kind)
    }

    pub fn get_mut(&mut self, kind: RestrictionKind) -> Option<&mut Restriction> {
        self.restrictions.get_mut(&kind)
    }

    pub fn set_enabled(&mut self, kind: RestrictionKind, enabled: bool) -> bool {
        let Some(restriction) = self.restrictions.get_mut(&kind) else {
            return false;
        };

        restriction.set_enabled(enabled);
        true
    }

    pub fn enable(&mut self, kind: RestrictionKind) -> bool {
        self.set_enabled(kind, true)
    }

    pub fn disable(&mut self, kind: RestrictionKind) -> bool {
        self.set_enabled(kind, false)
    }

    pub fn iter(&self) -> impl Iterator<Item = &Restriction> {
        self.restrictions.values()
    }

    pub fn active(&self) -> impl Iterator<Item = &Restriction> {
        self.restrictions
            .values()
            .filter(|restriction| restriction.enabled())
    }

    pub fn active_count(&self) -> usize {
        self.restrictions
            .values()
            .filter(|restriction| restriction.enabled())
            .count()
    }

    pub fn len(&self) -> usize {
        self.restrictions.len()
    }

    pub fn is_empty(&self) -> bool {
        self.restrictions.is_empty()
    }

    pub fn clear(&mut self) {
        self.restrictions.clear();
    }

    pub fn enable_all(&mut self) {
        for restriction in self.restrictions.values_mut() {
            restriction.enable();
        }
    }

    pub fn disable_all(&mut self) {
        for restriction in self.restrictions.values_mut() {
            restriction.disable();
        }
    }
}

impl Default for Restriction {
    fn default() -> Self {
        Self::new(
            RestrictionKind::Clipboard,
            "Clipboard",
            "Restricts clipboard access.",
        )
    }
}
