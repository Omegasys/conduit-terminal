use std::collections::HashMap;

use super::event::Event;
use super::filters::{AllowAll, EventFilter};
use super::priorities::EventPriority;

/// Unique subscription identifier.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct SubscriptionId(u64);

impl SubscriptionId {
    pub fn new(value: u64) -> Self {
        Self(value)
    }

    pub fn value(self) -> u64 {
        self.0
    }
}

/// Callback used to receive events.
pub type EventCallback = Box<dyn Fn(&Event) + Send + Sync>;

/// A registered event subscription.
pub struct Subscription {
    id: SubscriptionId,
    priority: EventPriority,
    filter: Box<dyn EventFilter>,
    callback: EventCallback,
    enabled: bool,
}

impl Subscription {
    pub fn new(
        id: SubscriptionId,
        priority: EventPriority,
        filter: Box<dyn EventFilter>,
        callback: EventCallback,
    ) -> Self {
        Self {
            id,
            priority,
            filter,
            callback,
            enabled: true,
        }
    }

    pub fn id(&self) -> SubscriptionId {
        self.id
    }

    pub fn priority(&self) -> EventPriority {
        self.priority
    }

    pub fn is_enabled(&self) -> bool {
        self.enabled
    }

    pub fn enable(&mut self) {
        self.enabled = true;
    }

    pub fn disable(&mut self) {
        self.enabled = false;
    }

    pub fn matches(&self, event: &Event) -> bool {
        self.enabled && self.filter.matches(event)
    }

    pub fn invoke(&self, event: &Event) {
        if self.matches(event) {
            (self.callback)(event);
        }
    }
}

/// Manages event subscriptions.
pub struct SubscriptionManager {
    subscriptions: HashMap<SubscriptionId, Subscription>,
    next_id: u64,
}

impl Default for SubscriptionManager {
    fn default() -> Self {
        Self::new()
    }
}

impl SubscriptionManager {
    pub fn new() -> Self {
        Self {
            subscriptions: HashMap::new(),
            next_id: 1,
        }
    }

    pub fn subscribe<F>(
        &mut self,
        priority: EventPriority,
        filter: Box<dyn EventFilter>,
        callback: F,
    ) -> SubscriptionId
    where
        F: Fn(&Event) + Send + Sync + 'static,
    {
        let id = SubscriptionId::new(self.next_id);
        self.next_id = self.next_id.saturating_add(1);

        let subscription = Subscription::new(
            id,
            priority,
            filter,
            Box::new(callback),
        );

        self.subscriptions.insert(id, subscription);

        id
    }

    pub fn subscribe_all<F>(
        &mut self,
        priority: EventPriority,
        callback: F,
    ) -> SubscriptionId
    where
        F: Fn(&Event) + Send + Sync + 'static,
    {
        self.subscribe(priority, Box::new(AllowAll), callback)
    }

    pub fn unsubscribe(&mut self, id: SubscriptionId) -> bool {
        self.subscriptions.remove(&id).is_some()
    }

    pub fn get(&self, id: SubscriptionId) -> Option<&Subscription> {
        self.subscriptions.get(&id)
    }

    pub fn get_mut(&mut self, id: SubscriptionId) -> Option<&mut Subscription> {
        self.subscriptions.get_mut(&id)
    }

    pub fn enable(&mut self, id: SubscriptionId) -> bool {
        if let Some(subscription) = self.subscriptions.get_mut(&id) {
            subscription.enable();
            true
        } else {
            false
        }
    }

    pub fn disable(&mut self, id: SubscriptionId) -> bool {
        if let Some(subscription) = self.subscriptions.get_mut(&id) {
            subscription.disable();
            true
        } else {
            false
        }
    }

    pub fn iter(&self) -> impl Iterator<Item = &Subscription> {
        self.subscriptions.values()
    }

    pub fn len(&self) -> usize {
        self.subscriptions.len()
    }

    pub fn is_empty(&self) -> bool {
        self.subscriptions.is_empty()
    }

    pub fn clear(&mut self) {
        self.subscriptions.clear();
    }
}
