use super::connections::ConnectionManager;
use super::navigation::NavigationManager;
use super::sessions::SessionManager;
use super::state::SidebarState;
use super::tools::SidebarToolManager;
use super::workspaces::SidebarWorkspaceManager;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SidebarPosition {
    Left,
    Right,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SidebarPanel {
    Navigation,
    Workspaces,
    Sessions,
    Connections,
    Tools,
}

#[derive(Debug)]
pub struct Sidebar {
    id: String,
    title: String,
    position: SidebarPosition,
    width: u32,
    minimum_width: u32,
    maximum_width: u32,
    visible: bool,
    enabled: bool,
    resizable: bool,
    overlay: bool,
    active_panel: SidebarPanel,
    state: SidebarState,
    workspaces: SidebarWorkspaceManager,
    sessions: SessionManager,
    connections: ConnectionManager,
    tools: SidebarToolManager,
    navigation: NavigationManager,
}

impl Sidebar {
    pub fn new(id: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            title: "Sidebar".to_string(),
            position: SidebarPosition::Left,
            width: 280,
            minimum_width: 160,
            maximum_width: 640,
            visible: true,
            enabled: true,
            resizable: true,
            overlay: false,
            active_panel: SidebarPanel::Navigation,
            state: SidebarState::Created,
            workspaces: SidebarWorkspaceManager::new(),
            sessions: SessionManager::new(),
            connections: ConnectionManager::new(),
            tools: SidebarToolManager::new(),
            navigation: NavigationManager::new(),
        }
    }

    pub fn id(&self) -> &str {
        &self.id
    }

    pub fn title(&self) -> &str {
        &self.title
    }

    pub fn position(&self) -> SidebarPosition {
        self.position
    }

    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn minimum_width(&self) -> u32 {
        self.minimum_width
    }

    pub fn maximum_width(&self) -> u32 {
        self.maximum_width
    }

    pub fn is_visible(&self) -> bool {
        self.visible
    }

    pub fn is_enabled(&self) -> bool {
        self.enabled
    }

    pub fn is_resizable(&self) -> bool {
        self.resizable
    }

    pub fn is_overlay(&self) -> bool {
        self.overlay
    }

    pub fn active_panel(&self) -> SidebarPanel {
        self.active_panel
    }

    pub fn state(&self) -> SidebarState {
        self.state
    }

    pub fn workspaces(&self) -> &SidebarWorkspaceManager {
        &self.workspaces
    }

    pub fn workspaces_mut(&mut self) -> &mut SidebarWorkspaceManager {
        &mut self.workspaces
    }

    pub fn sessions(&self) -> &SessionManager {
        &self.sessions
    }

    pub fn sessions_mut(&mut self) -> &mut SessionManager {
        &mut self.sessions
    }

    pub fn connections(&self) -> &ConnectionManager {
        &self.connections
    }

    pub fn connections_mut(&mut self) -> &mut ConnectionManager {
        &mut self.connections
    }

    pub fn tools(&self) -> &SidebarToolManager {
        &self.tools
    }

    pub fn tools_mut(&mut self) -> &mut SidebarToolManager {
        &mut self.tools
    }

    pub fn navigation(&self) -> &NavigationManager {
        &self.navigation
    }

    pub fn navigation_mut(&mut self) -> &mut NavigationManager {
        &mut self.navigation
    }

    pub fn set_title(&mut self, title: impl Into<String>) {
        self.title = title.into();
    }

    pub fn set_position(&mut self, position: SidebarPosition) {
        self.position = position;
    }

    pub fn set_width(&mut self, width: u32) {
        self.width = width.clamp(self.minimum_width, self.maximum_width);
    }

    pub fn set_width_limits(&mut self, minimum: u32, maximum: u32) {
        self.minimum_width = minimum.min(maximum);
        self.maximum_width = maximum.max(minimum);
        self.set_width(self.width);
    }

    pub fn set_visible(&mut self, visible: bool) {
        self.visible = visible;

        if visible {
            self.state = SidebarState::Visible;
        } else {
            self.state = SidebarState::Hidden;
        }
    }

    pub fn set_enabled(&mut self, enabled: bool) {
        self.enabled = enabled;

        if !enabled {
            self.state = SidebarState::Disabled;
        } else if self.visible {
            self.state = SidebarState::Visible;
        }
    }

    pub fn set_resizable(&mut self, resizable: bool) {
        self.resizable = resizable;
    }

    pub fn set_overlay(&mut self, overlay: bool) {
        self.overlay = overlay;
    }

    pub fn activate_panel(&mut self, panel: SidebarPanel) -> bool {
        if !self.enabled {
            return false;
        }

        self.active_panel = panel;
        true
    }

    pub fn show(&mut self) {
        self.set_visible(true);
    }

    pub fn hide(&mut self) {
        self.set_visible(false);
    }

    pub fn toggle(&mut self) {
        self.set_visible(!self.visible);
    }

    pub fn next_panel(&mut self) -> SidebarPanel {
        self.active_panel = match self.active_panel {
            SidebarPanel::Navigation => SidebarPanel::Workspaces,
            SidebarPanel::Workspaces => SidebarPanel::Sessions,
            SidebarPanel::Sessions => SidebarPanel::Connections,
            SidebarPanel::Connections => SidebarPanel::Tools,
            SidebarPanel::Tools => SidebarPanel::Navigation,
        };

        self.active_panel
    }

    pub fn previous_panel(&mut self) -> SidebarPanel {
        self.active_panel = match self.active_panel {
            SidebarPanel::Navigation => SidebarPanel::Tools,
            SidebarPanel::Workspaces => SidebarPanel::Navigation,
            SidebarPanel::Sessions => SidebarPanel::Workspaces,
            SidebarPanel::Connections => SidebarPanel::Sessions,
            SidebarPanel::Tools => SidebarPanel::Connections,
        };

        self.active_panel
    }

    pub fn set_state(&mut self, state: SidebarState) {
        self.state = state;
    }

    pub fn initialize(&mut self) {
        self.state = if self.visible {
            SidebarState::Visible
        } else {
            SidebarState::Hidden
        };
    }

    pub fn close(&mut self) {
        self.state = SidebarState::Closed;
    }
}
