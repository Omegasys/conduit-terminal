use std::time::SystemTime;

/// Events that can be delivered to IPC subscribers.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IpcEvent {
    pub sequence: u64,
    pub timestamp: SystemTime,
    pub kind: IpcEventKind,
}

impl IpcEvent {
    pub fn new(sequence: u64, kind: IpcEventKind) -> Self {
        Self {
            sequence,
            timestamp: SystemTime::now(),
            kind,
        }
    }
}

/// Public events exposed through the IPC system.
///
/// These intentionally contain stable identifiers rather than references to
/// internal Conduit objects.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum IpcEventKind {
    Started,

    Stopped,

    WindowCreated {
        window_id: u64,
    },

    WindowClosed {
        window_id: u64,
    },

    WindowFocused {
        window_id: u64,
    },

    TabCreated {
        tab_id: u64,
        window_id: u64,
    },

    TabClosed {
        tab_id: u64,
    },

    TabFocused {
        tab_id: u64,
    },

    PaneCreated {
        pane_id: u64,
        tab_id: u64,
    },

    PaneClosed {
        pane_id: u64,
    },

    PaneFocused {
        pane_id: u64,
    },

    PaneResized {
        pane_id: u64,
        rows: u16,
        columns: u16,
    },

    Output {
        pane_id: u64,
        data: Vec<u8>,
    },

    Input {
        pane_id: u64,
        data: Vec<u8>,
    },

    CommandStarted {
        pane_id: u64,
        command: String,
    },

    CommandFinished {
        pane_id: u64,
        status: i32,
    },

    DirectoryChanged {
        pane_id: u64,
        path: String,
    },

    TitleChanged {
        pane_id: u64,
        title: String,
    },

    ModeChanged {
        mode: String,
    },

    ConfigurationReloaded,

    Notification {
        title: String,
        message: String,
    },

    Custom {
        name: String,
        data: String,
    },
}
