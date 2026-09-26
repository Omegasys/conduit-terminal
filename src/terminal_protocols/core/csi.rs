#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CsiSequence {
    pub private: Option<u8>,
    pub parameters: Vec<u16>,
    pub intermediate: Vec<u8>,
    pub final_byte: u8,
}

impl CsiSequence {
    pub fn new(final_byte: u8) -> Self {
        Self {
            private: None,
            parameters: Vec::new(),
            intermediate: Vec::new(),
            final_byte,
        }
    }

    pub fn parameter(&self, index: usize) -> Option<u16> {
        self.parameters.get(index).copied()
    }

    pub fn first_parameter_or(&self, default: u16) -> u16 {
        self.parameter(0).unwrap_or(default)
    }

    pub fn is_private(&self) -> bool {
        self.private.is_some()
    }
}

#[derive(Debug, Default)]
pub struct CsiParser {
    active: bool,
    private: Option<u8>,
    parameter_buffer: Vec<u8>,
    intermediate: Vec<u8>,
}

impl CsiParser {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn reset(&mut self) {
        self.active = false;
        self.private = None;
        self.parameter_buffer.clear();
        self.intermediate.clear();
    }

    pub fn begin(&mut self) {
        self.reset();
        self.active = true;
    }

    pub fn feed_byte(&mut self, byte: u8) -> Option<CsiSequence> {
        if !self.active {
            return None;
        }

        match byte {
            b'?' | b'>' | b'!' | b'='
                if self.private.is_none() && self.parameter_buffer.is_empty() =>
            {
                self.private = Some(byte);
                None
            }

            b'0'..=b'9' | b';' | b':' => {
                self.parameter_buffer.push(byte);
                None
            }

            0x20..=0x2F => {
                self.intermediate.push(byte);
                None
            }

            0x40..=0x7E => {
                let sequence = self.finish(byte);
                Some(sequence)
            }

            _ => {
                self.reset();
                None
            }
        }
    }

    fn finish(&mut self, final_byte: u8) -> CsiSequence {
        let parameters = parse_parameters(&self.parameter_buffer);

        let sequence = CsiSequence {
            private: self.private,
            parameters,
            intermediate: self.intermediate.clone(),
            final_byte,
        };

        self.reset();
        sequence
    }
}

fn parse_parameters(buffer: &[u8]) -> Vec<u16> {
    if buffer.is_empty() {
        return Vec::new();
    }

    buffer
        .split(|byte| *byte == b';' || *byte == b':')
        .map(|part| {
            if part.is_empty() {
                0
            } else {
                std::str::from_utf8(part)
                    .ok()
                    .and_then(|value| value.parse::<u16>().ok())
                    .unwrap_or(0)
            }
        })
        .collect()
}
