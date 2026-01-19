use std::path::Path;
use thiserror::Error;
use tracing::debug;

#[derive(Error, Debug)]
pub enum ExrError {
    #[error("Failed to open EXR file: {0}")]
    OpenError(String),
    #[error("Failed to read EXR data: {0}")]
    ReadError(String),
    #[error("Failed to write EXR file: {0}")]
    WriteError(String),
    #[error("Unsupported EXR feature: {0}")]
    UnsupportedFeature(String),
    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),
}

#[derive(Debug, Clone)]
pub struct ExrMetadata {
    pub width: u32,
    pub height: u32,
    pub channels: Vec<ChannelInfo>,
    pub compression: Compression,
    pub data_window: (i32, i32, i32, i32),
    pub display_window: (i32, i32, i32, i32),
    pub pixel_aspect_ratio: f32,
    pub attributes: Vec<(String, String)>,
}

#[derive(Debug, Clone)]
pub struct ChannelInfo {
    pub name: String,
    pub pixel_type: PixelType,
    pub x_sampling: u32,
    pub y_sampling: u32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PixelType {
    Uint,
    Half,
    Float,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Compression {
    None,
    Rle,
    ZipsS,
    Zip,
    Piz,
    Pxr24,
    B44,
    B44a,
    Dwaa,
    Dwab,
}

pub struct ExrImage {
    pub metadata: ExrMetadata,
    pub pixels: Vec<f32>,
}

impl ExrImage {
    #[cfg(feature = "openexr")]
    pub fn open<P: AsRef<Path>>(path: P) -> Result<Self, ExrError> {
        use openexr::prelude::*;

        let path = path.as_ref();
        debug!("Opening EXR file: {:?}", path);

        let reader = read()
            .no_deep_data()
            .largest_resolution_level()
            .rgba_channels(
                PixelVec::<(f32, f32, f32, f32)>::constructor,
                PixelVec::set_pixel,
            )
            .first_valid_layer()
            .all_attributes()
            .from_file(path)
            .map_err(|e| ExrError::OpenError(e.to_string()))?;

        let layer = reader.layer_data;
        let size = reader.attributes.layer_size;

        let metadata = ExrMetadata {
            width: size.width() as u32,
            height: size.height() as u32,
            channels: vec![
                ChannelInfo {
                    name: "R".to_string(),
                    pixel_type: PixelType::Float,
                    x_sampling: 1,
                    y_sampling: 1,
                },
                ChannelInfo {
                    name: "G".to_string(),
                    pixel_type: PixelType::Float,
                    x_sampling: 1,
                    y_sampling: 1,
                },
                ChannelInfo {
                    name: "B".to_string(),
                    pixel_type: PixelType::Float,
                    x_sampling: 1,
                    y_sampling: 1,
                },
                ChannelInfo {
                    name: "A".to_string(),
                    pixel_type: PixelType::Float,
                    x_sampling: 1,
                    y_sampling: 1,
                },
            ],
            compression: Compression::Zip,
            data_window: (0, 0, size.width() as i32, size.height() as i32),
            display_window: (0, 0, size.width() as i32, size.height() as i32),
            pixel_aspect_ratio: 1.0,
            attributes: Vec::new(),
        };

        let mut pixels = Vec::with_capacity(size.width() * size.height() * 4);
        for (r, g, b, a) in layer.channel_data.pixels.iter() {
            pixels.push(*r);
            pixels.push(*g);
            pixels.push(*b);
            pixels.push(*a);
        }

        Ok(Self { metadata, pixels })
    }

    #[cfg(not(feature = "openexr"))]
    pub fn open<P: AsRef<Path>>(_path: P) -> Result<Self, ExrError> {
        Err(ExrError::UnsupportedFeature("OpenEXR not enabled".into()))
    }

    #[cfg(feature = "openexr")]
    pub fn save<P: AsRef<Path>>(&self, path: P) -> Result<(), ExrError> {
        use openexr::prelude::*;

        let path = path.as_ref();
        debug!("Saving EXR file: {:?}", path);

        let size = (self.metadata.width as usize, self.metadata.height as usize);

        let mut rgba_data: Vec<(f32, f32, f32, f32)> = Vec::with_capacity(size.0 * size.1);
        for chunk in self.pixels.chunks(4) {
            if chunk.len() == 4 {
                rgba_data.push((chunk[0], chunk[1], chunk[2], chunk[3]));
            }
        }

        let layer = Layer::new(
            size,
            LayerAttributes::named("main"),
            Encoding::SMALL_LOSSLESS,
            SpecificChannels::rgba(|pos: Vec2<usize>| rgba_data[pos.y() * size.0 + pos.x()]),
        );

        layer
            .write()
            .to_file(path)
            .map_err(|e| ExrError::WriteError(e.to_string()))?;

        Ok(())
    }

    #[cfg(not(feature = "openexr"))]
    pub fn save<P: AsRef<Path>>(&self, _path: P) -> Result<(), ExrError> {
        Err(ExrError::UnsupportedFeature("OpenEXR not enabled".into()))
    }

    pub fn width(&self) -> u32 {
        self.metadata.width
    }

    pub fn height(&self) -> u32 {
        self.metadata.height
    }

    pub fn get_pixel(&self, x: u32, y: u32) -> Option<[f32; 4]> {
        if x >= self.metadata.width || y >= self.metadata.height {
            return None;
        }
        let idx = ((y * self.metadata.width + x) * 4) as usize;
        Some([
            self.pixels[idx],
            self.pixels[idx + 1],
            self.pixels[idx + 2],
            self.pixels[idx + 3],
        ])
    }

    pub fn set_pixel(&mut self, x: u32, y: u32, rgba: [f32; 4]) {
        if x < self.metadata.width && y < self.metadata.height {
            let idx = ((y * self.metadata.width + x) * 4) as usize;
            self.pixels[idx] = rgba[0];
            self.pixels[idx + 1] = rgba[1];
            self.pixels[idx + 2] = rgba[2];
            self.pixels[idx + 3] = rgba[3];
        }
    }

    pub fn new(width: u32, height: u32) -> Self {
        let metadata = ExrMetadata {
            width,
            height,
            channels: vec![
                ChannelInfo {
                    name: "R".to_string(),
                    pixel_type: PixelType::Float,
                    x_sampling: 1,
                    y_sampling: 1,
                },
                ChannelInfo {
                    name: "G".to_string(),
                    pixel_type: PixelType::Float,
                    x_sampling: 1,
                    y_sampling: 1,
                },
                ChannelInfo {
                    name: "B".to_string(),
                    pixel_type: PixelType::Float,
                    x_sampling: 1,
                    y_sampling: 1,
                },
                ChannelInfo {
                    name: "A".to_string(),
                    pixel_type: PixelType::Float,
                    x_sampling: 1,
                    y_sampling: 1,
                },
            ],
            compression: Compression::Zip,
            data_window: (0, 0, width as i32, height as i32),
            display_window: (0, 0, width as i32, height as i32),
            pixel_aspect_ratio: 1.0,
            attributes: Vec::new(),
        };

        Self {
            metadata,
            pixels: vec![0.0; (width * height * 4) as usize],
        }
    }

    pub fn apply_exposure(&mut self, exposure: f32) {
        let factor = 2.0_f32.powf(exposure);
        for chunk in self.pixels.chunks_mut(4) {
            chunk[0] *= factor;
            chunk[1] *= factor;
            chunk[2] *= factor;
        }
    }

    pub fn tonemap_reinhard(&self) -> Vec<u8> {
        let mut result = Vec::with_capacity((self.metadata.width * self.metadata.height * 3) as usize);

        for chunk in self.pixels.chunks(4) {
            let r = chunk[0] / (1.0 + chunk[0]);
            let g = chunk[1] / (1.0 + chunk[1]);
            let b = chunk[2] / (1.0 + chunk[2]);

            result.push((r.clamp(0.0, 1.0) * 255.0) as u8);
            result.push((g.clamp(0.0, 1.0) * 255.0) as u8);
            result.push((b.clamp(0.0, 1.0) * 255.0) as u8);
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_exr_image() {
        let img = ExrImage::new(100, 100);
        assert_eq!(img.width(), 100);
        assert_eq!(img.height(), 100);
    }

    #[test]
    fn test_pixel_access() {
        let mut img = ExrImage::new(10, 10);
        img.set_pixel(5, 5, [1.0, 0.5, 0.25, 1.0]);

        let pixel = img.get_pixel(5, 5).unwrap();
        assert_eq!(pixel[0], 1.0);
        assert_eq!(pixel[1], 0.5);
        assert_eq!(pixel[2], 0.25);
        assert_eq!(pixel[3], 1.0);
    }

    #[test]
    fn test_tonemap() {
        let mut img = ExrImage::new(2, 2);
        img.set_pixel(0, 0, [1.0, 1.0, 1.0, 1.0]);
        img.set_pixel(1, 0, [0.0, 0.0, 0.0, 1.0]);

        let ldr = img.tonemap_reinhard();
        assert_eq!(ldr.len(), 12); // 2x2 * 3 channels
    }
}
