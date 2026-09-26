use super::{Vt320, VtCapabilities, VtLevel};

#[derive(Debug, Clone, Copy, Default)]
pub struct Vt340;

impl Vt340 {
    pub fn level(&self) -> VtLevel {
        VtLevel::Vt340
    }

    pub fn capabilities(&self) -> VtCapabilities {
        VtCapabilities::for_level(self.level())
    }

    pub fn base(&self) -> Vt320 {
        Vt320
    }

    pub fn supports_sixel(&self) -> bool {
        true
    }

    pub fn supports_regis(&self) -> bool {
        true
    }
}
