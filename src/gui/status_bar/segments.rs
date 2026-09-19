use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StatusSegmentAlignment {
    Left,
    Center,
    Right,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StatusSegmentKind {
    Text,
    Icon,
    Progress,
    Separator,
    Spacer,
    Custom,
}

#[derive(Debug, Clone)]
pub struct StatusSegment {
    id: String,
    label: String,
    value: String,
    icon: Option<String>,
    kind: StatusSegmentKind,
    alignment: StatusSegmentAlignment,
    visible: bool,
    enabled: bool,
    priority: i32,
    progress: Option<f32>,
}

impl StatusSegment {
    pub fn new(
        id: impl Into<String>,
        label: impl Into<String>,
        value: impl Into<String>,
    ) -> Self {
        Self {
            id: id.into(),
            label: label.into(),
            value: value.into(),
            icon: None,
            kind: StatusSegmentKind::Text,
            alignment: StatusSegmentAlignment::Left,
            visible: true,
            enabled: true,
            priority: 0,
            progress: None,
        }
    }

    pub fn id(&self) -> &str {
        &self.id
    }

    pub fn label(&self) -> &str {
        &self.label
    }

    pub fn value(&self) -> &str {
        &self.value
    }

    pub fn icon(&self) -> Option<&str> {
        self.icon.as_deref()
    }

    pub fn kind(&self) -> StatusSegmentKind {
        self.kind
    }

    pub fn alignment(&self) -> StatusSegmentAlignment {
        self.alignment
    }

    pub fn is_visible(&self) -> bool {
        self.visible
    }

    pub fn is_enabled(&self) -> bool {
        self.enabled
    }

    pub fn priority(&self) -> i32 {
        self.priority
    }

    pub fn progress(&self) -> Option<f32> {
        self.progress
    }

    pub fn set_label(&mut self, label: impl Into<String>) {
        self.label = label.into();
    }

    pub fn set_value(&mut self, value: impl Into<String>) {
        self.value = value.into();
    }

    pub fn set_icon(&mut self, icon: Option<String>) {
        self.icon = icon;
    }

    pub fn set_kind(&mut self, kind: StatusSegmentKind) {
        self.kind = kind;
    }

    pub fn set_alignment(&mut self, alignment: StatusSegmentAlignment) {
        self.alignment = alignment;
    }

    pub fn set_visible(&mut self, visible: bool) {
        self.visible = visible;
    }

    pub fn set_enabled(&mut self, enabled: bool) {
        self.enabled = enabled;
    }

    pub fn set_priority(&mut self, priority: i32) {
        self.priority = priority;
    }

    pub fn set_progress(&mut self, progress: Option<f32>) {
        self.progress = progress.map(|value| value.clamp(0.0, 1.0));
    }

    pub fn can_display(&self) -> bool {
        self.visible && self.enabled
    }
}

#[derive(Debug, Default)]
pub struct StatusSegmentManager {
    segments: HashMap<String, StatusSegment>,
}

impl StatusSegmentManager {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add(&mut self, segment: StatusSegment) -> Option<StatusSegment> {
        self.segments.insert(segment.id().to_string(), segment)
    }

    pub fn remove(&mut self, id: &str) -> Option<StatusSegment> {
        self.segments.remove(id)
    }

    pub fn get(&self, id: &str) -> Option<&StatusSegment> {
        self.segments.get(id)
    }

    pub fn get_mut(&mut self, id: &str) -> Option<&mut StatusSegment> {
        self.segments.get_mut(id)
    }

    pub fn contains(&self, id: &str) -> bool {
        self.segments.contains_key(id)
    }

    pub fn iter(&self) -> impl Iterator<Item = &StatusSegment> {
        self.segments.values()
    }

    pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut StatusSegment> {
        self.segments.values_mut()
    }

    pub fn visible(&self) -> Vec<&StatusSegment> {
        let mut segments: Vec<_> = self
            .segments
            .values()
            .filter(|segment| segment.can_display())
            .collect();

        segments.sort_by_key(|segment| segment.priority());

        segments
    }

    pub fn clear(&mut self) {
        self.segments.clear();
    }

    pub fn len(&self) -> usize {
        self.segments.len()
    }

    pub fn is_empty(&self) -> bool {
        self.segments.is_empty()
    }
}
