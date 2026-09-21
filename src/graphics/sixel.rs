use super::image::{
    Image,
    ImageData,
    ImageFormat,
    ImageId,
    ImageMetadata,
};

#[derive(Debug, Clone)]
pub struct SixelImage {
    pub image: Image,
}

impl SixelImage {
    pub fn new(image: Image) -> Self {
        Self { image }
    }

    pub fn into_image(self) -> Image {
        self.image
    }

    pub fn image(&self) -> &Image {
        &self.image
    }
}

#[derive(Debug, Clone, Default)]
pub struct SixelImageDecoder {
    next_id: u64,
}

impl SixelImageDecoder {
    pub fn new() -> Self {
        Self { next_id: 1 }
    }

    pub fn decode(
        &mut self,
        data: &[u8],
        width: u32,
        height: u32,
    ) -> Result<SixelImage, String> {
        if width == 0 || height == 0 {
            return Err("Sixel image dimensions cannot be zero".into());
        }

        if data.is_empty() {
            return Err("Sixel image contains no data".into());
        }

        let id = ImageId::new(self.next_id);
        self.next_id = self.next_id.wrapping_add(1);

        let metadata = ImageMetadata::new(
            id,
            width,
            height,
            ImageFormat::Sixel,
        )
        .with_protocol("sixel");

        let image = Image::new(
            metadata,
            ImageData::Encoded(data.to_vec()),
        );

        Ok(SixelImage::new(image))
    }

    pub fn reset(&mut self) {
        self.next_id = 1;
    }
}
