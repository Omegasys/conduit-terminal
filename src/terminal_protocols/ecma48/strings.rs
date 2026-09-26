#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ControlStringKind {
    Osc,
    Dcs,
    Apc,
    Pm,
    Sos,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ControlString {
    pub kind: ControlStringKind,
    pub data: Vec<u8>,
}

impl ControlString {
    pub fn new(kind: ControlStringKind, data: impl Into<Vec<u8>>) -> Self {
        Self {
            kind,
            data: data.into(),
        }
    }

    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }
}
