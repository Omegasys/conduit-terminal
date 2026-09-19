#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MenuAction {
    NewWindow,
    CloseWindow,
    Quit,

    NewTab,
    CloseTab,
    ReopenTab,
    NextTab,
    PreviousTab,

    NewPane,
    ClosePane,
    SplitHorizontal,
    SplitVertical,
    FocusNextPane,
    FocusPreviousPane,
    ZoomPane,
    SwapPane,

    NewWorkspace,
    CloseWorkspace,
    NextWorkspace,
    PreviousWorkspace,

    OpenFile,
    OpenFolder,
    OpenTerminal,
    SaveSession,
    RestoreSession,

    Copy,
    Paste,
    SelectAll,
    ClearSelection,

    Find,
    FindNext,
    FindPrevious,

    Undo,
    Redo,
    Cut,

    ToggleSidebar,
    ToggleToolbar,
    ToggleStatusBar,
    ToggleFullscreen,
    ToggleCommandPalette,

    OpenSettings,
    OpenProfiles,
    OpenThemes,
    OpenKeybindings,

    ReloadConfiguration,
    ReloadResources,

    About,
    Documentation,

    Custom(String),
}

impl MenuAction {
    pub fn command_id(&self) -> String {
        match self {
            Self::NewWindow => "window.new",
            Self::CloseWindow => "window.close",
            Self::Quit => "application.quit",

            Self::NewTab => "tab.new",
            Self::CloseTab => "tab.close",
            Self::ReopenTab => "tab.reopen",
            Self::NextTab => "tab.next",
            Self::PreviousTab => "tab.previous",

            Self::NewPane => "pane.new",
            Self::ClosePane => "pane.close",
            Self::SplitHorizontal => "pane.split.horizontal",
            Self::SplitVertical => "pane.split.vertical",
            Self::FocusNextPane => "pane.focus.next",
            Self::FocusPreviousPane => "pane.focus.previous",
            Self::ZoomPane => "pane.zoom",
            Self::SwapPane => "pane.swap",

            Self::NewWorkspace => "workspace.new",
            Self::CloseWorkspace => "workspace.close",
            Self::NextWorkspace => "workspace.next",
            Self::PreviousWorkspace => "workspace.previous",

            Self::OpenFile => "file.open",
            Self::OpenFolder => "folder.open",
            Self::OpenTerminal => "terminal.open",
            Self::SaveSession => "session.save",
            Self::RestoreSession => "session.restore",

            Self::Copy => "edit.copy",
            Self::Paste => "edit.paste",
            Self::SelectAll => "edit.select_all",
            Self::ClearSelection => "edit.clear_selection",

            Self::Find => "edit.find",
            Self::FindNext => "edit.find_next",
            Self::FindPrevious => "edit.find_previous",

            Self::Undo => "edit.undo",
            Self::Redo => "edit.redo",
            Self::Cut => "edit.cut",

            Self::ToggleSidebar => "view.sidebar.toggle",
            Self::ToggleToolbar => "view.toolbar.toggle",
            Self::ToggleStatusBar => "view.status_bar.toggle",
            Self::ToggleFullscreen => "view.fullscreen.toggle",
            Self::ToggleCommandPalette => "view.command_palette.toggle",

            Self::OpenSettings => "tools.settings",
            Self::OpenProfiles => "tools.profiles",
            Self::OpenThemes => "tools.themes",
            Self::OpenKeybindings => "tools.keybindings",

            Self::ReloadConfiguration => "configuration.reload",
            Self::ReloadResources => "resources.reload",

            Self::About => "help.about",
            Self::Documentation => "help.documentation",

            Self::Custom(id) => return id.clone(),
        }
        .to_string()
    }
}
