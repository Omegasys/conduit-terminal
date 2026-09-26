#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ApcSequence {
    pub data: Vec<u8>,
}

impl ApcSequence {
    pub fn new(data: impl Into<Vec<u8>>) -> Self {
        Self { data: data.into() }
    }
}

#[derive(Debug, Default)]
pub struct ApcParser {
    active: bool,
    buffer: Vec<u8>,
}

impl ApcParser {
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

    pub fn feed_byte(&mut self, byte: u8) -> Option<ApcSequence> {
        if !self.active {
            return None;
        }

        if byte == 0x1B {
            self.buffer.push(byte);
            return None;
        }

        if byte == b'\\' && self.buffer.last() == Some(&0x1B) {
            self.buffer.pop();

            let data = std::mem::take(&mut self.buffer);
            self.active = false;

            return Some(ApcSequence::new(data));
        }

        self.buffer.push(byte);
        None
    }
}
