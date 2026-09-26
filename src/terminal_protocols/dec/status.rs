#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DecStatusQuery {
    OperatingStatus,
    DeviceAttributes,
    CursorPosition,
    Mode(u16),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DecDeviceStatus {
    Ready,
    Malfunction,
    CursorPosition {
        row: u16,
        column: u16,
    },
    Attributes(Vec<u16>),
    Mode {
        mode: u16,
        enabled: bool,
    },
}
