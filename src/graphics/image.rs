use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ImageId(pub u64);

impl ImageId {
    pub const fn new(id: u64) -> Self {
        Self(id)
    }

    pub const fn get(self) -> u64 {
        self.0
    }
}

impl fmt::Display for ImageId {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{}", self.0)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ImageFormat {
    Rgba8,
    Rgb8,
    Gray8,
    Indexed8,
    Sixel,
    Kitty,
    Iterm2,
    Unknown,
}

#[derive(Debug, Clone)]
pub struct ImageMetadata {
    pub id: ImageId,
    pub width: u32,
    pub height: u32,
    pub format: ImageFormat,
    pub has_alpha: bool,
    pub source_protocol: Option<String>,
}

impl ImageMetadata {
    pub fn new(
        id: ImageId,
        width: u32,
        height: u32,
        format: ImageFormat,
    ) -> Self {
        Self {
            id,
            width,
            height,
            format,
            has_alpha: false,
            source_protocol: None,
        }
    }

    pub fn with_alpha(mut self, has_alpha: bool) -> Self {
        self.has_alpha = has_alpha;
        self
    }

    pub fn with_protocol(mut self, protocol: impl Into<String>) -> Self {
        self.source_protocol = Some(protocol.into());
        self
    }
}

#[derive(Debug, Clone)]
pub enum ImageData {
    Rgba8(Vec<u8>),
    Rgb8(Vec<u8>),
    Gray8(Vec<u8>),
    Indexed8 {
        pixels: Vec<u8>,
        palette: Vec<[u8; 4]>,
    },
    Encoded(Vec<u8>),
}

impl ImageData {
    pub fn len(&self) -> usize {
        match self {
            Self::Rgba8(data)
            | Self::Rgb8(data)
            | Self::Gray8(data)
            | Self::Encoded(data) => data.len(),

            Self::Indexed8 { pixels, .. } => pixels.len(),
        }
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

#[derive(Debug, Clone)]
pub struct Image {
    metadata: ImageMetadata,
    data: ImageData,
}

impl Image {
    pub fn new(metadata: ImageMetadata, data: ImageData) -> Self {
        Self { metadata, data }
    }

    pub fn metadata(&self) -> &ImageMetadata {
        &self.metadata
    }

    pub fn data(&self) -> &ImageData {
        &self.data
    }

    pub fn data_mut(&mut self) -> &mut ImageData {
        &mut self.data
    }

    pub fn id(&self) -> ImageId {
        self.metadata.id
    }

    pub fn width(&self) -> u32 {
        self.metadata.width
    }

    pub fn height(&self) -> u32 {
        self.metadata.height
    }

    pub fn format(&self) -> ImageFormat {
        self.metadata.format
    }

    pub fn byte_len(&self) -> usize {
        self.data.len()
    }
}
