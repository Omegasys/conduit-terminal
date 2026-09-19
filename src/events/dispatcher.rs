use std::sync::{Arc, Mutex};

use super::event::Event;
use super::subscriptions::SubscriptionManager;

/// Result of dispatching an event.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DispatchResult {
    pub invoked: usize,
    pub matched: usize,
    pub stopped: bool,
}

/// Dispatches events to subscriptions in priority order.
#[derive(Clone)]
pub struct EventDispatcher {
    subscriptions: Arc<Mutex<SubscriptionManager>>,
}

impl EventDispatcher {
    pub fn new(subscriptions: Arc<Mutex<SubscriptionManager>>) -> Self {
        Self { subscriptions }
    }

    pub fn dispatch(&self, event: &mut Event) -> DispatchResult {
        let manager = match self.subscriptions.lock() {
            Ok(manager) => manager,
            Err(poisoned) => poisoned.into_inner(),
        };

        let mut subscriptions: Vec<_> = manager.iter().collect();

        subscriptions.sort_by(|a, b| {
            b.priority()
                .cmp(&a.priority())
        });

        let mut matched = 0;
        let mut invoked = 0;

        for subscription in subscriptions {
            if event.is_propagation_stopped() {
                break;
            }

            if subscription.matches(event) {
                matched += 1;
                subscription.invoke(event);
                invoked += 1;
            }
        }

        DispatchResult {
            invoked,
            matched,
            stopped: event.is_propagation_stopped(),
        }
    }
}
