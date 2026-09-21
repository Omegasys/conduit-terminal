//! DEC private modes and DEC-specific terminal controls.

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DecMode {
    ApplicationCursorKeys,
    ApplicationKeypad,
    Origin,
    AutoWrap,
    ReverseVideo,
    CursorVisible,
    AlternateScreen,
    SaveCursor,
    BracketedPaste,
    FocusReporting,
    MouseButtonTracking,
    MouseAnyEvent,
    SgrMouse,
    Utf8Mouse,
    SynchronizedUpdates,
    LineFeedNewLine,
}

impl DecMode {
    pub fn number(self) -> u16 {
        match self {
            Self::ApplicationCursorKeys => 1,
            Self::ApplicationKeypad => 66,
            Self::Origin => 6,
            Self::AutoWrap => 7,
            Self::ReverseVideo => 5,
            Self::CursorVisible => 25,
            Self::AlternateScreen => 1049,
            Self::SaveCursor => 1048,
            Self::BracketedPaste => 2004,
            Self::FocusReporting => 1004,
            Self::MouseButtonTracking => 1000,
            Self::MouseAnyEvent => 1003,
            Self::SgrMouse => 1006,
            Self::Utf8Mouse => 1005,
            Self::SynchronizedUpdates => 2026,
            Self::LineFeedNewLine => 20,
        }
    }

    pub fn from_number(value: u16) -> Option<Self> {
        Some(match value {
            1 => Self::ApplicationCursorKeys,
            66 => Self::ApplicationKeypad,
            6 => Self::Origin,
            7 => Self::AutoWrap,
            5 => Self::ReverseVideo,
            25 => Self::CursorVisible,
            1049 => Self::AlternateScreen,
            1048 => Self::SaveCursor,
            2004 => Self::BracketedPaste,
            1004 => Self::FocusReporting,
            1000 => Self::MouseButtonTracking,
            1003 => Self::MouseAnyEvent,
            1006 => Self::SgrMouse,
            1005 => Self::Utf8Mouse,
            2026 => Self::SynchronizedUpdates,
            20 => Self::LineFeedNewLine,
            _ => return None,
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DecAction {
    SetMode(DecMode),
    ResetMode(DecMode),

    SaveCursor,
    RestoreCursor,

    SoftReset,

    FullReset,

    SetPrivateMode {
        mode: u16,
        enabled: bool,
    },

    DeviceAttributes,

    DeviceStatusReport,

    RequestTerminalParameters,

    UnknownSequence(Vec<u8>),
}

#[derive(Debug, Clone)]
pub struct DecParser;

impl Default for DecParser {
    fn default() -> Self {
        Self::new()
    }
}

impl DecParser {
    pub fn new() -> Self {
        Self
    }

    pub fn set_mode(&self, mode: u16, enabled: bool) -> DecAction {
        if let Some(mode) = DecMode::from_number(mode) {
            if enabled {
                DecAction::SetMode(mode)
            } else {
                DecAction::ResetMode(mode)
            }
        } else {
            DecAction::SetPrivateMode { mode, enabled }
        }
    }

    pub fn parse_private_modes(
        &self,
        modes: &[u16],
        enabled: bool,
    ) -> Vec<DecAction> {
        modes
            .iter()
            .map(|mode| self.set_mode(*mode, enabled))
            .collect()
    }

    pub fn parse_csi_final(&self, final_byte: u8) -> Option<DecAction> {
        match final_byte {
            b'c' => Some(DecAction::DeviceAttributes),
            b'n' => Some(DecAction::DeviceStatusReport),
            b'x' => Some(DecAction::RequestTerminalParameters),
            _ => None,
        }
    }
}
