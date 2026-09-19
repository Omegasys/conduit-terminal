use super::event::{Event, EventCategory, EventSource};

/// Predicate used to determine whether an event should reach a subscriber.
pub trait EventFilter: Send + Sync {
    fn matches(&self, event: &Event) -> bool;
}

/// Filter that accepts everything.
#[derive(Debug, Clone, Copy, Default)]
pub struct AllowAll;

impl EventFilter for AllowAll {
    fn matches(&self, _event: &Event) -> bool {
        true
    }
}

/// Filter by event category.
#[derive(Debug, Clone, Copy)]
pub struct CategoryFilter {
    category: EventCategory,
}

impl CategoryFilter {
    pub fn new(category: EventCategory) -> Self {
        Self { category }
    }
}

impl EventFilter for CategoryFilter {
    fn matches(&self, event: &Event) -> bool {
        event.category() == self.category
    }
}

/// Filter by event source.
#[derive(Debug, Clone)]
pub struct SourceFilter {
    source: EventSource,
}

impl SourceFilter {
    pub fn new(source: EventSource) -> Self {
        Self { source }
    }
}

impl EventFilter for SourceFilter {
    fn matches(&self, event: &Event) -> bool {
        event.source() == &self.source
    }
}

/// Filter by event name.
#[derive(Debug, Clone)]
pub struct NameFilter {
    name: String,
}

impl NameFilter {
    pub fn new(name: impl Into<String>) -> Self {
        Self { name: name.into() }
    }
}

impl EventFilter for NameFilter {
    fn matches(&self, event: &Event) -> bool {
        event.name() == self.name
    }
}

/// Filter by event-name prefix.
#[derive(Debug, Clone)]
pub struct NamePrefixFilter {
    prefix: String,
}

impl NamePrefixFilter {
    pub fn new(prefix: impl Into<String>) -> Self {
        Self {
            prefix: prefix.into(),
        }
    }
}

impl EventFilter for NamePrefixFilter {
    fn matches(&self, event: &Event) -> bool {
        event.name().starts_with(&self.prefix)
    }
}

/// Combines multiple filters using logical AND.
pub struct AllFilter {
    filters: Vec<Box<dyn EventFilter>>,
}

impl AllFilter {
    pub fn new() -> Self {
        Self {
            filters: Vec::new(),
        }
    }

    pub fn add<F>(&mut self, filter: F)
    where
        F: EventFilter + 'static,
    {
        self.filters.push(Box::new(filter));
    }
}

impl Default for AllFilter {
    fn default() -> Self {
        Self::new()
    }
}

impl EventFilter for AllFilter {
    fn matches(&self, event: &Event) -> bool {
        self.filters.iter().all(|filter| filter.matches(event))
    }
}

/// Combines multiple filters using logical OR.
pub struct AnyFilter {
    filters: Vec<Box<dyn EventFilter>>,
}

impl AnyFilter {
    pub fn new() -> Self {
        Self {
            filters: Vec::new(),
        }
    }

    pub fn add<F>(&mut self, filter: F)
    where
        F: EventFilter + 'static,
    {
        self.filters.push(Box::new(filter));
    }
}

impl Default for AnyFilter {
    fn default() -> Self {
        Self::new()
    }
}

impl EventFilter for AnyFilter {
    fn matches(&self, event: &Event) -> bool {
        self.filters.iter().any(|filter| filter.matches(event))
    }
}
