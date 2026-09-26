//! Accessibility announcements.
//!
//! Announcements provide a common channel for status changes, terminal
//! events, dialogs, errors, and other information that should be exposed
//! to assistive technologies.

use std::collections::VecDeque;

/// Importance of an accessibility announcement.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum AnnouncementPriority {
    Low,
    Normal,
    High,
    Critical,
}

/// Semantic category of an announcement.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AnnouncementKind {
    Status,
    Information,
    Warning,
    Error,
    Success,
    Focus,
    TerminalOutput,
    TerminalState,
    Dialog,
    Progress,
}

/// A message intended for an accessibility frontend.
#[derive(Debug, Clone)]
pub struct Announcement {
    pub kind: AnnouncementKind,
    pub priority: AnnouncementPriority,
    pub message: String,
    pub interrupt: bool,
}

impl Announcement {
    pub fn new(
        kind: AnnouncementKind,
        priority: AnnouncementPriority,
        message: impl Into<String>,
    ) -> Self {
        Self {
            kind,
            priority,
            message: message.into(),
            interrupt: priority >= AnnouncementPriority::High,
        }
    }

    pub fn interrupt(mut self, interrupt: bool) -> Self {
        self.interrupt = interrupt;
        self
    }

    pub fn status(message: impl Into<String>) -> Self {
        Self::new(
            AnnouncementKind::Status,
            AnnouncementPriority::Normal,
            message,
        )
    }

    pub fn error(message: impl Into<String>) -> Self {
        Self::new(
            AnnouncementKind::Error,
            AnnouncementPriority::High,
            message,
        )
    }

    pub fn success(message: impl Into<String>) -> Self {
        Self::new(
            AnnouncementKind::Success,
            AnnouncementPriority::Normal,
            message,
        )
    }

    pub fn terminal(message: impl Into<String>) -> Self {
        Self::new(
            AnnouncementKind::TerminalOutput,
            AnnouncementPriority::Low,
            message,
        )
    }
}

/// Queue consumed by GUI/TUI accessibility adapters.
#[derive(Debug, Clone, Default)]
pub struct AnnouncementQueue {
    queue: VecDeque<Announcement>,
    max_size: usize,
}

impl AnnouncementQueue {
    pub fn new() -> Self {
        Self {
            queue: VecDeque::new(),
            max_size: 256,
        }
    }

    pub fn with_capacity(max_size: usize) -> Self {
        Self {
            queue: VecDeque::new(),
            max_size: max_size.max(1),
        }
    }

    pub fn push(&mut self, announcement: Announcement) {
        if self.queue.len() >= self.max_size {
            self.queue.pop_front();
        }

        self.queue.push_back(announcement);
    }

    pub fn pop(&mut self) -> Option<Announcement> {
        self.queue.pop_front()
    }

    pub fn peek(&self) -> Option<&Announcement> {
        self.queue.front()
    }

    pub fn clear(&mut self) {
        self.queue.clear();
    }

    pub fn len(&self) -> usize {
        self.queue.len()
    }

    pub fn is_empty(&self) -> bool {
        self.queue.is_empty()
    }

    pub fn is_full(&self) -> bool {
        self.queue.len() >= self.max_size
    }

    pub fn set_max_size(&mut self, max_size: usize) {
        self.max_size = max_size.max(1);

        while self.queue.len() > self.max_size {
            self.queue.pop_front();
        }
    }

    pub fn max_size(&self) -> usize {
        self.max_size
    }

    /// Remove all low-priority announcements while retaining important ones.
    pub fn retain_important(&mut self) {
        self.queue.retain(|announcement| {
            announcement.priority >= AnnouncementPriority::Normal
        });
    }
}
