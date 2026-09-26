#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ControlCode {
    Nul,
    Bell,
    Backspace,
    HorizontalTab,
    LineFeed,
    VerticalTab,
    FormFeed,
    CarriageReturn,
    ShiftOut,
    ShiftIn,
    Escape,
    Delete,
    C1(u8),
    Other(u8),
}

impl ControlCode {
    pub fn from_byte(byte: u8) -> Self {
        match byte {
            0x00 => Self::Nul,
            0x07 => Self::Bell,
            0x08 => Self::Backspace,
            0x09 => Self::HorizontalTab,
            0x0A => Self::LineFeed,
            0x0B => Self::VerticalTab,
            0x0C => Self::FormFeed,
            0x0D => Self::CarriageReturn,
            0x0E => Self::ShiftOut,
            0x0F => Self::ShiftIn,
            0x1B => Self::Escape,
            0x7F => Self::Delete,
            0x80..=0x9F => Self::C1(byte),
            value => Self::Other(value),
        }
    }

    pub fn byte(self) -> u8 {
        match self {
            Self::Nul => 0x00,
            Self::Bell => 0x07,
            Self::Backspace => 0x08,
            Self::HorizontalTab => 0x09,
            Self::LineFeed => 0x0A,
            Self::VerticalTab => 0x0B,
            Self::FormFeed => 0x0C,
            Self::CarriageReturn => 0x0D,
            Self::ShiftOut => 0x0E,
            Self::ShiftIn => 0x0F,
            Self::Escape => 0x1B,
            Self::Delete => 0x7F,
            Self::C1(value) | Self::Other(value) => value,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ControlFunction {
    Code(ControlCode),
    Execute(u8),
    StringTerminated(Vec<u8>),
}

impl ControlFunction {
    pub fn code(code: ControlCode) -> Self {
        Self::Code(code)
    }

    pub fn execute(byte: u8) -> Self {
        Self::Execute(byte)
    }

    pub fn string(data: impl Into<Vec<u8>>) -> Self {
        Self::StringTerminated(data.into())
    }
}
