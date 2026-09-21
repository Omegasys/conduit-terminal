#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PasteDecision {
    Allowed,
    RequiresConfirmation,
    Blocked,
    LargePaste {
        bytes: usize,
        lines: usize,
    },
}

impl PasteDecision {
    pub fn is_allowed(&self) -> bool {
        matches!(self, Self::Allowed | Self::LargePaste { .. })
    }

    pub fn requires_confirmation(&self) -> bool {
        matches!(self, Self::RequiresConfirmation)
    }

    pub fn is_blocked(&self) -> bool {
        matches!(self, Self::Blocked)
    }
}

#[derive(Debug, Clone)]
pub struct PasteProtectionConfig {
    pub enabled: bool,
    pub confirm_multiline: bool,
    pub confirm_control_characters: bool,
    pub block_null_bytes: bool,
    pub maximum_unconfirmed_lines: usize,
}

impl Default for PasteProtectionConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            confirm_multiline: true,
            confirm_control_characters: true,
            block_null_bytes: true,
            maximum_unconfirmed_lines: 1,
        }
    }
}

#[derive(Debug, Clone)]
pub struct PasteProtection {
    config: PasteProtectionConfig,
}

impl Default for PasteProtection {
    fn default() -> Self {
        Self::new(PasteProtectionConfig::default())
    }
}

impl PasteProtection {
    pub fn new(config: PasteProtectionConfig) -> Self {
        Self { config }
    }

    pub fn config(&self) -> &PasteProtectionConfig {
        &self.config
    }

    pub fn config_mut(&mut self) -> &mut PasteProtectionConfig {
        &mut self.config
    }

    pub fn check(&self, text: &str) -> PasteDecision {
        if !self.config.enabled {
            return PasteDecision::Allowed;
        }

        if self.config.block_null_bytes && text.contains('\0') {
            return PasteDecision::Blocked;
        }

        if self.config.confirm_control_characters && contains_control_characters(text) {
            return PasteDecision::RequiresConfirmation;
        }

        let lines = text.lines().count().max(1);

        if self.config.confirm_multiline
            && lines > self.config.maximum_unconfirmed_lines
        {
            return PasteDecision::RequiresConfirmation;
        }

        PasteDecision::Allowed
    }
}

fn contains_control_characters(text: &str) -> bool {
    text.chars().any(|character| {
        character.is_control()
            && character != '\n'
            && character != '\r'
            && character != '\t'
    })
}
