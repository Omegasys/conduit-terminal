use crate::colors::Rgba;

use super::renderer::TextureId;

/// Image pixel format.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ImageFormat {
    Rgba8,
    Rgb8,
    Gray8,
    Indexed8,
}

/// Image scaling filter.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ImageFilter {
    Nearest,
    Bilinear,
    Bicubic,
    Lanczos,
}

impl Default for ImageFilter {
    fn default() -> Self {
        Self::Bilinear
    }
}

/// Image dimensions.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ImageSize {
    pub width: u32,
    pub height: u32,
}

impl ImageSize {
    pub fn new(width: u32, height: u32) -> Self {
        Self { width, height }
    }

    pub fn aspect_ratio(&self) -> f32 {
        if self.height == 0 {
            0.0
        } else {
            self.width as f32 / self.height as f32
        }
    }
}

/// Image resource known to the renderer.
#[derive(Debug, Clone)]
pub struct ImageResource {
    id: TextureId,
    size: ImageSize,
    format: ImageFormat,
    filter: ImageFilter,
    alpha: bool,
    loaded: bool,
}

impl ImageResource {
    pub fn new(
        id: TextureId,
        size: ImageSize,
        format: ImageFormat,
    ) -> Self {
        Self {
            id,
            size,
            format,
            filter: ImageFilter::Bilinear,
            alpha: matches!(format, ImageFormat::Rgba8 | ImageFormat::Indexed8),
            loaded: false,
        }
    }

    pub fn id(&self) -> TextureId {
        self.id
    }

    pub fn size(&self) -> ImageSize {
        self.size
    }

    pub fn format(&self) -> ImageFormat {
        self.format
    }

    pub fn filter(&self) -> ImageFilter {
        self.filter
    }

    pub fn alpha(&self) -> bool {
        self.alpha
    }

    pub fn loaded(&self) -> bool {
        self.loaded
    }

    pub fn set_filter(&mut self, filter: ImageFilter) {
        self.filter = filter;
    }

    pub fn set_alpha(&mut self, alpha: bool) {
        self.alpha = alpha;
    }

    pub fn mark_loaded(&mut self) {
        self.loaded = true;
    }

    pub fn mark_unloaded(&mut self) {
        self.loaded = false;
    }
}

/// Image draw operation.
#[derive(Debug, Clone)]
pub struct ImageDrawCommand {
    pub texture: TextureId,
    pub position: [f32; 2],
    pub size: [f32; 2],
    pub filter: ImageFilter,
    pub tint: Rgba,
    pub opacity: f32,
}

impl ImageDrawCommand {
    pub fn new(
        texture: TextureId,
        position: [f32; 2],
        size: [f32; 2],
    ) -> Self {
        Self {
            texture,
            position,
            size,
            filter: ImageFilter::Bilinear,
            tint: Rgba::WHITE,
            opacity: 1.0,
        }
    }

    pub fn set_filter(&mut self, filter: ImageFilter) {
        self.filter = filter;
    }

    pub fn set_tint(&mut self, tint: Rgba) {
        self.tint = tint;
    }

    pub fn set_opacity(&mut self, opacity: f32) {
        self.opacity = opacity.clamp(0.0, 1.0);
    }
}
