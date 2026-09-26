use std::collections::VecDeque;

#[derive(Debug, Clone)]
pub struct DesktopNotification {
    pub title: String,
    pub body: String,
    pub application_name: String,
    pub icon: Option<String>,
}

impl DesktopNotification {
    pub fn new(
        title: impl Into<String>,
        body: impl Into<String>,
    ) -> Self {
        Self {
            title: title.into(),
            body: body.into(),
            application_name: "Conduit".to_owned(),
            icon: None,
        }
    }

    pub fn with_icon(mut self, icon: impl Into<String>) -> Self {
        self.icon = Some(icon.into());
        self
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DesktopNotificationAction {
    Send,
    Dismiss,
}

#[derive(Debug, Default)]
pub struct DesktopNotificationBackend {
    pending: VecDeque<DesktopNotification>,
    sent_count: u64,
}

impl DesktopNotificationBackend {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn send(&mut self, notification: DesktopNotification) {
        self.pending.push_back(notification);
        self.sent_count = self.sent_count.saturating_add(1);
    }

    pub fn next(&mut self) -> Option<DesktopNotification> {
        self.pending.pop_front()
    }

    pub fn pending(&self) -> impl Iterator<Item = &DesktopNotification> {
        self.pending.iter()
    }

    pub fn pending_count(&self) -> usize {
        self.pending.len()
    }

    pub fn sent_count(&self) -> u64 {
        self.sent_count
    }

    pub fn clear(&mut self) {
        self.pending.clear();
    }
}
