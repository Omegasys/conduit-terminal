//! DEC Sixel graphics protocol support.
//!
//! This module parses Sixel data into a renderer-independent representation.
//! Decoding into an actual GPU texture belongs in the renderer/graphics
//! subsystem.

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SixelColor {
    pub red: u8,
    pub green: u8,
    pub blue: u8,
}

impl SixelColor {
    pub const BLACK: Self = Self {
        red: 0,
        green: 0,
        blue: 0,
    };

    pub fn new(red: u8, green: u8, blue: u8) -> Self {
        Self {
            red,
            green,
            blue,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SixelPalette {
    colors: Vec<SixelColor>,
}

impl Default for SixelPalette {
    fn default() -> Self {
        Self::new()
    }
}

impl SixelPalette {
    pub fn new() -> Self {
        Self {
            colors: vec![SixelColor::BLACK; 256],
        }
    }

    pub fn get(&self, index: usize) -> Option<SixelColor> {
        self.colors.get(index).copied()
    }

    pub fn set(&mut self, index: usize, color: SixelColor) {
        if index >= self.colors.len() {
            self.colors.resize(index + 1, SixelColor::BLACK);
        }

        self.colors[index] = color;
    }

    pub fn len(&self) -> usize {
        self.colors.len()
    }

    pub fn is_empty(&self) -> bool {
        self.colors.is_empty()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SixelImage {
    pub width: u32,
    pub height: u32,
    pub pixels: Vec<u8>,
    pub palette: SixelPalette,
}

impl SixelImage {
    pub fn new(width: u32, height: u32) -> Self {
        let pixel_count = width
            .saturating_mul(height)
            .saturating_mul(4) as usize;

        Self {
            width,
            height,
            pixels: vec![0; pixel_count],
            palette: SixelPalette::new(),
        }
    }

    pub fn pixel_offset(&self, x: u32, y: u32) -> Option<usize> {
        if x >= self.width || y >= self.height {
            return None;
        }

        Some(((y * self.width + x) * 4) as usize)
    }

    pub fn set_pixel(
        &mut self,
        x: u32,
        y: u32,
        color: SixelColor,
    ) {
        if let Some(offset) = self.pixel_offset(x, y) {
            self.pixels[offset] = color.red;
            self.pixels[offset + 1] = color.green;
            self.pixels[offset + 2] = color.blue;
            self.pixels[offset + 3] = 255;
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SixelState {
    Ground,
    RasterAttributes,
    ColorDefinition,
    Data,
}

#[derive(Debug, Clone)]
pub struct SixelDecoder {
    state: SixelState,
    palette: SixelPalette,
    current_color: usize,
    x: u32,
    y: u32,
    max_x: u32,
    max_y: u32,
}

impl Default for SixelDecoder {
    fn default() -> Self {
        Self::new()
    }
}

impl SixelDecoder {
    pub fn new() -> Self {
        Self {
            state: SixelState::Ground,
            palette: SixelPalette::new(),
            current_color: 0,
            x: 0,
            y: 0,
            max_x: 0,
            max_y: 0,
        }
    }

    pub fn reset(&mut self) {
        self.state = SixelState::Ground;
        self.current_color = 0;
        self.x = 0;
        self.y = 0;
        self.max_x = 0;
        self.max_y = 0;
    }

    pub fn palette(&self) -> &SixelPalette {
        &self.palette
    }

    pub fn set_color(&mut self, index: usize, color: SixelColor) {
        self.palette.set(index, color);
    }

    pub fn set_current_color(&mut self, index: usize) {
        self.current_color = index;
    }

    pub fn decode(&mut self, data: &[u8]) -> SixelImage {
        self.reset();

        self.state = SixelState::Data;

        for &byte in data {
            match byte {
                b'?'..=b'~' => {
                    self.decode_sixel(byte);
                }

                b'$' => {
                    self.x = 0;
                }

                b'-' => {
                    self.x = 0;
                    self.y = self.y.saturating_add(6);
                }

                b'#' => {
                    self.state = SixelState::ColorDefinition;
                }

                _ => {}
            }
        }

        let width = self.max_x.max(self.x).saturating_add(1);
        let height = self.max_y.saturating_add(1);

        let mut image = SixelImage::new(width, height);
        image.palette = self.palette.clone();

        self.render_into(&mut image, data);

        image
    }

    fn decode_sixel(&mut self, byte: u8) {
        if !(0x3f..=0x7e).contains(&byte) {
            return;
        }

        let value = byte - 0x3f;

        for bit in 0..6 {
            if value & (1 << bit) != 0 {
                self.max_x = self.max_x.max(self.x);
                self.max_y = self.max_y.max(self.y + bit);
            }
        }

        self.x = self.x.saturating_add(1);
    }

    fn render_into(&self, image: &mut SixelImage, data: &[u8]) {
        let mut x = 0u32;
        let mut y = 0u32;
        let mut color = self.current_color;

        for &byte in data {
            match byte {
                b'?'..=b'~' => {
                    let value = byte - 0x3f;

                    if let Some(pixel_color) = self.palette.get(color) {
                        for bit in 0..6 {
                            if value & (1 << bit) != 0 {
                                image.set_pixel(x, y + bit, pixel_color);
                            }
                        }
                    }

                    x = x.saturating_add(1);
                }

                b'$' => {
                    x = 0;
                }

                b'-' => {
                    x = 0;
                    y = y.saturating_add(6);
                }

                b'#' => {
                    color = self.current_color;
                }

                _ => {}
            }
        }
    }
}
