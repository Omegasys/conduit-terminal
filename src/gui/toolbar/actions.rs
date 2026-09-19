#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ToolbarAction {
    NewWindow,
    NewTab,
    CloseTab,

    NewPane,
    SplitHorizontal,
    SplitVertical,
    FocusNextPane,
    FocusPreviousPane,
    ZoomPane,

    Copy,
    Paste,

    Search,
    CommandPalette,

    NewWorkspace,
    NextWorkspace,
    PreviousWorkspace,

    OpenSettings,
    OpenProfiles,
    OpenThemes,

    SaveSession,
    RestoreSession,

    ReloadConfiguration,
    ReloadResources,

    ToggleSidebar,
    ToggleFullscreen,

    Custom(String),
}

impl ToolbarAction {
    pub fn command_id(&self) -> String {
        match self {
            Self::NewWindow => "window.new",
            Self::NewTab => "tab.new",
            Self::CloseTab => "tab.close",

            Self::NewPane => "pane.new",
            Self::SplitHorizontal => "pane.split.horizontal",
            Self::SplitVertical => "pane.split.vertical",
            Self::FocusNextPane => "pane.focus.next",
            Self::FocusPreviousPane => "pane.focus.previous",
            Self::ZoomPane => "pane.zoom",

            Self::Copy => "edit.copy",
            Self::Paste => "edit.paste",

            Self::Search => "edit.find",
            Self::CommandPalette => "view.command_palette.toggle",

            Self::NewWorkspace => "workspace.new",
            Self::NextWorkspace => "workspace.next",
            Self::PreviousWorkspace => "workspace.previous",

            Self::OpenSettings => "tools.settings",
            Self::OpenProfiles => "tools.profiles",
            Self::OpenThemes => "tools.themes",

            Self::SaveSession => "session.save",
            Self::RestoreSession => "session.restore",

            Self::ReloadConfiguration => "configuration.reload",
            Self::ReloadResources => "resources.reload",

            Self::ToggleSidebar => "view.sidebar.toggle",
            Self::ToggleFullscreen => "view.fullscreen.toggle",

            Self::Custom(id) => return id.clone(),
        }
        .to_string()
    }
}
