//! VT220 protocol capabilities.

use super::ansi::AnsiDecoder;
use super::vt100::Vt100Capabilities;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Vt220Capabilities {
    pub base: Vt100Capabilities,
    pub eight_bit_controls: bool,
    pub selectable_charsets: bool,
    pub insert_characters: bool,
    pub delete_characters: bool,
    pub function_keys: bool,
    pub numeric_keypad: bool,
    pub rectangular_operations: bool,
}

impl Default for Vt220Capabilities {
    fn default() -> Self {
        let base = Vt100Capabilities {
            color: false,
            alternate_screen: true,
            ..Default::default()
        };

        Self {
            base,
            eight_bit_controls: true,
            selectable_charsets: true,
            insert_characters: true,
            delete_characters: true,
            function_keys: true,
            numeric_keypad: true,
            rectangular_operations: false,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Vt220Protocol {
    capabilities: Vt220Capabilities,
    decoder: AnsiDecoder,
}

impl Default for Vt220Protocol {
    fn default() -> Self {
        Self::new()
    }
}

impl Vt220Protocol {
    pub fn new() -> Self {
        Self {
            capabilities: Vt220Capabilities::default(),
            decoder: AnsiDecoder::new(),
        }
    }

    pub fn capabilities(&self) -> Vt220Capabilities {
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
