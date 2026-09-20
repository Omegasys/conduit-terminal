use crate::panes::{Pane, PaneId, PaneState};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PaneFocusMode {
    Normal,
    Focused,
    Zoomed,
    Synchronized,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PaneDisplayState {
    Normal,
    Active,
    Busy,
    Error,
    Inactive,
    Closed,
}

impl PaneDisplayState {
    pub fn from_pane(pane: &Pane, active: bool) -> Self {
        if pane.state() == PaneState::Closed {
            return Self::Closed;
        }

        if pane.state() == PaneState::Error {
            return Self::Error;
        }

        if pane.state() == PaneState::Busy {
            return Self::Busy;
        }

        if active {
            return Self::Active;
        }

        match pane.state() {
            PaneState::Inactive => Self::Inactive,
            _ => Self::Normal,
        }
    }
}

pub struct PaneDisplay {
    pane: Pane,
    display_state: PaneDisplayState,
    focus_mode: PaneFocusMode,
    visible: bool,
}

impl PaneDisplay {
    pub fn new(pane: Pane, active: bool) -> Self {
        let display_state = PaneDisplayState::from_pane(&pane, active);

        Self {
            pane,
            display_state,
            focus_mode: if active {
                PaneFocusMode::Focused
            } else {
                PaneFocusMode::Normal
            },
            visible: true,
        }
    }

    pub fn pane(&self) -> &Pane {
        &self.pane
    }

    pub fn pane_mut(&mut self) -> &mut Pane {
        &mut self.pane
    }

    pub fn id(&self) -> PaneId {
        self.pane.id()
    }

    pub fn title(&self) -> &str {
        self.pane.title()
    }

    pub fn display_state(&self) -> PaneDisplayState {
        self.display_state
    }

    pub fn set_display_state(&mut self, state: PaneDisplayState) {
        self.display_state = state;
    }

    pub fn focus_mode(&self) -> PaneFocusMode {
        self.focus_mode
    }

    pub fn set_focus_mode(&mut self, mode: PaneFocusMode) {
        self.focus_mode = mode;
    }

    pub fn visible(&self) -> bool {
        self.visible
    }

    pub fn set_visible(&mut self, visible: bool) {
        self.visible = visible;
    }

    pub fn refresh_state(&mut self, active: bool) {
        self.display_state = PaneDisplayState::from_pane(&self.pane, active);

        self.focus_mode = if self.pane.zoomed() {
            PaneFocusMode::Zoomed
        } else if self.pane.synchronized() {
            PaneFocusMode::Synchronized
        } else if active {
            PaneFocusMode::Focused
        } else {
            PaneFocusMode::Normal
        };
    }
}

pub struct TuiPaneView {
    panes: Vec<PaneDisplay>,
    active: Option<PaneId>,
    synchronized: bool,
    zoomed: Option<PaneId>,
}

impl Default for TuiPaneView {
    fn default() -> Self {
        Self::new()
    }
}

impl TuiPaneView {
    pub fn new() -> Self {
        Self {
            panes: Vec::new(),
            active: None,
            synchronized: false,
            zoomed: None,
        }
    }

    pub fn add(&mut self, pane: Pane) -> PaneId {
        let id = pane.id();
        let active = self.active.is_none();

        self.panes.push(PaneDisplay::new(pane, active));

        if active {
            self.active = Some(id);
        }

        self.refresh();
        id
    }

    pub fn remove(&mut self, id: PaneId) -> Option<PaneDisplay> {
        let index = self.panes.iter().position(|pane| pane.id() == id)?;
        let removed = self.panes.remove(index);

        if self.active == Some(id) {
            self.active = self.panes.first().map(|pane| pane.id());
        }

        if self.zoomed == Some(id) {
            self.zoomed = None;
        }

        self.refresh();
        Some(removed)
    }

    pub fn get(&self, id: PaneId) -> Option<&PaneDisplay> {
        self.panes.iter().find(|pane| pane.id() == id)
    }

    pub fn get_mut(&mut self, id: PaneId) -> Option<&mut PaneDisplay> {
        self.panes.iter_mut().find(|pane| pane.id() == id)
    }

    pub fn activate(&mut self, id: PaneId) -> bool {
        if self.get(id).is_none() {
            return false;
        }

        self.active = Some(id);
        self.refresh();
        true
    }

    pub fn active(&self) -> Option<&PaneDisplay> {
        self.active.and_then(|id| self.get(id))
    }

    pub fn active_id(&self) -> Option<PaneId> {
        self.active
    }

    pub fn panes(&self) -> &[PaneDisplay] {
        &self.panes
    }

    pub fn panes_mut(&mut self) -> &mut [PaneDisplay] {
        &mut self.panes
    }

    pub fn visible_panes(&self) -> Vec<&PaneDisplay> {
        if let Some(zoomed) = self.zoomed {
            return self
                .get(zoomed)
                .into_iter()
                .filter(|pane| pane.visible())
                .collect();
        }

        self.panes.iter().filter(|pane| pane.visible()).collect()
    }

    pub fn zoom(&mut self, id: PaneId) -> bool {
        if self.get(id).is_none() {
            return false;
        }

        self.zoomed = Some(id);

        if let Some(pane) = self.get_mut(id) {
            pane.set_focus_mode(PaneFocusMode::Zoomed);
        }

        true
    }

    pub fn unzoom(&mut self) {
        self.zoomed = None;
        self.refresh();
    }

    pub fn zoomed(&self) -> Option<PaneId> {
        self.zoomed
    }

    pub fn toggle_zoom(&mut self) {
        if let Some(active) = self.active {
            if self.zoomed == Some(active) {
                self.unzoom();
            } else {
                self.zoom(active);
            }
        }
    }

    pub fn synchronized(&self) -> bool {
        self.synchronized
    }

    pub fn set_synchronized(&mut self, synchronized: bool) {
        self.synchronized = synchronized;

        for pane in &mut self.panes {
            if synchronized {
                pane.set_focus_mode(PaneFocusMode::Synchronized);
            }
        }
    }

    pub fn toggle_synchronized(&mut self) {
        self.set_synchronized(!self.synchronized);
    }

    pub fn refresh(&mut self) {
        for pane in &mut self.panes {
            let active = self.active == Some(pane.id());
            pane.refresh_state(active);
        }

        if self.synchronized {
            for pane in &mut self.panes {
                pane.set_focus_mode(PaneFocusMode::Synchronized);
            }
        }

        if let Some(zoomed) = self.zoomed {
            if let Some(pane) = self.get_mut(zoomed) {
                pane.set_focus_mode(PaneFocusMode::Zoomed);
            }
        }
    }

    pub fn len(&self) -> usize {
        self.panes.len()
    }

    pub fn is_empty(&self) -> bool {
        self.panes.is_empty()
    }

    pub fn clear(&mut self) {
        self.panes.clear();
        self.active = None;
        self.zoomed = None;
        self.synchronized = false;
    }
}
