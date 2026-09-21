//! VT520 protocol capabilities.
//!
//! VT520 support is represented as an extended capability profile. The
//! terminal core can selectively enable functionality instead of pretending
//! that every historical VT520 feature must be emulated.

use super::ansi::AnsiDecoder;
use super::vt420::Vt420Capabilities;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Vt520Capabilities {
    pub base: Vt420Capabilities,
    pub multiple_sessions: bool,
    pub extended_status_reporting: bool,
    pub programmable_keys: bool,
    pub soft_character_sets: bool,
    pub national_replacement_characters: bool,
    pub advanced_editing: bool,
    pub locator_support: bool,
    pub macro_support: bool,
}

impl Default for Vt520Capabilities {
    fn default() -> Self {
        Self {
            base: Vt420Capabilities::default(),
            multiple_sessions: true,
            extended_status_reporting: true,
            programmable_keys: true,
            soft_character_sets: true,
            national_replacement_characters: true,
            advanced_editing: true,
            locator_support: true,
            macro_support: true,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Vt520Protocol {
    capabilities: Vt520Capabilities,
    decoder: AnsiDecoder,
}

impl Default for Vt520Protocol {
    fn default() -> Self {
        Self::new()
    }
}

impl Vt520Protocol {
    pub fn new() -> Self {
        Self {
            capabilities: Vt520Capabilities::default(),
            decoder: AnsiDecoder::new(),
        }
    }

    pub fn capabilities(&self) -> Vt520Capabilities {
        self.capabilities
    }

    pub fn decoder(&self) -> &AnsiDecoder {
        &self.decoder
    }

    pub fn decoder_mut(&mut self) -> &mut AnsiDecoder {
        &mut self.decoder
    }

    pub fn reset(&mut self) {
        self.decoder.reset();
    }
}
