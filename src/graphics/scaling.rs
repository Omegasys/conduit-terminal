#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ImageScaleMode {
    Original,
    Fit,
    Fill,
    Stretch,
    Integer,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ScaleDimensions {
    pub width: u32,
    pub height: u32,
}

impl ScaleDimensions {
    pub const fn new(width: u32, height: u32) -> Self {
        Self { width, height }
    }
}

#[derive(Debug, Clone, Default)]
pub struct ImageScaler;

impl ImageScaler {
    pub fn new() -> Self {
        Self
    }

    pub fn calculate_dimensions(
        width: u32,
        height: u32,
        maximum_width: Option<u32>,
        maximum_height: Option<u32>,
        mode: ImageScaleMode,
        preserve_aspect_ratio: bool,
    ) -> ScaleDimensions {
        if width == 0 || height == 0 {
            return ScaleDimensions::new(0, 0);
        }

        if matches!(mode, ImageScaleMode::Original) {
            return ScaleDimensions::new(width, height);
        }

        let max_width = maximum_width.unwrap_or(width).max(1);
        let max_height = maximum_height.unwrap_or(height).max(1);

        if matches!(mode, ImageScaleMode::Stretch) || !preserve_aspect_ratio {
            return ScaleDimensions::new(max_width, max_height);
        }

        let width_ratio = max_width as f64 / width as f64;
        let height_ratio = max_height as f64 / height as f64;

        let scale = match mode {
            ImageScaleMode::Fit => width_ratio.min(height_ratio),
            ImageScaleMode::Fill => width_ratio.max(height_ratio),
            ImageScaleMode::Integer => {
                let integer_scale = width_ratio
                    .min(height_ratio)
                    .floor()
                    .max(1.0);

                integer_scale
            }
            ImageScaleMode::Original | ImageScaleMode::Stretch => 1.0,
        };

        let scaled_width = ((width as f64) * scale).round() as u32;
        let scaled_height = ((height as f64) * scale).round() as u32;

        ScaleDimensions::new(
            scaled_width.max(1),
            scaled_height.max(1),
        )
    }

    pub fn scale(
        image: &super::image::Image,
        maximum_width: Option<u32>,
        maximum_height: Option<u32>,
        mode: ImageScaleMode,
    ) -> ScaleDimensions {
        Self::calculate_dimensions(
            image.width(),
            image.height(),
            maximum_width,
            maximum_height,
            mode,
            true,
        )
    }
}
