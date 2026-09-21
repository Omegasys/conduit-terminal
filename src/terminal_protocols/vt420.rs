//! VT420 protocol capabilities.

use super::ansi::AnsiDecoder;
use super::vt320::Vt320Capabilities;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Vt420Capabilities {
    pub base: Vt320Capabilities,
    pub 8_bit_color: bool,
    pub rectangular_select: bool,
    pub rectangular_editing: bool,
    pub selective_erase: bool,
    pub user_preferred_charset: bool,
    pub device_attributes: bool,
    pub status_line: bool,
}

impl Default for Vt420Capabilities {
    fn default() -> Self {
        Self {
            base: Vt320Capabilities::default(),
            8_bit_color: true,
            rectangular_select: true,
            rectangular_editing: true,
            selective_erase: true,
            user_preferred_charset: true,
            device_attributes: true,
            status_line: true,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Vt420Protocol {
    capabilities: Vt420Capabilities,
    decoder: AnsiDecoder,
}

impl Default for Vt420Protocol {
    fn default() -> Self {
        Self::new()
    }
}

impl Vt420Protocol {
    pub fn new() -> Self {
        Self {
            capabilities: Vt420Capabilities::default(),
            decoder: AnsiDecoder::new(),
        }
    }

    pub fn capabilities(&self) -> Vt420Capabilities {
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
