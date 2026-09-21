use std::collections::HashMap;

use super::api::{Plugin, PluginContext, PluginEvent, PluginId};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LifecycleState {
    Discovered,
    Loaded,
    Initializing,
    Running,
    Stopping,
    Stopped,
    Failed,
}

pub struct LifecycleManager {
    states: HashMap<PluginId, LifecycleState>,
}

impl LifecycleManager {
    pub fn new() -> Self {
        Self {
            states: HashMap::new(),
        }
    }

    pub fn register(&mut self, id: PluginId) {
        self.states
            .entry(id)
            .or_insert(LifecycleState::Discovered);
    }

    pub fn state(&self, id: &PluginId) -> Option<LifecycleState> {
        self.states.get(id).copied()
    }

    pub fn set_state(
        &mut self,
        id: PluginId,
        state: LifecycleState,
    ) {
        self.states.insert(id, state);
    }

    pub fn initialize(
        &mut self,
        plugin: &mut dyn Plugin,
        context: &mut PluginContext,
    ) -> Result<(), String> {
        let id = plugin.id().clone();

        self.set_state(id.clone(), LifecycleState::Initializing);

        match plugin.initialize(context) {
            Ok(()) => {
                self.set_state(id, LifecycleState::Running);
                Ok(())
            }
            Err(error) => {
                self.set_state(id, LifecycleState::Failed);
                Err(error)
            }
        }
    }

    pub fn dispatch(
        &mut self,
        plugin: &mut dyn Plugin,
        event: &PluginEvent,
        context: &mut PluginContext,
    ) -> Result<(), String> {
        plugin.handle_event(event, context)
    }

    pub fn shutdown(
        &mut self,
        plugin: &mut dyn Plugin,
        context: &mut PluginContext,
    ) -> Result<(), String> {
        let id = plugin.id().clone();

        self.set_state(id.clone(), LifecycleState::Stopping);

        match plugin.shutdown(context) {
            Ok(()) => {
                self.set_state(id, LifecycleState::Stopped);
                Ok(())
            }
            Err(error) => {
                self.set_state(id, LifecycleState::Failed);
                Err(error)
            }
        }
    }

    pub fn remove(&mut self, id: &PluginId) {
        self.states.remove(id);
    }

    pub fn clear(&mut self) {
        self.states.clear();
    }

    pub fn states(
        &self,
    ) -> impl Iterator<Item = (&PluginId, &LifecycleState)> {
        self.states.iter()
    }
}

impl Default for LifecycleManager {
    fn default() -> Self {
        Self::new()
    }
}
