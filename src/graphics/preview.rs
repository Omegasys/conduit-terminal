use super::image::Image;
use super::scaling::{ImageScaleMode, ImageScaler};

#[derive(Debug, Clone)]
pub struct PreviewOptions {
    pub maximum_width: Option<u32>,
    pub maximum_height: Option<u32>,
    pub scale_mode: ImageScaleMode,
    pub preserve_aspect_ratio: bool,
}

impl Default for PreviewOptions {
    fn default() -> Self {
        Self {
            maximum_width: None,
            maximum_height: None,
            scale_mode: ImageScaleMode::Fit,
            preserve_aspect_ratio: true,
        }
    }
}

#[derive(Debug, Clone)]
pub struct ImagePreview {
    image: Image,
    width: u32,
    height: u32,
}

impl ImagePreview {
    pub fn create(
        image: Image,
        options: &PreviewOptions,
    ) -> Self {
        let dimensions = ImageScaler::calculate_dimensions(
            image.width(),
            image.height(),
            options.maximum_width,
            options.maximum_height,
            options.scale_mode,
            options.preserve_aspect_ratio,
        );

        Self {
            image,
            width: dimensions.width,
            height: dimensions.height,
        }
    }

    pub fn image(&self) -> &Image {
        &self.image
    }

    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height
    }

    pub fn dimensions(&self) -> (u32, u32) {
        (self.width, self.height)
    }
}
