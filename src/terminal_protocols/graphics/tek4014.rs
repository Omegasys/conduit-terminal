use super::tek4010::{Tek4010, Tek4010Point};

#[derive(Debug, Default)]
pub struct Tek4014;

impl Tek4014 {
    pub fn new() -> Self {
        Self
    }

    pub fn decode_point(&self, bytes: &[u8]) -> Option<Tek4010Point> {
        Tek4010::new().decode_point(bytes)
    }

    pub fn supports_extended_addressing(&self) -> bool {
        true
    }
}
