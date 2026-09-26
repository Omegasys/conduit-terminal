#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TerminalStatusType {
    OperatingStatus,
    CursorPosition,
    DeviceAttributes,
    DeviceStatus,
    Mode,
    Unknown,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TerminalStatus {
    Ready,
    Busy,
    CursorPosition {
        row: u16,
        column: u16,
    },
    DeviceAttributes(Vec<u16>),
    DeviceStatus(u16),
    Mode {
        mode: u16,
        enabled: bool,
    },
    Unknown(Vec<u8>),
}

impl TerminalStatus {
    pub fn status_type(&self) -> TerminalStatusType {
        match self {
            Self::Ready | Self::Busy => TerminalStatusType::OperatingStatus,

            Self::CursorPosition { .. } => {
                TerminalStatusType::CursorPosition
            }

            Self::DeviceAttributes(_) => {
                TerminalStatusType::DeviceAttributes
            }

            Self::DeviceStatus(_) => TerminalStatusType::DeviceStatus,

            Self::Mode { .. } => TerminalStatusType::Mode,

            Self::Unknown(_) => TerminalStatusType::Unknown,
        }
    }
}
