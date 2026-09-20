use std::collections::BTreeMap;

use super::permissions::{PermissionDecision, PermissionKind};
use super::restrictions::RestrictionKind;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SecurityProfileId(u64);

impl SecurityProfileId {
    pub fn new(value: u64) -> Self {
        Self(value)
    }

    pub fn value(&self) -> u64 {
        self.0
    }
}

#[derive(Debug, Clone)]
pub struct SecurityProfile {
    id: SecurityProfileId,
    name: String,
    description: String,
    permissions: BTreeMap<PermissionKind, PermissionDecision>,
    restrictions: Vec<RestrictionKind>,
    safe_mode: bool,
}

impl SecurityProfile {
    pub fn new(
        id: SecurityProfileId,
        name: impl Into<String>,
        description: impl Into<String>,
    ) -> Self {
        Self {
            id,
            name: name.into(),
            description: description.into(),
            permissions: BTreeMap::new(),
            restrictions: Vec::new(),
            safe_mode: false,
        }
    }

    pub fn id(&self) -> SecurityProfileId {
        self.id
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn description(&self) -> &str {
        &self.description
    }

    pub fn safe_mode(&self) -> bool {
        self.safe_mode
    }

    pub fn set_safe_mode(&mut self, enabled: bool) {
        self.safe_mode = enabled;
    }

    pub fn set_permission(
        &mut self,
        permission: PermissionKind,
        decision: PermissionDecision,
    ) {
        self.permissions.insert(permission, decision);
    }

    pub fn permission(
        &self,
        permission: PermissionKind,
    ) -> PermissionDecision {
        self.permissions
            .get(&permission)
            .copied()
            .unwrap_or(PermissionDecision::Ask)
    }

    pub fn permissions(
        &self,
    ) -> impl Iterator<Item = (&PermissionKind, &PermissionDecision)> {
        self.permissions.iter()
    }

    pub fn add_restriction(&mut self, restriction: RestrictionKind) {
        if !self.restrictions.contains(&restriction) {
            self.restrictions.push(restriction);
        }
    }

    pub fn remove_restriction(&mut self, restriction: RestrictionKind) {
        self.restrictions.retain(|item| *item != restriction);
    }

    pub fn restrictions(&self) -> &[RestrictionKind] {
        &self.restrictions
    }
}

#[derive(Debug, Default)]
pub struct SecurityProfileManager {
    profiles: BTreeMap<SecurityProfileId, SecurityProfile>,
    active: Option<SecurityProfileId>,
    next_id: u64,
}

impl SecurityProfileManager {
    pub fn new() -> Self {
        Self {
            next_id: 1,
            ..Default::default()
        }
    }

    pub fn create(
        &mut self,
        name: impl Into<String>,
        description: impl Into<String>,
    ) -> SecurityProfileId {
        let id = SecurityProfileId::new(self.next_id);
        self.next_id += 1;

        let profile = SecurityProfile::new(id, name, description);
        self.profiles.insert(id, profile);

        id
    }

    pub fn add(&mut self, profile: SecurityProfile) {
        self.profiles.insert(profile.id(), profile);
    }

    pub fn get(&self, id: SecurityProfileId) -> Option<&SecurityProfile> {
        self.profiles.get(&id)
    }

    pub fn get_mut(
        &mut self,
        id: SecurityProfileId,
    ) -> Option<&mut SecurityProfile> {
        self.profiles.get_mut(&id)
    }

    pub fn activate(&mut self, id: SecurityProfileId) -> bool {
        if !self.profiles.contains_key(&id) {
            return false;
        }

        self.active = Some(id);
        true
    }

    pub fn deactivate(&mut self) {
        self.active = None;
    }

    pub fn active(&self) -> Option<&SecurityProfile> {
        self.active.and_then(|id| self.profiles.get(&id))
    }

    pub fn active_id(&self) -> Option<SecurityProfileId> {
        self.active
    }

    pub fn iter(&self) -> impl Iterator<Item = &SecurityProfile> {
        self.profiles.values()
    }

    pub fn remove(&mut self, id: SecurityProfileId) -> Option<SecurityProfile> {
        if self.active == Some(id) {
            self.active = None;
        }

        self.profiles.remove(&id)
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
