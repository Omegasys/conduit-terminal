#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CompressionAlgorithm {
    None,
    RunLength,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CompressionError {
    InvalidData,
}

#[derive(Debug, Clone, Copy, Default)]
pub struct Compressor;

impl Compressor {
    pub fn new() -> Self {
        Self
    }

    pub fn compress(
        &self,
        data: &[u8],
        algorithm: CompressionAlgorithm,
    ) -> Result<Vec<u8>, CompressionError> {
        match algorithm {
            CompressionAlgorithm::None => Ok(data.to_vec()),
            CompressionAlgorithm::RunLength => {
                Ok(Self::rle_compress(data))
            }
        }
    }

    pub fn decompress(
        &self,
        data: &[u8],
        algorithm: CompressionAlgorithm,
    ) -> Result<Vec<u8>, CompressionError> {
        match algorithm {
            CompressionAlgorithm::None => Ok(data.to_vec()),
            CompressionAlgorithm::RunLength => {
                Self::rle_decompress(data)
            }
        }
    }

    fn rle_compress(data: &[u8]) -> Vec<u8> {
        let mut output = Vec::new();
        let mut index = 0;

        while index < data.len() {
            let byte = data[index];
            let mut count = 1usize;

            while index + count < data.len()
                && data[index + count] == byte
                && count < 255
            {
                count += 1;
            }

            output.push(count as u8);
            output.push(byte);

            index += count;
        }

        output
    }

    fn rle_decompress(
        data: &[u8],
    ) -> Result<Vec<u8>, CompressionError> {
        if data.len() % 2 != 0 {
            return Err(CompressionError::InvalidData);
        }

        let mut output = Vec::new();

        for pair in data.chunks_exact(2) {
            let count = pair[0];

            if count == 0 {
                return Err(CompressionError::InvalidData);
            }

            for _ in 0..count {
                output.push(pair[1]);
            }
        }

        Ok(output)
    }
}
