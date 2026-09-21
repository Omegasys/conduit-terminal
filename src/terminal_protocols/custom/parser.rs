use super::{
    actions::{CustomProtocolAction, CustomProtocolEvent},
    errors::{CustomProtocolError, CustomProtocolResult},
};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ParserState {
    Ground,
    Escape,
    Custom,
}

pub struct CustomProtocolParser {
    state: ParserState,
    buffer: Vec<u8>,
    maximum_sequence_length: usize,
}

impl CustomProtocolParser {
    pub fn new() -> Self {
        Self {
            state: ParserState::Ground,
            buffer: Vec::new(),
            maximum_sequence_length: 4096,
        }
    }

    pub fn with_maximum_sequence_length(maximum: usize) -> Self {
        Self {
            state: ParserState::Ground,
            buffer: Vec::new(),
            maximum_sequence_length: maximum.max(1),
        }
    }

    pub fn state(&self) -> ParserState {
        self.state
    }

    pub fn reset(&mut self) {
        self.state = ParserState::Ground;
        self.buffer.clear();
    }

    pub fn feed(&mut self, bytes: &[u8]) -> CustomProtocolResult<Vec<CustomProtocolEvent>> {
        let mut events = Vec::new();

        for &byte in bytes {
            match self.state {
                ParserState::Ground => {
                    if byte == 0x1b {
                        self.state = ParserState::Escape;
                        self.buffer.clear();
                    } else {
                        events.push(CustomProtocolEvent::Action(
                            CustomProtocolAction::Print(
                                String::from_utf8_lossy(&[byte]).into_owned(),
                            ),
                        ));
                    }
                }

                ParserState::Escape => {
                    self.buffer.push(byte);

                    if self.buffer.len() > self.maximum_sequence_length {
                        self.reset();

                        return Err(CustomProtocolError::SequenceTooLong {
                            maximum: self.maximum_sequence_length,
                        });
                    }

                    if byte == b'[' || byte == b']' || byte == b'P' {
                        self.state = ParserState::Custom;
                    } else {
                        let payload = std::mem::take(&mut self.buffer);

                        events.push(CustomProtocolEvent::ProtocolMessage {
                            name: "escape-sequence".to_string(),
                            data: payload,
                        });

                        self.state = ParserState::Ground;
                    }
                }

                ParserState::Custom => {
                    self.buffer.push(byte);

                    if self.buffer.len() > self.maximum_sequence_length {
                        self.reset();

                        return Err(CustomProtocolError::SequenceTooLong {
                            maximum: self.maximum_sequence_length,
                        });
                    }

                    if byte == b'm' || byte == b'~' || byte == b'\\' {
                        let payload = std::mem::take(&mut self.buffer);

                        events.push(CustomProtocolEvent::ProtocolMessage {
                            name: "custom-sequence".to_string(),
                            data: payload,
                        });

                        self.state = ParserState::Ground;
                    }
                }
            }
        }

        Ok(events)
    }
}

impl Default for CustomProtocolParser {
    fn default() -> Self {
        Self::new()
    }
}
