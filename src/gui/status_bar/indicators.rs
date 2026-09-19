use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum IndicatorKind {
    Connection,
    Shell,
    Session,
    Recording,
    Synchronization,
    Security,
    Network,
    Resource,
    Configuration,
    Update,
    Warning,
    Error,
    Custom,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StatusIndicatorState {
    Unknown,
    Inactive,
    Active,
    Warning,
    Error,
    Disabled,
}

#[derive(Debug, Clone)]
pub struct StatusIndicator {
    id: String,
    label: String,
    tooltip: String,
    icon: Option<String>,
    kind: IndicatorKind,
    state: StatusIndicatorState,
    visible: bool,
    enabled: bool,
    value: Option<String>,
}

impl StatusIndicator {
    pub fn new(
        id: impl Into<String>,
        label: impl Into<String>,
        kind: IndicatorKind,
    ) -> Self {
        Self {
            id: id.into(),
            label: label.into(),
            tooltip: String::new(),
            icon: None,
            kind,
            state: StatusIndicatorState::Unknown,
            visible: true,
            enabled: true,
            value: None,
        }
    }

    pub fn id(&self) -> &str {
        &self.id
    }

    pub fn label(&self) -> &str {
        &self.label
    }

    pub fn tooltip(&self) -> &str {
        &self.tooltip
    }

    pub fn icon(&self) -> Option<&str> {
        self.icon.as_deref()
    }

    pub fn kind(&self) -> IndicatorKind {
        self.kind
    }

    pub fn state(&self) -> StatusIndicatorState {
        self.state
    }

    pub fn value(&self) -> Option<&str> {
        self.value.as_deref()
    }

    pub fn is_visible(&self) -> bool {
        self.visible
    }

    pub fn is_enabled(&self) -> bool {
        self.enabled
    }

    pub fn set_label(&mut self, label: impl Into<String>) {
        self.label = label.into();
    }

    pub fn set_tooltip(&mut self, tooltip: impl Into<String>) {
        self.tooltip = tooltip.into();
    }

    pub fn set_icon(&mut self, icon: Option<String>) {
        self.icon = icon;
    }

    pub fn set_state(&mut self, state: StatusIndicatorState) {
        self.state = state;
    }

    pub fn set_value(&mut self, value: Option<String>) {
        self.value = value;
    }

    pub fn set_visible(&mut self, visible: bool) {
        self.visible = visible;
    }

    pub fn set_enabled(&mut self, enabled: bool) {
        self.enabled = enabled;

        if !enabled {
            self.state = StatusIndicatorState::Disabled;
        }
    }

    pub fn activate(&mut self) {
        self.enabled = true;
        self.state = StatusIndicatorState::Active;
    }

    pub fn deactivate(&mut self) {
        self.state = StatusIndicatorState::Inactive;
    }

    pub fn warn(&mut self) {
        self.state = StatusIndicatorState::Warning;
    }

    pub fn error(&mut self) {
        self.state = StatusIndicatorState::Error;
    }

    pub fn reset(&mut self) {
        self.state = StatusIndicatorState::Unknown;
    }

    pub fn can_display(&self) -> bool {
        self.visible && self.enabled
    }
}

#[derive(Debug, Default)]
pub struct StatusIndicatorManager {
    indicators: HashMap<String, StatusIndicator>,
}

impl StatusIndicatorManager {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add(&mut self, indicator: StatusIndicator) -> Option<StatusIndicator> {
        self.indicators
            .insert(indicator.id().to_string(), indicator)
    }

    pub fn remove(&mut self, id: &str) -> Option<StatusIndicator> {
        self.indicators.remove(id)
    }

    pub fn get(&self, id: &str) -> Option<&StatusIndicator> {
        self.indicators.get(id)
    }

    pub fn get_mut(&mut self, id: &str) -> Option<&mut StatusIndicator> {
        self.indicators.get_mut(id)
    }

    pub fn contains(&self, id: &str) -> bool {
        self.indicators.contains_key(id)
    }

    pub fn iter(&self) -> impl Iterator<Item = &StatusIndicator> {
        self.indicators.values()
    }

    pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut StatusIndicator> {
        self.indicators.values_mut()
    }

    pub fn visible(&self) -> Vec<&StatusIndicator> {
        self.indicators
            .values()
            .filter(|indicator| indicator.can_display())
            .collect()
    }

    pub fn by_kind(&self, kind: IndicatorKind) -> Vec<&StatusIndicator> {
        self.indicators
            .values()
            .filter(|indicator| {
                indicator.kind() == kind && indicator.can_display()
            })
            .collect()
    }

    pub fn clear(&mut self) {
        self.indicators.clear();
    }

    pub fn len(&self) -> usize {
        self.indicators.len()
    }

    pub fn is_empty(&self) -> bool {
        self.indicators.is_empty()
    }
}
