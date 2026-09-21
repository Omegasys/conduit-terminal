use super::image::{
    Image,
    ImageData,
    ImageFormat,
    ImageId,
    ImageMetadata,
};

#[derive(Debug, Clone)]
pub struct Iterm2Image {
    pub image: Image,
}

impl Iterm2Image {
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
pub struct Iterm2ImageDecoder {
    next_id: u64,
}

impl Iterm2ImageDecoder {
    pub fn new() -> Self {
        Self { next_id: 1 }
    }

    pub fn decode(
        &mut self,
        data: &[u8],
        width: u32,
        height: u32,
        inline: bool,
    ) -> Result<Iterm2Image, String> {
        if width == 0 || height == 0 {
            return Err("iTerm2 image dimensions cannot be zero".into());
        }

        if data.is_empty() {
            return Err("iTerm2 image contains no data".into());
        }

        let id = ImageId::new(self.next_id);
        self.next_id = self.next_id.wrapping_add(1);

        let metadata = ImageMetadata::new(
            id,
            width,
            height,
            ImageFormat::Iterm2,
        )
        .with_alpha(true)
        .with_protocol("iterm2");

        let mut image = Image::new(
            metadata,
            ImageData::Encoded(data.to_vec()),
        );

        if inline {
            image
                .metadata
                .source_protocol = Some("iterm2-inline".into());
        }

        Ok(Iterm2Image::new(image))
    }

    pub fn reset(&mut self) {
        self.next_id = 1;
    }
}
