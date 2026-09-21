use super::image::{
    Image,
    ImageData,
    ImageFormat,
    ImageId,
    ImageMetadata,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum KittyImageFormat {
    Rgb,
    Rgba,
    Png,
    Unknown,
}

#[derive(Debug, Clone)]
pub struct KittyImage {
    pub image: Image,
}

impl KittyImage {
    pub fn new(image: Image) -> Self {
        Self { image }
    }

    pub fn image(&self) -> &Image {
        &self.image
    }

    pub fn into_image(self) -> Image {
        self.image
    }
}

#[derive(Debug, Clone, Default)]
pub struct KittyImageDecoder {
    next_id: u64,
}

impl KittyImageDecoder {
    pub fn new() -> Self {
        Self { next_id: 1 }
    }

    pub fn decode(
        &mut self,
        data: &[u8],
        width: u32,
        height: u32,
        format: KittyImageFormat,
    ) -> Result<KittyImage, String> {
        if width == 0 || height == 0 {
            return Err("Kitty image dimensions cannot be zero".into());
        }

        if data.is_empty() {
            return Err("Kitty image contains no data".into());
        }

        let image_format = match format {
            KittyImageFormat::Rgb => ImageFormat::Rgb8,
            KittyImageFormat::Rgba => ImageFormat::Rgba8,
            KittyImageFormat::Png => ImageFormat::Kitty,
            KittyImageFormat::Unknown => ImageFormat::Unknown,
        };

        let has_alpha = matches!(format, KittyImageFormat::Rgba);

        let id = ImageId::new(self.next_id);
        self.next_id = self.next_id.wrapping_add(1);

        let metadata = ImageMetadata::new(
            id,
            width,
            height,
            image_format,
        )
        .with_alpha(has_alpha)
        .with_protocol("kitty");

        let image = Image::new(
            metadata,
            ImageData::Encoded(data.to_vec()),
        );

        Ok(KittyImage::new(image))
    }

    pub fn reset(&mut self) {
        self.next_id = 1;
    }
}
