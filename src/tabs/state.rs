//! Tab state and activity tracking.

/// Current tab state.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TabState {
    Created,
    Initializing,
    Active,
    Inactive,
    Closing,
    Closed,
}

/// Activity indicator for a tab.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TabActivity {
    None,
    Output,
    Notification,
    Bell,
    Attention,
    Error,
}

impl TabActivity {
    pub fn requires_attention(self) -> bool {
        matches!(
            self,
            Self::Notification
                | Self::Bell
                | Self::Attention
                | Self::Error
        )
    }
}
