use std::collections::BTreeMap;

use crate::resources::{Profile, ProfileValue, ResourceId};

/// GUI representation of a profile.
#[derive(Debug, Clone)]
pub struct GuiProfile {
    resource_id: Option<ResourceId>,
    name: String,
    description: String,
    values: BTreeMap<String, ProfileValue>,
    selected: bool,
    enabled: bool,
}

impl GuiProfile {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            resource_id: None,
            name: name.into(),
            description: String::new(),
            values: BTreeMap::new(),
            selected: false,
            enabled: true,
        }
    }

    pub fn from_resource(profile: &Profile) -> Self {
        let mut gui_profile = Self::new(profile.name());

        gui_profile.resource_id = Some(profile.id());
        gui_profile.description = profile.description().to_string();
        gui_profile.values = profile.values().clone();
        gui_profile.enabled = profile.is_enabled();

        gui_profile
    }

    pub fn resource_id(&self) -> Option<ResourceId> {
        self.resource_id
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn description(&self) -> &str {
        &self.description
    }

    pub fn values(&self) -> &BTreeMap<String, ProfileValue> {
        &self.values
    }

    pub fn get(&self, key: &str) -> Option<&ProfileValue> {
        self.values.get(key)
    }

    pub fn selected(&self) -> bool {
        self.selected
    }

    pub fn enabled(&self) -> bool {
        self.enabled
    }

    pub fn set_description(&mut self, description: impl Into<String>) {
        self.description = description.into();
    }

    pub fn set(&mut self, key: impl Into<String>, value: ProfileValue) {
        self.values.insert(key.into(), value);
    }

    pub fn remove(&mut self, key: &str) -> Option<ProfileValue> {
        self.values.remove(key)
    }

    pub fn set_selected(&mut self, selected: bool) {
        self.selected = selected;
    }

    pub fn set_enabled(&mut self, enabled: bool) {
        self.enabled = enabled;
    }
}

/// Manages profiles presented by the GUI.
#[derive(Debug, Default)]
pub struct GuiProfileManager {
    profiles: Vec<GuiProfile>,
    active: Option<String>,
}

impl GuiProfileManager {
    pub fn new() -> Self {
        Self {
            profiles: Vec::new(),
            active: None,
        }
    }

    pub fn add(&mut self, profile: GuiProfile) -> bool {
        if self.profiles.iter().any(|p| p.name() == profile.name()) {
            return false;
        }

        self.profiles.push(profile);
        true
    }

    pub fn add_from_resource(&mut self, profile: &Profile) -> bool {
        self.add(GuiProfile::from_resource(profile))
    }

    pub fn remove(&mut self, name: &str) -> Option<GuiProfile> {
        let position = self.profiles.iter().position(|p| p.name() == name)?;

        let removed = self.profiles.remove(position);

        if self.active.as_deref() == Some(name) {
            self.active = None;
        }

        Some(removed)
    }

    pub fn get(&self, name: &str) -> Option<&GuiProfile> {
        self.profiles.iter().find(|p| p.name() == name)
    }

    pub fn get_mut(&mut self, name: &str) -> Option<&mut GuiProfile> {
        self.profiles.iter_mut().find(|p| p.name() == name)
    }

    pub fn activate(&mut self, name: &str) -> bool {
        if !self.profiles.iter().any(|p| p.name() == name) {
            return false;
        }

        for profile in &mut self.profiles {
            profile.set_selected(profile.name() == name);
        }

        self.active = Some(name.to_string());
        true
    }

    pub fn active(&self) -> Option<&str> {
        self.active.as_deref()
    }

    pub fn iter(&self) -> impl Iterator<Item = &GuiProfile> {
        self.profiles.iter()
    }

    pub fn len(&self) -> usize {
        self.profiles.len()
    }

    pub fn is_empty(&self) -> bool {
        self.profiles.is_empty()
    }

    pub fn clear(&mut self) {
        self.profiles.clear();
        self.active = None;
    }
}
