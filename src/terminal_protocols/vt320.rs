//! VT320 protocol capabilities.

use super::ansi::AnsiDecoder;
use super::vt220::Vt220Capabilities;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Vt320Capabilities {
    pub base: Vt220Capabilities,
    pub 132_column_mode: bool,
    pub smooth_scroll: bool,
    pub autowrap: bool,
    pub origin_mode: bool,
    pub protected_fields: bool,
    pub rectangular_erase: bool,
    pub rectangular_fill: bool,
    pub rectangular_copy: bool,
    pub rectangular_move: bool,
}

impl Default for Vt320Capabilities {
    fn default() -> Self {
        Self {
            base: Vt220Capabilities::default(),
            132_column_mode: true,
            smooth_scroll: true,
            autowrap: true,
            origin_mode: true,
            protected_fields: true,
            rectangular_erase: true,
            rectangular_fill: true,
            rectangular_copy: true,
            rectangular_move: true,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Vt320Protocol {
    capabilities: Vt320Capabilities,
    decoder: AnsiDecoder,
}

impl Default for Vt320Protocol {
    fn default() -> Self {
        Self::new()
    }
}

impl Vt320Protocol {
    pub fn new() -> Self {
        Self {
            capabilities: Vt320Capabilities::default(),
            decoder: AnsiDecoder::new(),
        }
    }

    pub fn capabilities(&self) -> Vt320Capabilities {
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
