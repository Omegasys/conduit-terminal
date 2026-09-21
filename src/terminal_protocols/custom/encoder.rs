use super::errors::CustomProtocolResult;

#[derive(Debug, Clone, Default)]
pub struct ProtocolEncodeResult {
    pub data: Vec<u8>,
}

impl ProtocolEncodeResult {
    pub fn new(data: Vec<u8>) -> Self {
        Self { data }
    }
}

#[derive(Debug, Default)]
pub struct CustomProtocolEncoder;

impl CustomProtocolEncoder {
    pub fn new() -> Self {
        Self
    }

    pub fn encode(
        &self,
        data: &[u8],
    ) -> CustomProtocolResult<ProtocolEncodeResult> {
        Ok(ProtocolEncodeResult::new(data.to_vec()))
    }
}
