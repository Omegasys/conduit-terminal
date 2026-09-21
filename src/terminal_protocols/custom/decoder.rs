use super::{
    errors::CustomProtocolResult,
    parser::{
        CustomProtocolParser,
        ProtocolParseResult,
    },
};

#[derive(Debug, Default)]
pub struct CustomProtocolDecoder {
    parser: CustomProtocolParser,
}

impl CustomProtocolDecoder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn decode(
        &mut self,
        input: &[u8],
    ) -> CustomProtocolResult<ProtocolParseResult> {
        self.parser.feed(input)
    }

    pub fn parser(&self) -> &CustomProtocolParser {
        &self.parser
    }

    pub fn parser_mut(&mut self) -> &mut CustomProtocolParser {
        &mut self.parser
    }

    pub fn reset(&mut self) {
        self.parser.reset();
    }
}
