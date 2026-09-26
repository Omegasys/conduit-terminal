#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VteProtocol {
    Standard,
    Extended,
}

impl Default for VteProtocol {
    fn default() -> Self {
        Self::Standard
    }
}

impl VteProtocol {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn supports_extended_sequences(self) -> bool {
        matches!(self, Self::Extended)
    }
}
