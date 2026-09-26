use std::collections::VecDeque;
use std::time::{Duration, Instant};

use super::activity::{ActivityKind, ActivityTracker};
use super::bell::{BellConfig, BellHandler};
use super::desktop::{
    DesktopNotification,
    DesktopNotificationBackend,
};
use super::urgency::{UrgencyLevel, UrgencyTracker};
use super::visual::{
    VisualNotification,
    VisualNotificationConfig,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct NotificationId(pub u64);

impl NotificationId {
    pub fn value(self) -> u64 {
        self.0
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum NotificationPriority {
    Low,
    Normal,
    High,
    Critical,
}

impl Default for NotificationPriority {
    fn default() -> Self {
        Self::Normal
    }
}

#[derive(Debug, Clone)]
pub struct Notification {
    pub id: NotificationId,
    pub title: String,
    pub body: String,
    pub priority: NotificationPriority,
    pub created_at: Instant,
    pub duration: Option<Duration>,
    pub terminal_bell: bool,
    pub desktop: bool,
    pub visual: bool,
}

impl Notification {
    pub fn new(
        id: NotificationId,
        title: impl Into<String>,
        body: impl Into<String>,
    ) -> Self {
        Self {
            id,
            title: title.into(),
            body: body.into(),
            priority: NotificationPriority::Normal,
            created_at: Instant::now(),
            duration: Some(Duration::from_secs(5)),
            terminal_bell: false,
            desktop: true,
            visual: false,
        }
    }

    pub fn expired(&self, now: Instant) -> bool {
        match self.duration {
            Some(duration) => now.duration_since(self.created_at) >= duration,
            None => false,
        }
    }
}

#[derive(Debug)]
pub struct NotificationManager {
    next_id: u64,
    notifications: VecDeque<Notification>,

    bell: BellHandler,
    activity: ActivityTracker,
    urgency: UrgencyTracker,

    desktop: DesktopNotificationBackend,
    visual_config: VisualNotificationConfig,

    desktop_enabled: bool,
    visual_enabled: bool,
    bell_enabled: bool,

    max_notifications: usize,
}

impl Default for NotificationManager {
    fn default() -> Self {
        Self::new()
    }
}

impl NotificationManager {
    pub fn new() -> Self {
        Self {
            next_id: 1,
            notifications: VecDeque::new(),

            bell: BellHandler::new(BellConfig::default()),
            activity: ActivityTracker::new(),
            urgency: UrgencyTracker::new(),

            desktop: DesktopNotificationBackend::new(),
            visual_config: VisualNotificationConfig::default(),

            desktop_enabled: true,
            visual_enabled: true,
            bell_enabled: true,

            max_notifications: 100,
        }
    }

    pub fn notify(
        &mut self,
        title: impl Into<String>,
        body: impl Into<String>,
    ) -> NotificationId {
        let id = NotificationId(self.next_id);
        self.next_id = self.next_id.saturating_add(1);

        let notification = Notification::new(id, title, body);

        self.push(notification);

        id
    }

    pub fn notify_terminal(
        &mut self,
        title: impl Into<String>,
        body: impl Into<String>,
    ) -> NotificationId {
        let id = NotificationId(self.next_id);
        self.next_id = self.next_id.saturating_add(1);

        let mut notification = Notification::new(id, title, body);
        notification.terminal_bell = true;

        self.push(notification);

        id
    }

    pub fn push(&mut self, notification: Notification) {
        if self.bell_enabled && notification.terminal_bell {
            self.bell.ring();
        }

        if self.desktop_enabled && notification.desktop {
            self.desktop.send(DesktopNotification::new(
                notification.title.clone(),
                notification.body.clone(),
            ));
        }

        if self.visual_enabled && notification.visual {
            self.visual_config.trigger();
        }

        if notification.priority >= NotificationPriority::High {
            self.urgency.set(
                notification.id,
                UrgencyLevel::High,
            );
        }

        self.notifications.push_back(notification);

        while self.notifications.len() > self.max_notifications {
            self.notifications.pop_front();
        }
    }

    pub fn update(&mut self) {
        let now = Instant::now();

        self.notifications
            .retain(|notification| !notification.expired(now));

        self.bell.update();
        self.activity.update();
        self.urgency.update();
        self.visual_config.update();
    }

    pub fn dismiss(&mut self, id: NotificationId) -> bool {
        if let Some(position) = self
            .notifications
            .iter()
            .position(|notification| notification.id == id)
        {
            self.notifications.remove(position);
            self.urgency.clear(id);
            true
        } else {
            false
        }
    }

    pub fn clear(&mut self) {
        self.notifications.clear();
        self.urgency.clear_all();
    }

    pub fn notifications(&self) -> impl Iterator<Item = &Notification> {
        self.notifications.iter()
    }

    pub fn latest(&self) -> Option<&Notification> {
        self.notifications.back()
    }

    pub fn count(&self) -> usize {
        self.notifications.len()
    }

    pub fn is_empty(&self) -> bool {
        self.notifications.is_empty()
    }

    pub fn set_desktop_enabled(&mut self, enabled: bool) {
        self.desktop_enabled = enabled;
    }

    pub fn set_visual_enabled(&mut self, enabled: bool) {
        self.visual_enabled = enabled;
    }

    pub fn set_bell_enabled(&mut self, enabled: bool) {
        self.bell_enabled = enabled;
    }

    pub fn desktop_enabled(&self) -> bool {
        self.desktop_enabled
    }

    pub fn visual_enabled(&self) -> bool {
        self.visual_enabled
    }

    pub fn bell_enabled(&self) -> bool {
        self.bell_enabled
    }

    pub fn set_max_notifications(&mut self, maximum: usize) {
        self.max_notifications = maximum.max(1);

        while self.notifications.len() > self.max_notifications {
            self.notifications.pop_front();
        }
    }

    pub fn bell(&self) -> &BellHandler {
        &self.bell
    }

    pub fn bell_mut(&mut self) -> &mut BellHandler {
        &mut self.bell
    }

    pub fn activity(&self) -> &ActivityTracker {
        &self.activity
    }

    pub fn activity_mut(&mut self) -> &mut ActivityTracker {
        &mut self.activity
    }

    pub fn urgency(&self) -> &UrgencyTracker {
        &self.urgency
    }

    pub fn urgency_mut(&mut self) -> &mut UrgencyTracker {
        &mut self.urgency
    }

    pub fn record_activity(
        &mut self,
        kind: ActivityKind,
    ) {
        self.activity.record(kind);
    }
}
