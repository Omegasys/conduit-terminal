/// Controls whether searches distinguish between upper- and lower-case text.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum CaseSensitivity {
    #[default]
    Insensitive,
    Sensitive,
}

impl CaseSensitivity {
    pub fn is_sensitive(self) -> bool {
        matches!(self, Self::Sensitive)
    }

    pub fn normalize(self, value: &str) -> String {
        match self {
            Self::Sensitive => value.to_owned(),
            Self::Insensitive => value.to_lowercase(),
        }
    }
}
