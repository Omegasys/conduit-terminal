//! Logical pane representation.

use std::fmt;
use std::sync::atomic::{
    AtomicU64,
    Ordering,
};

/// Globally unique pane identifier.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct PaneId(u64);

impl PaneId {
    pub fn new() -> Self {
        static NEXT_ID: AtomicU64 =
            AtomicU64::new(1);

        Self(
            NEXT_ID.fetch_add(
                1,
                Ordering::Relaxed,
            ),
        )
    }

    pub fn value(self) -> u64 {
        self.0
    }
}

impl Default for PaneId {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for PaneId {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        write!(
            formatter,
            "pane-{}",
            self.0
        )
    }
}

/// Orientation used by a pane's primary split.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PaneOrientation {
    Horizontal,
    Vertical,
}

impl PaneOrientation {
    pub fn opposite(self) -> Self {
        match self {
            Self::Horizontal => Self::Vertical,
            Self::Vertical => Self::Horizontal,
        }
    }
}

/// A logical terminal pane.
///
/// The pane does not directly own a PTY. Instead, `session_id` refers
/// to the terminal session managed by Conduit's core/session subsystem.
#[derive(Debug, Clone)]
pub struct Pane {
    id: PaneId,

    session_id: Option<String>,

    title: String,

    orientation: PaneOrientation,

    active: bool,
    focused: bool,
    zoomed: bool,

    /// Whether this pane should remain visible when its parent layout
    /// is recalculated.
    visible: bool,

    /// Relative size requested by the layout engine.
    size_ratio: f32,

    created_at: u64,
}

impl Pane {
    pub fn new(
        title: impl Into<String>,
    ) -> Self {
        Self {
            id: PaneId::new(),

            session_id: None,

            title: title.into(),

            orientation:
                PaneOrientation::Horizontal,

            active: false,
            focused: false,
            zoomed: false,

            visible: true,

            size_ratio: 1.0,

            created_at:
                current_timestamp(),
        }
    }

    pub fn id(&self) -> PaneId {
        self.id
    }

    pub fn session_id(
        &self,
    ) -> Option<&str> {
        self.session_id
            .as_deref()
    }

    pub fn set_session_id(
        &mut self,
        session_id: Option<String>,
    ) {
        self.session_id = session_id;
    }

    pub fn title(&self) -> &str {
        &self.title
    }

    pub fn set_title(
        &mut self,
        title: impl Into<String>,
    ) {
        self.title = title.into();
    }

    pub fn orientation(
        &self,
    ) -> PaneOrientation {
        self.orientation
    }

    pub fn set_orientation(
        &mut self,
        orientation: PaneOrientation,
    ) {
        self.orientation = orientation;
    }

    pub fn active(&self) -> bool {
        self.active
    }

    pub fn set_active(
        &mut self,
        active: bool,
    ) {
        self.active = active;
    }

    pub fn focused(&self) -> bool {
        self.focused
    }

    pub fn set_focused(
        &mut self,
        focused: bool,
    ) {
        self.focused = focused;
    }

    pub fn zoomed(&self) -> bool {
        self.zoomed
    }

    pub fn set_zoomed(
        &mut self,
        zoomed: bool,
    ) {
        self.zoomed = zoomed;
    }

    pub fn visible(&self) -> bool {
        self.visible
    }

    pub fn set_visible(
        &mut self,
        visible: bool,
    ) {
        self.visible = visible;
    }

    pub fn size_ratio(&self) -> f32 {
        self.size_ratio
    }

    pub fn set_size_ratio(
        &mut self,
        ratio: f32,
    ) {
        self.size_ratio =
            ratio.clamp(
                0.05,
                0.95,
            );
    }

    pub fn created_at(&self) -> u64 {
        self.created_at
    }

    pub fn activate(&mut self) {
        self.active = true;
        self.focused = true;
    }

    pub fn deactivate(&mut self) {
        self.active = false;
        self.focused = false;
    }

    pub fn focus(&mut self) {
        self.focused = true;
        self.active = true;
    }

    pub fn unfocus(&mut self) {
        self.focused = false;
    }
}

fn current_timestamp() -> u64 {
    use std::time::{
        SystemTime,
        UNIX_EPOCH,
    };

    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs()
}
