use super::Result;
use std::path::Path;

pub struct PngData {
    data: Vec<u8>,
}

impl PngData {
    pub fn from_raw(raw: Vec<u8>) -> Self {
        PngData { data: raw }
    }

    pub fn from_path(path: impl AsRef<Path>) -> Result<Self> {
        let data = std::fs::read(path).map_err(|_| "Could not read file.")?;
        Ok(Self::from_raw(data))
    }

    pub fn len(&self) -> usize {
        self.data.len()
    }

    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    pub const fn data(&self) -> &Vec<u8> {
        &self.data
    }

    pub fn data_mut(&mut self) -> &mut Vec<u8> {
        &mut self.data
    }
}
