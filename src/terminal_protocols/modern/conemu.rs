#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ConEmuProgress {
    pub active: bool,
    pub percent: Option<u8>,
}

#[derive(Debug, Default)]
pub struct ConEmuProtocol;

impl ConEmuProtocol {
    pub fn new() -> Self {
        Self
    }

    pub fn progress(&self, value: u8) -> ConEmuProgress {
        ConEmuProgress {
            active: true,
            percent: Some(value.min(100)),
        }
    }

    pub fn clear(&self) -> ConEmuProgress {
        ConEmuProgress {
            active: false,
            percent: None,
        }
    }
}
