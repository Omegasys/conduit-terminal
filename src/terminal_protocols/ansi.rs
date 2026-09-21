//! ANSI terminal escape sequence support.

/// State of the ANSI escape-sequence parser.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AnsiParserState {
    Ground,
    Escape,
    Csi,
    Osc,
    OscEscape,
    Charset,
    Dcs,
}

/// Normalized actions produced by ANSI parsing.
#[derive(Debug, Clone, PartialEq)]
pub enum AnsiAction {
    Print(char),

    CarriageReturn,
    LineFeed,
    Backspace,
    Tab,

    Bell,

    SaveCursor,
    RestoreCursor,

    CursorUp(u16),
    CursorDown(u16),
    CursorForward(u16),
    CursorBackward(u16),

    CursorPosition {
        row: u16,
        column: u16,
    },

    EraseDisplay(u8),
    EraseLine(u8),

    InsertLines(u16),
    DeleteLines(u16),
    InsertCharacters(u16),
    DeleteCharacters(u16),

    SetGraphicsRendition(Vec<u16>),

    SetMode {
        mode: u16,
        enabled: bool,
    },

    ResetMode {
        mode: u16,
    },

    DeviceStatusReport(u16),

    OperatingSystemCommand(String),

    UnknownSequence(Vec<u8>),
}

/// Incremental ANSI decoder.
///
/// This decoder intentionally does not modify terminal state itself. It
/// converts incoming bytes into normalized actions for the terminal core.
#[derive(Debug, Clone)]
pub struct AnsiDecoder {
    state: AnsiParserState,
    params: Vec<u16>,
    current_param: u16,
    private: bool,
    osc_buffer: Vec<u8>,
    sequence_buffer: Vec<u8>,
}

impl Default for AnsiDecoder {
    fn default() -> Self {
        Self::new()
    }
}

impl AnsiDecoder {
    pub fn new() -> Self {
        Self {
            state: AnsiParserState::Ground,
            params: Vec::new(),
            current_param: 0,
            private: false,
            osc_buffer: Vec::new(),
            sequence_buffer: Vec::new(),
        }
    }

    pub fn state(&self) -> AnsiParserState {
        self.state
    }

    pub fn reset(&mut self) {
        self.state = AnsiParserState::Ground;
        self.params.clear();
        self.current_param = 0;
        self.private = false;
        self.osc_buffer.clear();
        self.sequence_buffer.clear();
    }

    pub fn feed(&mut self, bytes: &[u8]) -> Vec<AnsiAction> {
        let mut actions = Vec::new();

        for &byte in bytes {
            actions.extend(self.feed_byte(byte));
        }

        actions
    }

    pub fn feed_byte(&mut self, byte: u8) -> Vec<AnsiAction> {
        match self.state {
            AnsiParserState::Ground => self.ground(byte),
            AnsiParserState::Escape => self.escape(byte),
            AnsiParserState::Csi => self.csi(byte),
            AnsiParserState::Osc => self.osc(byte),
            AnsiParserState::OscEscape => self.osc_escape(byte),
            AnsiParserState::Charset => self.charset(byte),
            AnsiParserState::Dcs => self.dcs(byte),
        }
    }

    fn ground(&mut self, byte: u8) -> Vec<AnsiAction> {
        match byte {
            0x1b => {
                self.sequence_buffer.clear();
                self.sequence_buffer.push(byte);
                self.state = AnsiParserState::Escape;
                Vec::new()
            }

            0x07 => vec![AnsiAction::Bell],
            0x08 => vec![AnsiAction::Backspace],
            0x09 => vec![AnsiAction::Tab],
            0x0a | 0x0b | 0x0c => vec![AnsiAction::LineFeed],
            0x0d => vec![AnsiAction::CarriageReturn],

            0x00..=0x1f | 0x7f => Vec::new(),

            _ => vec![AnsiAction::Print(byte as char)],
        }
    }

    fn escape(&mut self, byte: u8) -> Vec<AnsiAction> {
        self.sequence_buffer.push(byte);

        match byte {
            b'[' => {
                self.begin_csi();
                Vec::new()
            }

            b']' => {
                self.params.clear();
                self.current_param = 0;
                self.osc_buffer.clear();
                self.state = AnsiParserState::Osc;
                Vec::new()
            }

            b'7' => {
                self.reset();
                vec![AnsiAction::SaveCursor]
            }

            b'8' => {
                self.reset();
                vec![AnsiAction::RestoreCursor]
            }

            b'c' => {
                self.reset();
                vec![AnsiAction::UnknownSequence(vec![0x1b, b'c'])]
            }

            b'(' | b')' | b'*' | b'+' => {
                self.state = AnsiParserState::Charset;
                Vec::new()
            }

            b'P' => {
                self.state = AnsiParserState::Dcs;
                Vec::new()
            }

            b'=' | b'>' => {
                self.reset();
                Vec::new()
            }

            0x40..=0x5f => {
                let sequence = self.sequence_buffer.clone();
                self.reset();
                vec![AnsiAction::UnknownSequence(sequence)]
            }

            _ => {
                let sequence = self.sequence_buffer.clone();
                self.reset();
                vec![AnsiAction::UnknownSequence(sequence)]
            }
        }
    }

    fn begin_csi(&mut self) {
        self.params.clear();
        self.current_param = 0;
        self.private = false;
        self.state = AnsiParserState::Csi;
    }

    fn csi(&mut self, byte: u8) -> Vec<AnsiAction> {
        self.sequence_buffer.push(byte);

        match byte {
            b'?' if self.params.is_empty() && self.current_param == 0 => {
                self.private = true;
                Vec::new()
            }

            b'0'..=b'9' => {
                self.current_param = self
                    .current_param
                    .saturating_mul(10)
                    .saturating_add((byte - b'0') as u16);
                Vec::new()
            }

            b';' => {
                self.params.push(self.current_param);
                self.current_param = 0;
                Vec::new()
            }

            0x40..=0x7e => {
                self.params.push(self.current_param);
                let params = self.params.clone();
                let private = self.private;

                self.reset();

                self.dispatch_csi(byte, &params, private)
            }

            _ => Vec::new(),
        }
    }

    fn dispatch_csi(
        &self,
        final_byte: u8,
        params: &[u16],
        private: bool,
    ) -> Vec<AnsiAction> {
        let first = |default: u16| {
            params
                .first()
                .copied()
                .filter(|value| *value != 0)
                .unwrap_or(default)
        };

        match final_byte {
            b'A' => vec![AnsiAction::CursorUp(first(1))],
            b'B' => vec![AnsiAction::CursorDown(first(1))],
            b'C' => vec![AnsiAction::CursorForward(first(1))],
            b'D' => vec![AnsiAction::CursorBackward(first(1))],

            b'H' | b'f' => {
                let row = params.first().copied().filter(|v| *v != 0).unwrap_or(1);
                let column = params.get(1).copied().filter(|v| *v != 0).unwrap_or(1);

                vec![AnsiAction::CursorPosition { row, column }]
            }

            b'J' => vec![AnsiAction::EraseDisplay(first(0))],
            b'K' => vec![AnsiAction::EraseLine(first(0))],

            b'L' => vec![AnsiAction::InsertLines(first(1))],
            b'M' => vec![AnsiAction::DeleteLines(first(1))],
            b'@' => vec![AnsiAction::InsertCharacters(first(1))],
            b'P' => vec![AnsiAction::DeleteCharacters(first(1))],

            b'm' => vec![AnsiAction::SetGraphicsRendition(params.to_vec())],

            b'h' if private => params
                .iter()
                .map(|mode| AnsiAction::SetMode {
                    mode: *mode,
                    enabled: true,
                })
                .collect(),

            b'l' if private => params
                .iter()
                .map(|mode| AnsiAction::ResetMode { mode: *mode })
                .collect(),

            b'h' => params
                .iter()
                .map(|mode| AnsiAction::SetMode {
                    mode: *mode,
                    enabled: true,
                })
                .collect(),

            b'l' => params
                .iter()
                .map(|mode| AnsiAction::SetMode {
                    mode: *mode,
                    enabled: false,
                })
                .collect(),

            b'n' => vec![AnsiAction::DeviceStatusReport(first(0))],

            _ => vec![AnsiAction::UnknownSequence(
                self.sequence_buffer.clone(),
            )],
        }
    }

    fn osc(&mut self, byte: u8) -> Vec<AnsiAction> {
        match byte {
            0x07 => {
                let command = String::from_utf8_lossy(&self.osc_buffer).into_owned();
                self.reset();
                vec![AnsiAction::OperatingSystemCommand(command)]
            }

            0x1b => {
                self.state = AnsiParserState::OscEscape;
                Vec::new()
            }

            _ => {
                self.osc_buffer.push(byte);
                Vec::new()
            }
        }
    }

    fn osc_escape(&mut self, byte: u8) -> Vec<AnsiAction> {
        if byte == b'\\' {
            let command = String::from_utf8_lossy(&self.osc_buffer).into_owned();
            self.reset();
            vec![AnsiAction::OperatingSystemCommand(command)]
        } else {
            self.osc_buffer.push(0x1b);
            self.osc_buffer.push(byte);
            self.state = AnsiParserState::Osc;
            Vec::new()
        }
    }

    fn charset(&mut self, byte: u8) -> Vec<AnsiAction> {
        self.sequence_buffer.push(byte);
        self.reset();
        Vec::new()
    }

    fn dcs(&mut self, byte: u8) -> Vec<AnsiAction> {
        self.sequence_buffer.push(byte);

        if byte == 0x1b {
            self.reset();
        }

        Vec::new()
    }
}
