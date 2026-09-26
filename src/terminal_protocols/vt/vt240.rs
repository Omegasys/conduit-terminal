use super::{Vt220, VtCapabilities, VtLevel};

#[derive(Debug, Clone, Copy, Default)]
pub struct Vt240;

impl Vt240 {
    pub fn level(&self) -> VtLevel {
        VtLevel::Vt240
    }

    pub fn capabilities(&self) -> VtCapabilities {
        VtCapabilities::for_level(self.level())
    }

    pub fn base(&self) -> Vt220 {
        Vt220
    }

    pub fn supports_sixel(&self) -> bool {
        true
    }

    pub fn supports_regis(&self) -> bool {
        true
    }
}
