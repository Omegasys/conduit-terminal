#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Tek4010Point {
    pub x: u16,
    pub y: u16,
}

#[derive(Debug, Default)]
pub struct Tek4010;

impl Tek4010 {
    pub fn new() -> Self {
        Self
    }

    pub fn decode_point(&self, bytes: &[u8]) -> Option<Tek4010Point> {
        if bytes.len() < 4 {
            return None;
        }

        let x = (((bytes[0] & 0x1F) as u16) << 5)
            | ((bytes[1] & 0x1F) as u16);

        let y = (((bytes[2] & 0x1F) as u16) << 5)
            | ((bytes[3] & 0x1F) as u16);

        Some(Tek4010Point { x, y })
    }
}
