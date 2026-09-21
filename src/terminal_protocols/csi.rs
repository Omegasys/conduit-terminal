//! CSI (Control Sequence Introducer) parsing.
//!
//! CSI sequences generally have the form:
//!
//! ESC [ [private/intermediate bytes] parameters final
//!
//! This module parses the structure and leaves interpretation of individual
//! commands to higher terminal layers.

use std::fmt;

/// CSI parser state.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CsiState {
    Ground,
    Parameter,
    Intermediate,
    Finished,
}

/// Parsed CSI sequence.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CsiSequence {
    pub private: Option<u8>,
    pub parameters: Vec<Option<u16>>,
    pub intermediates: Vec<u8>,
    pub final_byte: u8,
}

impl CsiSequence {
    pub fn new(final_byte: u8) -> Self {
        Self {
            private: None,
            parameters: Vec::new(),
            intermediates: Vec::new(),
            final_byte,
        }
    }

    pub fn parameter(&self, index: usize, default: u16) -> u16 {
        self.parameters
            .get(index)
            .and_then(|value| *value)
            .filter(|value| *value != 0)
            .unwrap_or(default)
    }

    pub fn has_private_marker(&self, marker: u8) -> bool {
        self.private == Some(marker)
    }

    pub fn is_private(&self) -> bool {
        self.private.is_some()
    }

    pub fn is_sgr(&self) -> bool {
        self.final_byte == b'm'
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CsiError {
    InvalidByte(u8),
    MissingFinalByte,
    ParameterOverflow,
}

impl fmt::Display for CsiError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidByte(byte) => {
                write!(f, "invalid CSI byte: 0x{byte:02x}")
            }
            Self::MissingFinalByte => write!(f, "CSI sequence has no final byte"),
            Self::ParameterOverflow => write!(f, "CSI parameter overflow"),
        }
    }
}

impl std::error::Error for CsiError {}

#[derive(Debug, Clone)]
pub struct CsiParser {
    state: CsiState,
    private: Option<u8>,
    parameters: Vec<Option<u16>>,
    current_parameter: Option<u16>,
    intermediates: Vec<u8>,
}

impl Default for CsiParser {
    fn default() -> Self {
        Self::new()
    }
}

impl CsiParser {
    pub fn new() -> Self {
        Self {
            state: CsiState::Ground,
            private: None,
            parameters: Vec::new(),
            current_parameter: None,
            intermediates: Vec::new(),
        }
    }

    pub fn state(&self) -> CsiState {
        self.state
    }

    pub fn reset(&mut self) {
        self.state = CsiState::Ground;
        self.private = None;
        self.parameters.clear();
        self.current_parameter = None;
        self.intermediates.clear();
    }

    pub fn parse(&mut self, bytes: &[u8]) -> Result<Option<CsiSequence>, CsiError> {
        self.reset();
        self.state = CsiState::Parameter;

        for &byte in bytes {
            if let Some(sequence) = self.feed(byte)? {
                return Ok(Some(sequence));
            }
        }

        Ok(None)
    }

    pub fn feed(&mut self, byte: u8) -> Result<Option<CsiSequence>, CsiError> {
        match self.state {
            CsiState::Ground => {
                if byte == 0x1b {
                    self.state = CsiState::Parameter;
                    Ok(None)
                } else {
                    Err(CsiError::InvalidByte(byte))
                }
            }

            CsiState::Parameter => match byte {
                b'?' | b'>' | b'!' | b'=' if self.parameters.is_empty() => {
                    self.private = Some(byte);
                    Ok(None)
                }

                b'0'..=b'9' => {
                    let current = self.current_parameter.unwrap_or(0);

                    let next = current
                        .checked_mul(10)
                        .and_then(|value| value.checked_add((byte - b'0') as u16))
                        .ok_or(CsiError::ParameterOverflow)?;

                    self.current_parameter = Some(next);
                    Ok(None)
                }

                b';' | b':' => {
                    self.parameters.push(self.current_parameter.take());
                    Ok(None)
                }

                0x20..=0x2f => {
                    if let Some(value) = self.current_parameter.take() {
                        self.parameters.push(Some(value));
                    }

                    self.intermediates.push(byte);
                    self.state = CsiState::Intermediate;
                    Ok(None)
                }

                0x40..=0x7e => {
                    if let Some(value) = self.current_parameter.take() {
                        self.parameters.push(Some(value));
                    } else if !self.parameters.is_empty() {
                        self.parameters.push(None);
                    }

                    let sequence = CsiSequence {
                        private: self.private,
                        parameters: self.parameters.clone(),
                        intermediates: self.intermediates.clone(),
                        final_byte: byte,
                    };

                    self.state = CsiState::Finished;

                    Ok(Some(sequence))
                }

                _ => Err(CsiError::InvalidByte(byte)),
            },

            CsiState::Intermediate => match byte {
                0x20..=0x2f => {
                    self.intermediates.push(byte);
                    Ok(None)
                }

                0x40..=0x7e => {
                    let sequence = CsiSequence {
                        private: self.private,
                        parameters: self.parameters.clone(),
                        intermediates: self.intermediates.clone(),
                        final_byte: byte,
                    };

                    self.state = CsiState::Finished;

                    Ok(Some(sequence))
                }

                _ => Err(CsiError::InvalidByte(byte)),
            },

            CsiState::Finished => {
                self.reset();
                self.feed(byte)
            }
        }
    }
}
