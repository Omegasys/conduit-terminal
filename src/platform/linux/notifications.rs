use std::fmt;

/// Notification urgency.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NotificationUrgency {
    Low,
    Normal,
    Critical,
}

impl Default for NotificationUrgency {
    fn default() -> Self {
        Self::Normal
    }
}

/// Linux desktop notification.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LinuxNotification {
    pub title: String,
    pub body: String,
    pub urgency: NotificationUrgency,
    pub application_name: String,
}

impl LinuxNotification {
    pub fn new(
        title: impl Into<String>,
        body: impl Into<String>,
    ) -> Self {
        Self {
            title: title.into(),
            body: body.into(),
            urgency: NotificationUrgency::Normal,
            application_name: "Conduit".to_string(),
        }
    }

    pub fn with_urgency(mut self, urgency: NotificationUrgency) -> Self {
        self.urgency = urgency;
        self
    }

    pub fn with_application_name(
        mut self,
        name: impl Into<String>,
    ) -> Self {
        self.application_name = name.into();
        self
    }
}

/// Linux notification manager.
///
/// This is a platform boundary. A future implementation can use the
/// freedesktop.org notification D-Bus interface through `dbus.rs`.
#[derive(Debug, Default)]
pub struct LinuxNotificationManager {
    enabled: bool,
}

impl LinuxNotificationManager {
    pub fn new() -> Self {
        Self { enabled: true }
    }

    pub fn enabled(&self) -> bool {
        self.enabled
    }

    pub fn set_enabled(&mut self, enabled: bool) {
        self.enabled = enabled;
    }

    /// Queues/sends a notification through the platform notification layer.
    ///
    /// The transport is intentionally not hard-coded here yet.
    pub fn notify(
        &self,
        notification: &LinuxNotification,
    ) -> Result<(), LinuxNotificationError> {
        if !self.enabled {
            return Err(LinuxNotificationError::Disabled);
        }

        if notification.title.is_empty() {
            return Err(LinuxNotificationError::Invalid(
                "notification title cannot be empty".to_string(),
            ));
        }

        Ok(())
    }
}

/// Notification errors.
#[derive(Debug)]
pub enum LinuxNotificationError {
    Disabled,
    Unavailable,
    Invalid(String),
    Transport(String),
}

impl fmt::Display for LinuxNotificationError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Disabled => formatter.write_str("notifications are disabled"),
            Self::Unavailable => {
                formatter.write_str("desktop notification service is unavailable")
            }
            Self::Invalid(message) => {
                write!(formatter, "invalid notification: {message}")
            }
            Self::Transport(message) => {
                write!(formatter, "notification transport error: {message}")
            }
        }
    }
}

impl std::error::Error for LinuxNotificationError {}
