use std::collections::{HashMap, HashSet};
use std::time::{Duration, Instant};

use crate::resources::ResourceId;

use super::restart_policy::RestartLevel;

/// Current state of a component restart operation.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ComponentRestartState {
    Idle,
    Requested,
    Restarting,
    Completed,
    Failed,
}

/// A restart request for a component.
#[derive(Debug, Clone)]
pub struct ComponentRestartRequest {
    pub component: String,
    pub resource: ResourceId,
    pub level: RestartLevel,
    pub requested_at: Instant,
    pub reason: String,
}

impl ComponentRestartRequest {
    pub fn new(
        component: impl Into<String>,
        resource: ResourceId,
        level: RestartLevel,
        reason: impl Into<String>,
    ) -> Self {
        Self {
            component: component.into(),
            resource,
            level,
            requested_at: Instant::now(),
            reason: reason.into(),
        }
    }
}

/// Runtime record for a component.
#[derive(Debug, Clone)]
pub struct ComponentRestartRecord {
    component: String,
    state: ComponentRestartState,
    restart_count: u64,
    last_restart: Option<Instant>,
    last_error: Option<String>,
}

impl ComponentRestartRecord {
    pub fn new(component: impl Into<String>) -> Self {
        Self {
            component: component.into(),
            state: ComponentRestartState::Idle,
            restart_count: 0,
            last_restart: None,
            last_error: None,
        }
    }

    pub fn component(&self) -> &str {
        &self.component
    }

    pub fn state(&self) -> ComponentRestartState {
        self.state
    }

    pub fn restart_count(&self) -> u64 {
        self.restart_count
    }

    pub fn last_restart(&self) -> Option<Instant> {
        self.last_restart
    }

    pub fn last_error(&self) -> Option<&str> {
        self.last_error.as_deref()
    }

    pub fn mark_requested(&mut self) {
        self.state = ComponentRestartState::Requested;
        self.last_error = None;
    }

    pub fn mark_restarting(&mut self) {
        self.state = ComponentRestartState::Restarting;
    }

    pub fn mark_completed(&mut self) {
        self.state = ComponentRestartState::Completed;
        self.restart_count = self.restart_count.saturating_add(1);
        self.last_restart = Some(Instant::now());
        self.last_error = None;
    }

    pub fn mark_failed(&mut self, error: impl Into<String>) {
        self.state = ComponentRestartState::Failed;
        self.last_error = Some(error.into());
    }

    pub fn reset(&mut self) {
        self.state = ComponentRestartState::Idle;
        self.last_error = None;
    }
}

/// Coordinates restart requests without deciding how an individual
/// component is actually stopped or started.
///
/// The application/runtime layer can consume the pending requests and
/// perform the platform-specific restart operation.
#[derive(Debug)]
pub struct ComponentRestartManager {
    components: HashMap<String, ComponentRestartRecord>,
    pending: Vec<ComponentRestartRequest>,
    completed: HashSet<String>,
    cooldown: Duration,
}

impl Default for ComponentRestartManager {
    fn default() -> Self {
        Self::new()
    }
}

impl ComponentRestartManager {
    pub fn new() -> Self {
        Self {
            components: HashMap::new(),
            pending: Vec::new(),
            completed: HashSet::new(),
            cooldown: Duration::from_millis(250),
        }
    }

    pub fn register(&mut self, component: impl Into<String>) {
        let component = component.into();

        self.components
            .entry(component.clone())
            .or_insert_with(|| ComponentRestartRecord::new(component));
    }

    pub fn unregister(&mut self, component: &str) -> Option<ComponentRestartRecord> {
        self.components.remove(component)
    }

    pub fn request(&mut self, request: ComponentRestartRequest) -> bool {
        self.register(request.component.clone());

        if let Some(record) = self.components.get_mut(&request.component) {
            record.mark_requested();
        }

        if self.is_in_cooldown(&request.component) {
            return false;
        }

        self.pending.push(request);
        true
    }

    pub fn next_request(&mut self) -> Option<ComponentRestartRequest> {
        self.pending.pop()
    }

    pub fn pending(&self) -> &[ComponentRestartRequest] {
        &self.pending
    }

    pub fn mark_restarting(&mut self, component: &str) {
        if let Some(record) = self.components.get_mut(component) {
            record.mark_restarting();
        }
    }

    pub fn mark_completed(&mut self, component: &str) {
        if let Some(record) = self.components.get_mut(component) {
            record.mark_completed();
        }

        self.completed.insert(component.to_string());
    }

    pub fn mark_failed(&mut self, component: &str, error: impl Into<String>) {
        if let Some(record) = self.components.get_mut(component) {
            record.mark_failed(error);
        }
    }

    pub fn record(&self, component: &str) -> Option<&ComponentRestartRecord> {
        self.components.get(component)
    }

    pub fn components(&self) -> impl Iterator<Item = &ComponentRestartRecord> {
        self.components.values()
    }

    pub fn completed(&self, component: &str) -> bool {
        self.completed.contains(component)
    }

    pub fn clear_completed(&mut self) {
        self.completed.clear();
    }

    pub fn clear_pending(&mut self) {
        self.pending.clear();
    }

    pub fn set_cooldown(&mut self, cooldown: Duration) {
        self.cooldown = cooldown;
    }

    fn is_in_cooldown(&self, component: &str) -> bool {
        self.components
            .get(component)
            .and_then(ComponentRestartRecord::last_restart)
            .map(|last| last.elapsed() < self.cooldown)
            .unwrap_or(false)
    }

    pub fn pending_count(&self) -> usize {
        self.pending.len()
    }

    pub fn component_count(&self) -> usize {
        self.components.len()
    }
}
