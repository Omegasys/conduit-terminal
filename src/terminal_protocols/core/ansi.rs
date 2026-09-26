#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AnsiSequence {
    Printable(Vec<u8>),
    Control(u8),
    Escape(Vec<u8>),
}

#[derive(Debug, Default)]
pub struct AnsiParser {
    buffer: Vec<u8>,
}

impl AnsiParser {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn reset(&mut self) {
        self.buffer.clear();
    }

    pub fn feed(&mut self, input: &[u8]) -> Vec<AnsiSequence> {
        let mut output = Vec::new();

        for &byte in input {
            match byte {
                0x1B => {
                    if !self.buffer.is_empty() {
                        output.push(AnsiSequence::Printable(
                            std::mem::take(&mut self.buffer),
                        ));
                    }

                    self.buffer.push(byte);
                }

                0x20..=0x7E => {
                    if self.buffer.first() == Some(&0x1B) {
                        self.buffer.push(byte);

                        if byte != b'['
                            && byte != b']'
                            && byte != b'P'
                            && byte != b'^'
                            && byte != b'_'
                        {
                            output.push(AnsiSequence::Escape(
                                std::mem::take(&mut self.buffer),
                            ));
                        }
                    } else {
                        self.buffer.push(byte);
                    }
                }

                value if value < 0x20 || value == 0x7F => {
                    if !self.buffer.is_empty() {
                        output.push(AnsiSequence::Printable(
                            std::mem::take(&mut self.buffer),
                        ));
                    }

                    output.push(AnsiSequence::Control(value));
                }

                _ => self.buffer.push(byte),
            }
        }

        output
    }
}
