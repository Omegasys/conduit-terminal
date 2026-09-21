use std::collections::BTreeMap;

use crate::colors::Color;

#[derive(Clone, Debug)]
pub enum CustomProtocolAction {
    Print(String),

    CarriageReturn,
    LineFeed,
    Backspace,
    Tab,
    Bell,

    CursorUp(u16),
    CursorDown(u16),
    CursorForward(u16),
    CursorBackward(u16),
    CursorPosition {
        row: u16,
        column: u16,
    },

    SetForeground(Color),
    SetBackground(Color),
    ResetColors,

    SaveCursor,
    RestoreCursor,

    SetMode {
        mode: u16,
        enabled: bool,
    },

    SetTitle(String),

    ClipboardRead {
        selection: String,
    },

    ClipboardWrite {
        selection: String,
        data: Vec<u8>,
    },

    EnableBracketedPaste,
    DisableBracketedPaste,

    EnableMouse,
    DisableMouse,

    BeginSynchronizedUpdate,
    EndSynchronizedUpdate,

    Custom {
        name: String,
        parameters: BTreeMap<String, String>,
        payload: Vec<u8>,
    },
}

#[derive(Clone, Debug)]
pub enum ProtocolResponse {
    None,
    Bytes(Vec<u8>),
    Text(String),
}

#[derive(Clone, Debug)]
pub enum CustomProtocolEvent {
    Action(CustomProtocolAction),

    Response(ProtocolResponse),

    Warning(String),

    Error(String),

    RequestPermission {
        capability: String,
        reason: String,
    },

    ProtocolMessage {
        name: String,
        data: Vec<u8>,
    },
}
