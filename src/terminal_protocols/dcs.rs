//! DCS (Device Control String) parsing.
//!
//! DCS is intentionally kept as a generic container here. Different terminal
//! families use DCS for different features, including sixel, DECRQSS,
//! programmable keys, and other device-specific functionality.

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DcsSequence {
    pub parameters: Vec<Option<u16>>,
    pub intermediates: Vec<u8>,
    pub final_byte: Option<u8>,
    pub payload: Vec<u8>,
}

impl DcsSequence {
    pub fn new() -> Self {
        Self {
            parameters: Vec::new(),
            intermediates: Vec::new(),
            final_byte: None,
            payload: Vec::new(),
        }
    }

    pub fn is_sixel(&self) -> bool {
        self.final_byte == Some(b'q')
    }

    pub fn is_decrqss(&self) -> bool {
        self.intermediates.contains(&b'$')
            && self.final_byte == Some(b'q')
    }

    pub fn payload_as_string(&self) -> String {
        String::from_utf8_lossy(&self.payload).into_owned()
    }
}

impl Default for DcsSequence {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DcsState {
    Ground,
    Parameter,
    Intermediate,
    Payload,
}

#[derive(Debug, Clone)]
pub struct DcsParser {
    state: DcsState,
    sequence: DcsSequence,
    current_parameter: Option<u16>,
}

impl Default for DcsParser {
    fn default() -> Self {
        Self::new()
    }
}

impl DcsParser {
    pub fn new() -> Self {
        Self {
            state: DcsState::Ground,
            sequence: DcsSequence::new(),
            current_parameter: None,
        }
    }

    pub fn reset(&mut self) {
        self.state = DcsState::Ground;
        self.sequence = DcsSequence::new();
        self.current_parameter = None;
    }

    pub fn begin(&mut self) {
        self.reset();
        self.state = DcsState::Parameter;
    }

    pub fn feed(&mut self, byte: u8) -> Option<DcsSequence> {
        match self.state {
            DcsState::Ground => {
                if byte == b'P' {
                    self.begin();
                }
            }

            DcsState::Parameter => match byte {
                b'0'..=b'9' => {
                    let current = self.current_parameter.unwrap_or(0);

                    self.current_parameter = Some(
                        current
                            .saturating_mul(10)
                            .saturating_add((byte - b'0') as u16),
                    );
                }

                b';' => {
                    self.sequence
                        .parameters
                        .push(self.current_parameter.take());
                }

                0x20..=0x2f => {
                    if let Some(value) = self.current_parameter.take() {
                        self.sequence.parameters.push(Some(value));
                    }

                    self.sequence.intermediates.push(byte);
                    self.state = DcsState::Intermediate;
                }

                0x40..=0x7e => {
                    if let Some(value) = self.current_parameter.take() {
                        self.sequence.parameters.push(Some(value));
                    }

                    self.sequence.final_byte = Some(byte);
                    self.state = DcsState::Payload;
                }

                _ => {}
            },

            DcsState::Intermediate => match byte {
                0x20..=0x2f => {
                    self.sequence.intermediates.push(byte);
                }

                0x40..=0x7e => {
                    self.sequence.final_byte = Some(byte);
                    self.state = DcsState::Payload;
                }

                _ => {}
            },

            DcsState::Payload => {
                if byte == 0x1b {
                    return Some(self.take_sequence());
                }

                self.sequence.payload.push(byte);
            }
        }

        None
    }

    pub fn finish(&mut self) -> DcsSequence {
        self.take_sequence()
    }

    fn take_sequence(&mut self) -> DcsSequence {
        let sequence = std::mem::take(&mut self.sequence);
        self.reset();
        sequence
    }
}
