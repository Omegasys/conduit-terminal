#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OscSequence {
    pub command: String,
    pub data: String,
}

impl OscSequence {
    pub fn new(
        command: impl Into<String>,
        data: impl Into<String>,
    ) -> Self {
        Self {
            command: command.into(),
            data: data.into(),
        }
    }

    pub fn numeric_command(&self) -> Option<u16> {
        self.command.parse().ok()
    }

    pub fn data_parts(&self) -> impl Iterator<Item = &str> {
        self.data.split(';')
    }
}

#[derive(Debug, Default)]
pub struct OscParser {
    active: bool,
    buffer: Vec<u8>,
}

impl OscParser {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn reset(&mut self) {
        self.active = false;
        self.buffer.clear();
    }

    pub fn begin(&mut self) {
        self.reset();
        self.active = true;
    }

    pub fn feed_byte(&mut self, byte: u8) -> Option<OscSequence> {
        if !self.active {
            return None;
        }

        match byte {
            0x07 => self.finish(),

            0x1B => {
                self.buffer.push(byte);
                None
            }

            b'\\' if self.buffer.last() == Some(&0x1B) => {
                self.buffer.pop();
                self.finish()
            }

            _ => {
                self.buffer.push(byte);
                None
            }
        }
    }

    fn finish(&mut self) -> Option<OscSequence> {
        let data = std::mem::take(&mut self.buffer);

        self.active = false;

        let text = String::from_utf8_lossy(&data);

        let mut parts = text.splitn(2, ';');

        let command = parts.next()?.to_string();
        let data = parts.next().unwrap_or_default().to_string();

        Some(OscSequence::new(command, data))
    }
}
