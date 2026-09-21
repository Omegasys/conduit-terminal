/// Image/content scaling strategy.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ScalingMode {
    None,
    Stretch,
    Fit,
    Fill,
    Crop,
    Integer,
    PixelPerfect,
}

impl Default for ScalingMode {
    fn default() -> Self {
        Self::Fit
    }
}

/// Scaling filter.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ScalingFilter {
    Nearest,
    Linear,
    Cubic,
    Lanczos,
}

impl Default for ScalingFilter {
    fn default() -> Self {
        Self::Linear
    }
}

/// Scaling calculation result.
#[derive(Debug, Clone, Copy)]
pub struct ScaleResult {
    pub source_width: f32,
    pub source_height: f32,
    pub destination_width: f32,
    pub destination_height: f32,
    pub offset_x: f32,
    pub offset_y: f32,
}

impl ScaleResult {
    pub fn new(
        source_width: f32,
        source_height: f32,
        destination_width: f32,
        destination_height: f32,
    ) -> Self {
        Self {
            source_width,
            source_height,
            destination_width,
            destination_height,
            offset_x: 0.0,
            offset_y: 0.0,
        }
    }
}

/// Renderer scaling configuration.
#[derive(Debug, Clone)]
pub struct ScalingConfig {
    mode: ScalingMode,
    filter: ScalingFilter,
    preserve_aspect_ratio: bool,
    allow_fractional_scaling: bool,
    integer_scale_only: bool,
}

impl Default for ScalingConfig {
    fn default() -> Self {
        Self {
            mode: ScalingMode::Fit,
            filter: ScalingFilter::Linear,
            preserve_aspect_ratio: true,
            allow_fractional_scaling: true,
            integer_scale_only: false,
        }
    }
}

impl ScalingConfig {
    pub fn mode(&self) -> ScalingMode {
        self.mode
    }

    pub fn filter(&self) -> ScalingFilter {
        self.filter
    }

    pub fn preserve_aspect_ratio(&self) -> bool {
        self.preserve_aspect_ratio
    }

    pub fn allow_fractional_scaling(&self) -> bool {
        self.allow_fractional_scaling
    }

    pub fn integer_scale_only(&self) -> bool {
        self.integer_scale_only
    }

    pub fn set_mode(&mut self, mode: ScalingMode) {
        self.mode = mode;
    }

    pub fn set_filter(&mut self, filter: ScalingFilter) {
        self.filter = filter;
    }

    pub fn set_preserve_aspect_ratio(&mut self, enabled: bool) {
        self.preserve_aspect_ratio = enabled;
    }

    pub fn set_allow_fractional_scaling(&mut self, enabled: bool) {
        self.allow_fractional_scaling = enabled;
    }

    pub fn set_integer_scale_only(&mut self, enabled: bool) {
        self.integer_scale_only = enabled;
    }

    pub fn calculate(
        &self,
        source_width: f32,
        source_height: f32,
        destination_width: f32,
        destination_height: f32,
    ) -> ScaleResult {
        if source_width <= 0.0
            || source_height <= 0.0
            || destination_width <= 0.0
            || destination_height <= 0.0
        {
            return ScaleResult::new(
                source_width,
                source_height,
                0.0,
                0.0,
            );
        }

        match self.mode {
            ScalingMode::None => ScaleResult::new(
                source_width,
                source_height,
                source_width,
                source_height,
            ),

            ScalingMode::Stretch => ScaleResult::new(
                source_width,
                source_height,
                destination_width,
                destination_height,
            ),

            ScalingMode::Integer | ScalingMode::PixelPerfect => {
                let scale_x = destination_width / source_width;
                let scale_y = destination_height / source_height;
                let scale = scale_x.min(scale_y).floor().max(1.0);

                let width = source_width * scale;
                let height = source_height * scale;

                let mut result = ScaleResult::new(
                    source_width,
                    source_height,
                    width,
                    height,
                );

                result.offset_x = (destination_width - width) / 2.0;
                result.offset_y = (destination_height - height) / 2.0;
                result
            }

            ScalingMode::Fit => {
                if !self.preserve_aspect_ratio {
                    return ScaleResult::new(
                        source_width,
                        source_height,
                        destination_width,
                        destination_height,
                    );
                }

                let scale_x = destination_width / source_width;
                let scale_y = destination_height / source_height;
                let scale = scale_x.min(scale_y);

                let width = source_width * scale;
                let height = source_height * scale;

                let mut result = ScaleResult::new(
                    source_width,
                    source_height,
                    width,
                    height,
                );

                result.offset_x = (destination_width - width) / 2.0;
                result.offset_y = (destination_height - height) / 2.0;
                result
            }

            ScalingMode::Fill | ScalingMode::Crop => {
                if !self.preserve_aspect_ratio {
                    return ScaleResult::new(
                        source_width,
                        source_height,
                        destination_width,
                        destination_height,
                    );
                }

                let scale_x = destination_width / source_width;
                let scale_y = destination_height / source_height;
                let scale = scale_x.max(scale_y);

                let width = source_width * scale;
                let height = source_height * scale;

                let mut result = ScaleResult::new(
                    source_width,
                    source_height,
                    width,
                    height,
                );

                result.offset_x = (destination_width - width) / 2.0;
                result.offset_y = (destination_height - height) / 2.0;
                result
            }
        }
    }
}
