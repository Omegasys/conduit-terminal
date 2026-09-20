use super::events::{
    TerminalEvent,
    TerminalEventKind,
};

pub trait TerminalEventFilter {
    fn matches(&self, event: &TerminalEvent) -> bool;
}

#[derive(Debug, Clone, Copy)]
pub struct AllowAll;

impl TerminalEventFilter for AllowAll {
    fn matches(&self, _event: &TerminalEvent) -> bool {
        true
    }
}

#[derive(Debug, Clone, Copy)]
pub struct EventKindFilter {
    kind: TerminalEventKind,
}

impl EventKindFilter {
    pub fn new(kind: TerminalEventKind) -> Self {
        Self { kind }
    }
}

impl TerminalEventFilter for EventKindFilter {
    fn matches(&self, event: &TerminalEvent) -> bool {
        event.kind() == self.kind
    }
}

#[derive(Debug, Clone, Copy)]
pub struct EventKindsFilter {
    kinds: &'static [TerminalEventKind],
}

impl EventKindsFilter {
    pub fn new(kinds: &'static [TerminalEventKind]) -> Self {
        Self { kinds }
    }
}

impl TerminalEventFilter for EventKindsFilter {
    fn matches(&self, event: &TerminalEvent) -> bool {
        self.kinds.contains(&event.kind())
    }
}

#[derive(Debug, Clone, Copy)]
pub struct PtyFilter {
    pty_id: super::pty::PtyId,
}

impl PtyFilter {
    pub fn new(pty_id: super::pty::PtyId) -> Self {
        Self { pty_id }
    }
}

impl TerminalEventFilter for PtyFilter {
    fn matches(&self, event: &TerminalEvent) -> bool {
        event.pty_id() == Some(self.pty_id)
    }
}

#[derive(Debug, Clone, Copy)]
pub struct PaneFilter {
    pane_id: super::panes::FlowPaneId,
}

impl PaneFilter {
    pub fn new(pane_id: super::panes::FlowPaneId) -> Self {
        Self { pane_id }
    }
}

impl TerminalEventFilter for PaneFilter {
    fn matches(&self, event: &TerminalEvent) -> bool {
        event.pane_id() == Some(self.pane_id)
    }
}

pub struct AllFilter {
    filters: Vec<Box<dyn TerminalEventFilter + Send + Sync>>,
}

impl AllFilter {
    pub fn new() -> Self {
        Self {
            filters: Vec::new(),
        }
    }

    pub fn add<F>(&mut self, filter: F)
    where
        F: TerminalEventFilter + Send + Sync + 'static,
    {
        self.filters.push(Box::new(filter));
    }
}

impl Default for AllFilter {
    fn default() -> Self {
        Self::new()
    }
}

impl TerminalEventFilter for AllFilter {
    fn matches(&self, event: &TerminalEvent) -> bool {
        self.filters
            .iter()
            .all(|filter| filter.matches(event))
    }
}

pub struct AnyFilter {
    filters: Vec<Box<dyn TerminalEventFilter + Send + Sync>>,
}

impl AnyFilter {
    pub fn new() -> Self {
        Self {
            filters: Vec::new(),
        }
    }

    pub fn add<F>(&mut self, filter: F)
    where
        F: TerminalEventFilter + Send + Sync + 'static,
    {
        self.filters.push(Box::new(filter));
    }
}

impl Default for AnyFilter {
    fn default() -> Self {
        Self::new()
    }
}

impl TerminalEventFilter for AnyFilter {
    fn matches(&self, event: &TerminalEvent) -> bool {
        self.filters
            .iter()
            .any(|filter| filter.matches(event))
    }
}

pub struct NotFilter<F>
where
    F: TerminalEventFilter,
{
    filter: F,
}

impl<F> NotFilter<F>
where
    F: TerminalEventFilter,
{
    pub fn new(filter: F) -> Self {
        Self { filter }
    }
}

impl<F> TerminalEventFilter for NotFilter<F>
where
    F: TerminalEventFilter,
{
    fn matches(&self, event: &TerminalEvent) -> bool {
        !self.filter.matches(event)
    }
}
