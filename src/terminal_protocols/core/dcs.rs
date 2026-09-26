#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DcsSequence {
    pub command: Option<u8>,
    pub parameters: Vec<u16>,
    pub data: Vec<u8>,
}

impl DcsSequence {
    pub fn new(data: impl Into<Vec<u8>>) -> Self {
        Self {
            command: None,
            parameters: Vec::new(),
            data: data.into(),
        }
    }
}

#[derive(Debug, Default)]
pub struct DcsParser {
    active: bool,
    buffer: Vec<u8>,
}

impl DcsParser {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn begin(&mut self) {
        self.active = true;
        self.buffer.clear();
    }

    pub fn reset(&mut self) {
        self.active = false;
        self.buffer.clear();
    }

    pub fn feed_byte(&mut self, byte: u8) -> Option<DcsSequence> {
        if !self.active {
            return None;
        }

        if byte == 0x1B {
            self.buffer.push(byte);
            return None;
        }

        if byte == b'\\' && self.buffer.last() == Some(&0x1B) {
            self.buffer.pop();
            return self.finish();
        }

        self.buffer.push(byte);
        None
    }

    fn finish(&mut self) -> Option<DcsSequence> {
        let data = std::mem::take(&mut self.buffer);
        self.active = false;

        if data.is_empty() {
            return Some(DcsSequence::new(Vec::new()));
        }

        Some(DcsSequence::new(data))
    }
}
