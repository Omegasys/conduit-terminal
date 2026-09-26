use super::{Vt100, VtCapabilities, VtLevel};

#[derive(Debug, Clone, Copy, Default)]
pub struct Vt125;

impl Vt125 {
    pub fn level(&self) -> VtLevel {
        VtLevel::Vt125
    }

    pub fn capabilities(&self) -> VtCapabilities {
        VtCapabilities::for_level(self.level())
    }

    pub fn base(&self) -> Vt100 {
        Vt100
    }

    pub fn supports_sixel(&self) -> bool {
        true
    }

    pub fn supports_regis(&self) -> bool {
        true
    }
}
