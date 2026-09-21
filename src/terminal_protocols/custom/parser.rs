use super::errors::{
    CustomProtocolError,
    CustomProtocolResult,
};

#[derive(Debug, Clone, Default)]
pub struct ProtocolParseResult {
    pub consumed: usize,
    pub events: Vec<Vec<u8>>,
    pub output: Vec<u8>,
    pub complete: bool,
}

impl ProtocolParseResult {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add_event(
        &mut self,
        event: Vec<u8>,
    ) {
        self.events.push(event);
    }

    pub fn set_output(
        &mut self,
        output: Vec<u8>,
    ) {
        self.output = output;
    }

    pub fn set_consumed(
        &mut self,
        consumed: usize,
    ) {
        self.consumed = consumed;
    }

    pub fn complete(&mut self) {
        self.complete = true;
    }
}

#[derive(Debug)]
pub struct CustomProtocolParser {
    buffer: Vec<u8>,
    maximum_buffer_size: usize,
}

impl CustomProtocolParser {
    pub fn new(
        maximum_buffer_size: usize,
    ) -> Self {
        Self {
            buffer: Vec::new(),
            maximum_buffer_size,
        }
    }

    pub fn feed(
        &mut self,
        input: &[u8],
    ) -> CustomProtocolResult<ProtocolParseResult> {
        if self.buffer.len() + input.len()
            > self.maximum_buffer_size
        {
            return Err(CustomProtocolError::BufferTooLarge {
                maximum: self.maximum_buffer_size,
            });
        }

        self.buffer.extend_from_slice(input);

        let mut result = ProtocolParseResult::new();

        result.set_consumed(input.len());
        result.set_output(input.to_vec());

        if !self.buffer.is_empty() {
            result.complete();
        }

        Ok(result)
    }

    pub fn buffer(&self) -> &[u8] {
        &self.buffer
    }

    pub fn clear(&mut self) {
        self.buffer.clear();
    }

    pub fn reset(&mut self) {
        self.clear();
    }
}

impl Default for CustomProtocolParser {
    fn default() -> Self {
        Self::new(1024 * 1024)
    }
}
