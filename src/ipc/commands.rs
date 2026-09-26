/// Commands accepted by the Conduit IPC server.
///
/// Commands describe intent. The server should forward them into the normal
/// application/event system rather than manipulating UI state directly.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum IpcCommand {
    Ping,

    GetStatus,

    GetVersion,

    Shutdown,

    OpenWindow,

    CloseWindow {
        window_id: u64,
    },

    FocusWindow {
        window_id: u64,
    },

    CreateTab {
        window_id: Option<u64>,
    },

    CloseTab {
        tab_id: u64,
    },

    FocusTab {
        tab_id: u64,
    },

    CreatePane {
        tab_id: u64,
    },

    ClosePane {
        pane_id: u64,
    },

    FocusPane {
        pane_id: u64,
    },

    SplitPane {
        pane_id: u64,
        direction: SplitDirection,
    },

    SendInput {
        pane_id: u64,
        data: Vec<u8>,
    },

    ResizePane {
        pane_id: u64,
        rows: u16,
        columns: u16,
    },

    ExecuteCommand {
        pane_id: u64,
        command: String,
    },

    SwitchMode {
        mode: String,
    },

    ReloadConfiguration,

    GetConfiguration,

    SetConfiguration {
        key: String,
        value: String,
    },

    Search {
        query: String,
    },

    Subscribe {
        event_types: Vec<String>,
    },

    Unsubscribe {
        event_types: Vec<String>,
    },

    Custom {
        name: String,
        arguments: Vec<String>,
    },
}

/// Pane splitting direction.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SplitDirection {
    Horizontal,
    Vertical,
}

/// Result returned for an IPC command.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum IpcCommandResult {
    Success,

    Pong,

    Version {
        version: String,
        protocol: String,
    },

    Status {
        running: bool,
        windows: usize,
        tabs: usize,
        panes: usize,
    },

    Text(String),

    Data(Vec<u8>),

    Error {
        code: String,
        message: String,
    },
}

impl IpcCommandResult {
    pub fn success() -> Self {
        Self::Success
    }

    pub fn error(
        code: impl Into<String>,
        message: impl Into<String>,
    ) -> Self {
        Self::Error {
            code: code.into(),
            message: message.into(),
        }
    }

    pub fn is_success(&self) -> bool {
        !matches!(self, Self::Error { .. })
    }
}
