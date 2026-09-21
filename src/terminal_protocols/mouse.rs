//! Terminal mouse reporting protocols.
//!
//! Supports classic X10/UTF-8 style reports and SGR mouse reports.
//! Coordinates are normalized into a common representation for the input
//! subsystem.

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MouseButton {
    Left,
    Middle,
    Right,
    Release,
    WheelUp,
    WheelDown,
    WheelLeft,
    WheelRight,
    Other(u8),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MouseAction {
    Press,
    Release,
    Motion,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MouseModifiers {
    pub shift: bool,
    pub alt: bool,
    pub ctrl: bool,
    pub meta: bool,
}

impl Default for MouseModifiers {
    fn default() -> Self {
        Self {
            shift: false,
            alt: false,
            ctrl: false,
            meta: false,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MouseEvent {
    pub button: MouseButton,
    pub action: MouseAction,
    pub column: u16,
    pub row: u16,
    pub modifiers: MouseModifiers,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MouseProtocol {
    X10,
    Utf8,
    Sgr,
    Urxvt,
}

#[derive(Debug, Clone)]
pub struct MouseParser {
    protocol: MouseProtocol,
    buffer: Vec<u8>,
}

impl MouseParser {
    pub fn new(protocol: MouseProtocol) -> Self {
        Self {
            protocol,
            buffer: Vec::new(),
        }
    }

    pub fn protocol(&self) -> MouseProtocol {
        self.protocol
    }

    pub fn set_protocol(&mut self, protocol: MouseProtocol) {
        self.protocol = protocol;
        self.buffer.clear();
    }

    pub fn reset(&mut self) {
        self.buffer.clear();
    }

    pub fn parse_sgr(&self, sequence: &str) -> Option<MouseEvent> {
        let trimmed = sequence
            .strip_prefix("<")
            .unwrap_or(sequence);

        let release = trimmed.ends_with('m');
        let body = trimmed
            .strip_suffix('M')
            .or_else(|| trimmed.strip_suffix('m'))?;

        let mut parts = body.split(';');

        let code = parts.next()?.parse::<u16>().ok()?;
        let column = parts.next()?.parse::<u16>().ok()?;
        let row = parts.next()?.parse::<u16>().ok()?;

        let modifiers = MouseModifiers {
            shift: code & 4 != 0,
            alt: code & 8 != 0,
            ctrl: code & 16 != 0,
            meta: false,
        };

        let action = if release {
            MouseAction::Release
        } else if code & 32 != 0 {
            MouseAction::Motion
        } else {
            MouseAction::Press
        };

        let button_code = (code & 0x03ff) as u8;

        let button = match button_code & 0x03 {
            0 if button_code & 64 == 64 => MouseButton::WheelUp,
            1 if button_code & 64 == 64 => MouseButton::WheelDown,
            2 if button_code & 64 == 64 => MouseButton::WheelLeft,
            3 if button_code & 64 == 64 => MouseButton::WheelRight,

            0 => MouseButton::Left,
            1 => MouseButton::Middle,
            2 => MouseButton::Right,
            _ => MouseButton::Other(button_code),
        };

        Some(MouseEvent {
            button,
            action,
            column,
            row,
            modifiers,
        })
    }

    pub fn parse_x10(&self, bytes: &[u8]) -> Option<MouseEvent> {
        if bytes.len() < 3 {
            return None;
        }

        let code = bytes[0].saturating_sub(32);
        let column = bytes[1].saturating_sub(32) as u16;
        let row = bytes[2].saturating_sub(32) as u16;

        let modifiers = MouseModifiers {
            shift: code & 4 != 0,
            alt: code & 8 != 0,
            ctrl: code & 16 != 0,
            meta: false,
        };

        let action = if code & 32 != 0 {
            MouseAction::Motion
        } else {
            MouseAction::Press
        };

        let button = match code & 0x03 {
            0 => MouseButton::Left,
            1 => MouseButton::Middle,
            2 => MouseButton::Right,
            3 => MouseButton::Release,
            _ => MouseButton::Other(code),
        };

        Some(MouseEvent {
            button,
            action,
            column,
            row,
            modifiers,
        })
    }

    pub fn parse(&self, input: &[u8]) -> Option<MouseEvent> {
        match self.protocol {
            MouseProtocol::Sgr => {
                let text = std::str::from_utf8(input).ok()?;
                self.parse_sgr(text)
            }

            MouseProtocol::X10
            | MouseProtocol::Utf8
            | MouseProtocol::Urxvt => self.parse_x10(input),
        }
    }

    pub fn feed(&mut self, byte: u8) -> Option<MouseEvent> {
        self.buffer.push(byte);

        let result = match self.protocol {
            MouseProtocol::Sgr => {
                if byte == b'M' || byte == b'm' {
                    self.parse(&self.buffer)
                } else {
                    None
                }
            }

            MouseProtocol::X10 => {
                if self.buffer.len() >= 3 {
                    self.parse(&self.buffer)
                } else {
                    None
                }
            }

            MouseProtocol::Utf8 | MouseProtocol::Urxvt => {
                if self.buffer.len() >= 3 {
                    self.parse(&self.buffer)
                } else {
                    None
                }
            }
        };

        if result.is_some() {
            self.buffer.clear();
        }

        result
    }
}
