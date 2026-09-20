use super::pty::PtyId;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct FlowPaneId(u64);

impl FlowPaneId {
    pub fn new(value: u64) -> Self {
        Self(value)
    }

    pub fn value(&self) -> u64 {
        self.0
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PaneState {
    Created,
    Initializing,
    Active,
    Inactive,
    Busy,
    Error,
    Closing,
    Closed,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PaneFocus {
    Focused,
    Unfocused,
}

#[derive(Debug, Clone)]
pub struct PaneInfo {
    id: FlowPaneId,
    pty_id: Option<PtyId>,
    state: PaneState,
    focus: PaneFocus,
    title: String,
    cwd: Option<String>,
    shell: Option<String>,
    synchronized: bool,
    zoomed: bool,
    visible: bool,
}

impl PaneInfo {
    pub fn new(id: FlowPaneId, title: impl Into<String>) -> Self {
        Self {
            id,
            pty_id: None,
            state: PaneState::Created,
            focus: PaneFocus::Unfocused,
            title: title.into(),
            cwd: None,
            shell: None,
            synchronized: false,
            zoomed: false,
            visible: true,
        }
    }

    pub fn id(&self) -> FlowPaneId {
        self.id
    }

    pub fn pty_id(&self) -> Option<PtyId> {
        self.pty_id
    }

    pub fn set_pty_id(&mut self, pty_id: Option<PtyId>) {
        self.pty_id = pty_id;
    }

    pub fn state(&self) -> PaneState {
        self.state
    }

    pub fn set_state(&mut self, state: PaneState) {
        self.state = state;
    }

    pub fn focus(&self) -> PaneFocus {
        self.focus
    }

    pub fn set_focus(&mut self, focus: PaneFocus) {
        self.focus = focus;
    }

    pub fn is_focused(&self) -> bool {
        self.focus == PaneFocus::Focused
    }

    pub fn title(&self) -> &str {
        &self.title
    }

    pub fn set_title(&mut self, title: impl Into<String>) {
        self.title = title.into();
    }

    pub fn cwd(&self) -> Option<&str> {
        self.cwd.as_deref()
    }

    pub fn set_cwd(&mut self, cwd: Option<String>) {
        self.cwd = cwd;
    }

    pub fn shell(&self) -> Option<&str> {
        self.shell.as_deref()
    }

    pub fn set_shell(&mut self, shell: Option<String>) {
        self.shell = shell;
    }

    pub fn synchronized(&self) -> bool {
        self.synchronized
    }

    pub fn set_synchronized(&mut self, synchronized: bool) {
        self.synchronized = synchronized;
    }

    pub fn zoomed(&self) -> bool {
        self.zoomed
    }

    pub fn set_zoomed(&mut self, zoomed: bool) {
        self.zoomed = zoomed;
    }

    pub fn visible(&self) -> bool {
        self.visible
    }

    pub fn set_visible(&mut self, visible: bool) {
        self.visible = visible;
    }
}

#[derive(Debug, Default)]
pub struct PaneViewManager {
    panes: Vec<PaneInfo>,
    active: Option<FlowPaneId>,
    next_id: u64,
}

impl PaneViewManager {
    pub fn new() -> Self {
        Self {
            next_id: 1,
            ..Default::default()
        }
    }

    pub fn create(&mut self, title: impl Into<String>) -> FlowPaneId {
        let id = FlowPaneId::new(self.next_id);
        self.next_id += 1;

        self.panes.push(PaneInfo::new(id, title));

        if self.active.is_none() {
            self.active = Some(id);
        }

        id
    }

    pub fn add(&mut self, pane: PaneInfo) {
        if self.active.is_none() {
            self.active = Some(pane.id());
        }

        self.panes.push(pane);
    }

    pub fn get(&self, id: FlowPaneId) -> Option<&PaneInfo> {
        self.panes.iter().find(|pane| pane.id() == id)
    }

    pub fn get_mut(&mut self, id: FlowPaneId) -> Option<&mut PaneInfo> {
        self.panes.iter_mut().find(|pane| pane.id() == id)
    }

    pub fn activate(&mut self, id: FlowPaneId) -> bool {
        if self.get(id).is_none() {
            return false;
        }

        for pane in &mut self.panes {
            pane.set_focus(PaneFocus::Unfocused);
        }

        if let Some(pane) = self.get_mut(id) {
            pane.set_focus(PaneFocus::Focused);
        }

        self.active = Some(id);
        true
    }

    pub fn active(&self) -> Option<&PaneInfo> {
        self.active.and_then(|id| self.get(id))
    }

    pub fn active_id(&self) -> Option<FlowPaneId> {
        self.active
    }

    pub fn remove(&mut self, id: FlowPaneId) -> Option<PaneInfo> {
        let index = self.panes.iter().position(|pane| pane.id() == id)?;
        let pane = self.panes.remove(index);

        if self.active == Some(id) {
            self.active = self.panes.first().map(|pane| pane.id());

            if let Some(active) = self.active {
                let _ = self.activate(active);
            }
        }

        Some(pane)
    }

    pub fn panes(&self) -> impl Iterator<Item = &PaneInfo> {
        self.panes.iter()
    }

    pub fn visible(&self) -> impl Iterator<Item = &PaneInfo> {
        self.panes.iter().filter(|pane| pane.visible())
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
    }
}
