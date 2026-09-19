use std::collections::HashMap;
use std::time::{Duration, Instant};

use crate::resources::{ResourceId, ResourceState};

/// Runtime state maintained for a live-reloadable resource.
#[derive(Debug, Clone)]
pub struct LiveResourceState {
    resource_id: ResourceId,
    state: ResourceState,
    generation: u64,
    last_change: Option<Instant>,
    last_reload: Option<Instant>,
    reload_count: u64,
    failure_count: u64,
    last_error: Option<String>,
}

impl LiveResourceState {
    pub fn new(resource_id: ResourceId) -> Self {
        Self {
            resource_id,
            state: ResourceState::Discovered,
            generation: 0,
            last_change: None,
            last_reload: None,
            reload_count: 0,
            failure_count: 0,
            last_error: None,
        }
    }

    pub fn resource_id(&self) -> ResourceId {
        self.resource_id
    }

    pub fn state(&self) -> &ResourceState {
        &self.state
    }

    pub fn generation(&self) -> u64 {
        self.generation
    }

    pub fn last_change(&self) -> Option<Instant> {
        self.last_change
    }

    pub fn last_reload(&self) -> Option<Instant> {
        self.last_reload
    }

    pub fn reload_count(&self) -> u64 {
        self.reload_count
    }

    pub fn failure_count(&self) -> u64 {
        self.failure_count
    }

    pub fn last_error(&self) -> Option<&str> {
        self.last_error.as_deref()
    }

    pub fn mark_changed(&mut self) {
        self.state = ResourceState::Modified;
        self.last_change = Some(Instant::now());
        self.generation = self.generation.saturating_add(1);
    }

    pub fn mark_loading(&mut self) {
        self.state = ResourceState::Loading;
        self.last_error = None;
    }

    pub fn mark_loaded(&mut self) {
        self.state = ResourceState::Loaded;
        self.last_reload = Some(Instant::now());
        self.reload_count = self.reload_count.saturating_add(1);
        self.last_error = None;
    }

    pub fn mark_invalid(&mut self, error: impl Into<String>) {
        self.state = ResourceState::Invalid;
        self.failure_count = self.failure_count.saturating_add(1);
        self.last_error = Some(error.into());
    }

    pub fn mark_disabled(&mut self) {
        self.state = ResourceState::Disabled;
    }

    pub fn mark_removed(&mut self) {
        self.state = ResourceState::Removed;
    }

    pub fn clear_error(&mut self) {
        self.last_error = None;
    }

    pub fn time_since_change(&self) -> Option<Duration> {
        self.last_change.map(|time| time.elapsed())
    }

    pub fn time_since_reload(&self) -> Option<Duration> {
        self.last_reload.map(|time| time.elapsed())
    }
}

/// Tracks live state for all resources known to the live subsystem.
#[derive(Debug, Default)]
pub struct LiveState {
    resources: HashMap<ResourceId, LiveResourceState>,
}

impl LiveState {
    pub fn new() -> Self {
        Self {
            resources: HashMap::new(),
        }
    }

    pub fn register(&mut self, resource_id: ResourceId) -> &mut LiveResourceState {
        self.resources
            .entry(resource_id)
            .or_insert_with(|| LiveResourceState::new(resource_id))
    }

    pub fn contains(&self, resource_id: ResourceId) -> bool {
        self.resources.contains_key(&resource_id)
    }

    pub fn get(&self, resource_id: ResourceId) -> Option<&LiveResourceState> {
        self.resources.get(&resource_id)
    }

    pub fn get_mut(&mut self, resource_id: ResourceId) -> Option<&mut LiveResourceState> {
        self.resources.get_mut(&resource_id)
    }

    pub fn unregister(&mut self, resource_id: ResourceId) -> Option<LiveResourceState> {
        self.resources.remove(&resource_id)
    }

    pub fn mark_changed(&mut self, resource_id: ResourceId) {
        self.register(resource_id).mark_changed();
    }

    pub fn mark_loading(&mut self, resource_id: ResourceId) {
        self.register(resource_id).mark_loading();
    }

    pub fn mark_loaded(&mut self, resource_id: ResourceId) {
        self.register(resource_id).mark_loaded();
    }

    pub fn mark_invalid(&mut self, resource_id: ResourceId, error: impl Into<String>) {
        self.register(resource_id).mark_invalid(error);
    }

    pub fn mark_removed(&mut self, resource_id: ResourceId) {
        self.register(resource_id).mark_removed();
    }

    pub fn iter(&self) -> impl Iterator<Item = (&ResourceId, &LiveResourceState)> {
        self.resources.iter()
    }

    pub fn len(&self) -> usize {
        self.resources.len()
    }

    pub fn is_empty(&self) -> bool {
        self.resources.is_empty()
    }

    pub fn clear(&mut self) {
        self.resources.clear();
    }
}
