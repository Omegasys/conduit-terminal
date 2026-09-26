pub mod regis;
pub mod sixel;
pub mod tek4010;
pub mod tek4014;

pub use regis::{
    RegisCommand,
    RegisParser,
};

pub use sixel::{
    SixelCommand,
    SixelParser,
};

pub use tek4010::Tek4010;
pub use tek4014::Tek4014;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GraphicsProtocol {
    Sixel,
    Regis,
    Tek4010,
    Tek4014,
}

impl GraphicsProtocol {
    pub fn name(self) -> &'static str {
        match self {
            Self::Sixel => "Sixel",
            Self::Regis => "ReGIS",
            Self::Tek4010 => "Tektronix 4010",
            Self::Tek4014 => "Tektronix 4014",
        }
    }
}
