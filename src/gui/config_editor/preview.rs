use crate::config_engine::{ConfigDiff, ConfigError, ConfigSerializer, ConfigValue};
use std::collections::BTreeMap;

/// Type of change displayed in the preview.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PreviewChangeKind {
    Added,
    Removed,
    Modified,
}

/// One configuration change in the preview.
#[derive(Debug, Clone)]
pub struct PreviewChange {
    key: String,
    kind: PreviewChangeKind,
    old_value: Option<ConfigValue>,
    new_value: Option<ConfigValue>,
}

impl PreviewChange {
    pub fn new(
        key: impl Into<String>,
        kind: PreviewChangeKind,
        old_value: Option<ConfigValue>,
        new_value: Option<ConfigValue>,
    ) -> Self {
        Self {
            key: key.into(),
            kind,
            old_value,
            new_value,
        }
    }

    pub fn key(&self) -> &str {
        &self.key
    }

    pub fn kind(&self) -> PreviewChangeKind {
        self.kind
    }

    pub fn old_value(&self) -> Option<&ConfigValue> {
        self.old_value.as_ref()
    }

    pub fn new_value(&self) -> Option<&ConfigValue> {
        self.new_value.as_ref()
    }
}

/// Complete preview of pending configuration changes.
#[derive(Debug, Clone, Default)]
pub struct ConfigPreview {
    changes: Vec<PreviewChange>,
    valid: bool,
    error: Option<String>,
}

impl ConfigPreview {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn from_texts(
        original: &str,
        modified: &str,
    ) -> Result<Self, ConfigError> {
        let original_values = ConfigSerializer::from_toml(original)?;
        let modified_values = ConfigSerializer::from_toml(modified)?;

        Ok(Self::from_values(
            &original_values,
            &modified_values,
        ))
    }

    pub fn from_values(
        original: &BTreeMap<String, ConfigValue>,
        modified: &BTreeMap<String, ConfigValue>,
    ) -> Self {
        let diff = ConfigDiff::from_maps(original, modified);
        let mut preview = Self::new();

        for change in diff.changes() {
            let preview_change = match change {
                crate::config_engine::ConfigChange::Added { key, new } => {
                    PreviewChange::new(
                        key,
                        PreviewChangeKind::Added,
                        None,
                        Some(new.clone()),
                    )
                }

                crate::config_engine::ConfigChange::Removed { key, old } => {
                    PreviewChange::new(
                        key,
                        PreviewChangeKind::Removed,
                        Some(old.clone()),
                        None,
                    )
                }

                crate::config_engine::ConfigChange::Modified {
                    key,
                    old,
                    new,
                } => PreviewChange::new(
                    key,
                    PreviewChangeKind::Modified,
                    Some(old.clone()),
                    Some(new.clone()),
                ),
            };

            preview.changes.push(preview_change);
        }

        preview.valid = true;
        preview
    }

    pub fn invalid(error: impl Into<String>) -> Self {
        Self {
            changes: Vec::new(),
            valid: false,
            error: Some(error.into()),
        }
    }

    pub fn changes(&self) -> &[PreviewChange] {
        &self.changes
    }

    pub fn added(&self) -> impl Iterator<Item = &PreviewChange> {
        self.changes
            .iter()
            .filter(|change| change.kind == PreviewChangeKind::Added)
    }

    pub fn removed(&self) -> impl Iterator<Item = &PreviewChange> {
        self.changes
            .iter()
            .filter(|change| change.kind == PreviewChangeKind::Removed)
    }

    pub fn modified(&self) -> impl Iterator<Item = &PreviewChange> {
        self.changes
            .iter()
            .filter(|change| change.kind == PreviewChangeKind::Modified)
    }

    pub fn is_valid(&self) -> bool {
        self.valid
    }

    pub fn error(&self) -> Option<&str> {
        self.error.as_deref()
    }

    pub fn is_empty(&self) -> bool {
        self.changes.is_empty()
    }

    pub fn len(&self) -> usize {
        self.changes.len()
    }
}
