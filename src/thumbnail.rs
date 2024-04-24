use crc32fast::Hasher;

use crate::Error;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Thumbnail {
    pub jpeg_data: Vec<u8>,
}

impl Thumbnail {
    pub fn from_bytes(bytes: &[u8]) -> Thumbnail {
        let jpeg_length = bytes[0x4..0x8].try_into().unwrap();
        let jpeg_length = u32::from_be_bytes(jpeg_length) as usize;

        Thumbnail {
            jpeg_data: bytes[0x8..0x8 + jpeg_length].to_vec(),
        }
    }

    pub fn to_bytes(&self) -> Result<Vec<u8>, Error> {
        let mut bytes_without_checksum = Vec::new();
        if self.jpeg_data.len() > 0xC7F8 {
            return Err(Error::FileTooLarge);
        }

        let jpeg_length = (self.jpeg_data.len() as u32).to_be_bytes();

        bytes_without_checksum.extend_from_slice(&jpeg_length);
        bytes_without_checksum.extend_from_slice(&self.jpeg_data);
        bytes_without_checksum.resize(0xC800 - 4, 0);

        let mut hasher = Hasher::new();
        hasher.update(&bytes_without_checksum);
        let checksum = hasher.finalize();

        let mut bytes = Vec::new();
        bytes.extend_from_slice(checksum.to_be_bytes().as_ref());
        bytes.extend_from_slice(&bytes_without_checksum);

        Ok(bytes)
    }
}
