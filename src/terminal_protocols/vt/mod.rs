pub mod vt52;
pub mod vt100;
pub mod vt102;
pub mod vt125;
pub mod vt131;
pub mod vt180;
pub mod vt220;
pub mod vt240;
pub mod vt320;
pub mod vt340;
pub mod vt420;
pub mod vt520;
pub mod vt525;

pub use vt52::Vt52;
pub use vt100::Vt100;
pub use vt102::Vt102;
pub use vt125::Vt125;
pub use vt131::Vt131;
pub use vt180::Vt180;
pub use vt220::Vt220;
pub use vt240::Vt240;
pub use vt320::Vt320;
pub use vt340::Vt340;
pub use vt420::Vt420;
pub use vt520::Vt520;
pub use vt525::Vt525;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum VtLevel {
    Vt52,
    Vt100,
    Vt102,
    Vt125,
    Vt131,
    Vt180,
    Vt220,
    Vt240,
    Vt320,
    Vt340,
    Vt420,
    Vt520,
    Vt525,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct VtCapabilities {
    pub level: VtLevel,
    pub sixel: bool,
    pub regis: bool,
    pub advanced_keyboard: bool,
    pub color: bool,
    pub double_height: bool,
    pub double_width: bool,
}

impl VtCapabilities {
    pub fn for_level(level: VtLevel) -> Self {
        let sixel = matches!(
            level,
            VtLevel::Vt125
                | VtLevel::Vt240
                | VtLevel::Vt340
        );

        let regis = matches!(
            level,
            VtLevel::Vt125
                | VtLevel::Vt240
                | VtLevel::Vt340
        );

        let color = matches!(
            level,
            VtLevel::Vt125
                | VtLevel::Vt240
                | VtLevel::Vt340
                | VtLevel::Vt420
                | VtLevel::Vt520
                | VtLevel::Vt525
        );

        Self {
            level,
            sixel,
            regis,
            advanced_keyboard: level >= VtLevel::Vt220,
            color,
            double_height: level >= VtLevel::Vt100,
            double_width: level >= VtLevel::Vt100,
        }
    }
}
