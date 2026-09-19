use std::collections::{BTreeMap, BTreeSet};

use super::ConfigValue;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ConfigChange {
    Added {
        key: String,
        value: ConfigValue,
    },
    Removed {
        key: String,
        old_value: ConfigValue,
    },
    Modified {
        key: String,
        old_value: ConfigValue,
        new_value: ConfigValue,
    },
}

impl ConfigChange {
    pub fn key(&self) -> &str {
        match self {
            Self::Added { key, .. }
            | Self::Removed { key, .. }
            | Self::Modified { key, .. } => key,
        }
    }

    pub fn old_value(&self) -> Option<&ConfigValue> {
        match self {
            Self::Added { .. } => None,
            Self::Removed { old_value, .. } => Some(old_value),
            Self::Modified { old_value, .. } => Some(old_value),
        }
    }

    pub fn new_value(&self) -> Option<&ConfigValue> {
        match self {
            Self::Added { value, .. } => Some(value),
            Self::Removed { .. } => None,
            Self::Modified { new_value, .. } => Some(new_value),
        }
    }
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct ConfigDiff {
    changes: Vec<ConfigChange>,
}

impl ConfigDiff {
    pub fn empty() -> Self {
        Self::default()
    }

    pub fn from_maps(
        old: &BTreeMap<String, ConfigValue>,
        new: &BTreeMap<String, ConfigValue>,
    ) -> Self {
        let keys: BTreeSet<String> = old.keys().chain(new.keys()).cloned().collect();

        let mut changes = Vec::new();

        for key in keys {
            match (old.get(&key), new.get(&key)) {
                (None, Some(value)) => changes.push(ConfigChange::Added {
                    key,
                    value: value.clone(),
                }),

                (Some(old_value), None) => changes.push(ConfigChange::Removed {
                    key,
                    old_value: old_value.clone(),
                }),

                (Some(old_value), Some(new_value)) if old_value != new_value => {
                    changes.push(ConfigChange::Modified {
                        key,
                        old_value: old_value.clone(),
                        new_value: new_value.clone(),
                    });
                }

                _ => {}
            }
        }

        Self { changes }
    }

    pub fn changes(&self) -> &[ConfigChange] {
        &self.changes
    }

    pub fn len(&self) -> usize {
        self.changes.len()
    }

    pub fn is_empty(&self) -> bool {
        self.changes.is_empty()
    }

    pub fn added(&self) -> impl Iterator<Item = &ConfigChange> {
        self.changes.iter().filter(|change| {
            matches!(change, ConfigChange::Added { .. })
        })
    }

    pub fn removed(&self) -> impl Iterator<Item = &ConfigChange> {
        self.changes.iter().filter(|change| {
            matches!(change, ConfigChange::Removed { .. })
        })
    }

    pub fn modified(&self) -> impl Iterator<Item = &ConfigChange> {
        self.changes.iter().filter(|change| {
            matches!(change, ConfigChange::Modified { .. })
        })
    }

    pub fn apply_to(
        &self,
        values: &mut BTreeMap<String, ConfigValue>,
    ) {
        for change in &self.changes {
            match change {
                ConfigChange::Added { key, value } => {
                    values.insert(key.clone(), value.clone());
                }

                ConfigChange::Removed { key, .. } => {
                    values.remove(key);
                }

                ConfigChange::Modified {
                    key,
                    new_value,
                    ..
                } => {
                    values.insert(key.clone(), new_value.clone());
                }
            }
        }
    }

    pub fn reverse(&self) -> Self {
        let changes = self
            .changes
            .iter()
            .map(|change| match change {
                ConfigChange::Added { key, value } => ConfigChange::Removed {
                    key: key.clone(),
                    old_value: value.clone(),
                },

                ConfigChange::Removed { key, old_value } => ConfigChange::Added {
                    key: key.clone(),
                    value: old_value.clone(),
                },

                ConfigChange::Modified {
                    key,
                    old_value,
                    new_value,
                } => ConfigChange::Modified {
                    key: key.clone(),
                    old_value: new_value.clone(),
                    new_value: old_value.clone(),
                },
            })
            .collect();

        Self { changes }
    }
}
