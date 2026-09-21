//! VT100 protocol capabilities.

use super::ansi::AnsiDecoder;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Vt100Capabilities {
    pub rows: u16,
    pub columns: u16,
    pub color: bool,
    pub extended_color: bool,
    pub cursor_keys: bool,
    pub alternate_screen: bool,
    pub origin_mode: bool,
    pub insert_mode: bool,
    pub delete_line: bool,
    pub erase_in_display: bool,
    pub erase_in_line: bool,
    pub device_status_report: bool,
}

impl Default for Vt100Capabilities {
    fn default() -> Self {
        Self {
            rows: 24,
            columns: 80,
            color: false,
            extended_color: false,
            cursor_keys: true,
            alternate_screen: false,
            origin_mode: true,
            insert_mode: true,
            delete_line: true,
            erase_in_display: true,
            erase_in_line: true,
            device_status_report: true,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Vt100Protocol {
    capabilities: Vt100Capabilities,
    decoder: AnsiDecoder,
}

impl Default for Vt100Protocol {
    fn default() -> Self {
        Self::new()
    }
}

impl Vt100Protocol {
    pub fn new() -> Self {
        Self {
            capabilities: Vt100Capabilities::default(),
            decoder: AnsiDecoder::new(),
        }
    }

    pub fn capabilities(&self) -> Vt100Capabilities {
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
