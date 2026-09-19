/// Priority controls the order in which subscribers receive events.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum EventPriority {
    Lowest = 0,
    Low = 25,
    Normal = 50,
    High = 75,
    Highest = 100,
}

impl Default for EventPriority {
    fn default() -> Self {
        Self::Normal
    }
}

impl EventPriority {
    pub fn value(self) -> u8 {
        self as u8
    }
}

/// A priority assigned to a subscription.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Priority(pub EventPriority);

impl Priority {
    pub fn new(priority: EventPriority) -> Self {
        Self(priority)
    }

    pub fn value(self) -> u8 {
        self.0.value()
    }
}

impl Default for Priority {
    fn default() -> Self {
        Self(EventPriority::Normal)
    }
}
