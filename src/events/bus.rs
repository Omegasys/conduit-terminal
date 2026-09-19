use std::sync::{Arc, Mutex};

use super::command::{Command, CommandHandler, CommandResult};
use super::dispatcher::{DispatchResult, EventDispatcher};
use super::event::{
    Event,
    EventCategory,
    EventId,
    EventPayload,
    EventSource,
};
use super::priorities::EventPriority;
use super::subscriptions::{
    SubscriptionId,
    SubscriptionManager,
};

/// Central event bus for Conduit.
///
/// The bus is intentionally independent of the GUI, TUI, CLI, terminal,
/// configuration, and plugin implementations. Those systems communicate
/// through events and commands rather than directly depending on one
/// another wherever practical.
#[derive(Clone)]
pub struct EventBus {
    subscriptions: Arc<Mutex<SubscriptionManager>>,
    dispatcher: EventDispatcher,
    commands: Arc<Mutex<std::collections::HashMap<String, Arc<dyn CommandHandler>>>>,
    next_event_id: Arc<Mutex<u64>>,
}

impl Default for EventBus {
    fn default() -> Self {
        Self::new()
    }
}

impl EventBus {
    pub fn new() -> Self {
        let subscriptions = Arc::new(Mutex::new(SubscriptionManager::new()));

        Self {
            dispatcher: EventDispatcher::new(subscriptions.clone()),
            subscriptions,
            commands: Arc::new(Mutex::new(std::collections::HashMap::new())),
            next_event_id: Arc::new(Mutex::new(1)),
        }
    }

    pub fn next_event_id(&self) -> EventId {
        let mut counter = match self.next_event_id.lock() {
            Ok(counter) => counter,
            Err(poisoned) => poisoned.into_inner(),
        };

        let id = EventId::new(*counter);
        *counter = counter.saturating_add(1);

        id
    }

    pub fn emit(
        &self,
        category: EventCategory,
        source: EventSource,
        name: impl Into<String>,
        payload: EventPayload,
    ) -> DispatchResult {
        let mut event = Event::new(
            self.next_event_id(),
            category,
            source,
            name,
            payload,
        );

        self.dispatcher.dispatch(&mut event)
    }

    pub fn emit_event(&self, mut event: Event) -> DispatchResult {
        self.dispatcher.dispatch(&mut event)
    }

    pub fn subscribe<F>(
        &self,
        priority: EventPriority,
        filter: Box<dyn super::filters::EventFilter>,
        callback: F,
    ) -> SubscriptionId
    where
        F: Fn(&Event) + Send + Sync + 'static,
    {
        let mut manager = match self.subscriptions.lock() {
            Ok(manager) => manager,
            Err(poisoned) => poisoned.into_inner(),
        };

        manager.subscribe(priority, filter, callback)
    }

    pub fn subscribe_all<F>(
        &self,
        priority: EventPriority,
        callback: F,
    ) -> SubscriptionId
    where
        F: Fn(&Event) + Send + Sync + 'static,
    {
        let mut manager = match self.subscriptions.lock() {
            Ok(manager) => manager,
            Err(poisoned) => poisoned.into_inner(),
        };

        manager.subscribe_all(priority, callback)
    }

    pub fn unsubscribe(&self, id: SubscriptionId) -> bool {
        let mut manager = match self.subscriptions.lock() {
            Ok(manager) => manager,
            Err(poisoned) => poisoned.into_inner(),
        };

        manager.unsubscribe(id)
    }

    pub fn register_command(
        &self,
        name: impl Into<String>,
        handler: Arc<dyn CommandHandler>,
    ) {
        let mut commands = match self.commands.lock() {
            Ok(commands) => commands,
            Err(poisoned) => poisoned.into_inner(),
        };

        commands.insert(name.into(), handler);
    }

    pub fn unregister_command(&self, name: &str) -> bool {
        let mut commands = match self.commands.lock() {
            Ok(commands) => commands,
            Err(poisoned) => poisoned.into_inner(),
        };

        commands.remove(name).is_some()
    }

    pub fn execute_command(&self, command: &Command) -> CommandResult {
        let commands = match self.commands.lock() {
            Ok(commands) => commands,
            Err(poisoned) => poisoned.into_inner(),
        };

        match commands.get(command.name()) {
            Some(handler) => handler.execute(command),
            None => CommandResult::error(format!(
                "unknown command: {}",
                command.name()
            )),
        }
    }

    pub fn subscription_count(&self) -> usize {
        let manager = match self.subscriptions.lock() {
            Ok(manager) => manager,
            Err(poisoned) => poisoned.into_inner(),
        };

        manager.len()
    }

    pub fn command_count(&self) -> usize {
        let commands = match self.commands.lock() {
            Ok(commands) => commands,
            Err(poisoned) => poisoned.into_inner(),
        };

        commands.len()
    }
}
