use std::path::{Path, PathBuf};
use thiserror::Error;
use tracing::{debug, warn};

#[derive(Error, Debug)]
pub enum ThumbnailError {
    #[error("Failed to generate thumbnail: {0}")]
    GenerationError(String),
    #[error("Unsupported format: {0}")]
    UnsupportedFormat(String),
    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),
    #[error("Image error: {0}")]
    ImageError(String),
}

#[derive(Debug, Clone, Copy)]
pub struct ThumbnailSize {
    pub width: u32,
    pub height: u32,
}

impl ThumbnailSize {
    pub const SMALL: Self = Self { width: 128, height: 128 };
    pub const MEDIUM: Self = Self { width: 256, height: 256 };
    pub const LARGE: Self = Self { width: 512, height: 512 };
}

pub struct ThumbnailGenerator {
    cache_dir: PathBuf,
}

impl ThumbnailGenerator {
    pub fn new(cache_dir: PathBuf) -> Self {
        std::fs::create_dir_all(&cache_dir).ok();
        Self { cache_dir }
    }

    pub fn generate(
        &self,
        source: &Path,
        size: ThumbnailSize,
    ) -> Result<PathBuf, ThumbnailError> {
        let cache_key = self.cache_key(source, size);
        let cache_path = self.cache_dir.join(&cache_key);

        if cache_path.exists() {
            debug!("Thumbnail cache hit: {:?}", cache_path);
            return Ok(cache_path);
        }

        let ext = source
            .extension()
            .and_then(|e| e.to_str())
            .unwrap_or("")
            .to_lowercase();

        match ext.as_str() {
            // Images
            "jpg" | "jpeg" | "png" | "gif" | "webp" | "bmp" | "tiff" | "tif" => {
                self.generate_image_thumbnail(source, &cache_path, size)
            }
            // RAW photos
            "cr2" | "cr3" | "nef" | "arw" | "dng" | "orf" | "rw2" | "raf" => {
                self.generate_raw_thumbnail(source, &cache_path, size)
            }
            // Video
            "mp4" | "mkv" | "mov" | "avi" | "webm" => {
                self.generate_video_thumbnail(source, &cache_path, size)
            }
            // Audio (waveform)
            "mp3" | "flac" | "wav" | "ogg" | "m4a" => {
                self.generate_audio_thumbnail(source, &cache_path, size)
            }
            // 3D models - placeholder
            "gltf" | "glb" | "obj" | "fbx" | "stl" => {
                self.generate_3d_thumbnail(source, &cache_path, size)
            }
            // Documents
            "pdf" => self.generate_pdf_thumbnail(source, &cache_path, size),
            _ => Err(ThumbnailError::UnsupportedFormat(ext)),
        }?;

        Ok(cache_path)
    }

    #[cfg(feature = "image-processing")]
    fn generate_image_thumbnail(
        &self,
        source: &Path,
        dest: &Path,
        size: ThumbnailSize,
    ) -> Result<(), ThumbnailError> {
        let img = image::open(source)
            .map_err(|e| ThumbnailError::ImageError(e.to_string()))?;

        let thumbnail = img.thumbnail(size.width, size.height);

        thumbnail
            .save(dest)
            .map_err(|e| ThumbnailError::ImageError(e.to_string()))?;

        debug!("Generated image thumbnail: {:?}", dest);
        Ok(())
    }

    #[cfg(not(feature = "image-processing"))]
    fn generate_image_thumbnail(
        &self,
        _source: &Path,
        _dest: &Path,
        _size: ThumbnailSize,
    ) -> Result<(), ThumbnailError> {
        Err(ThumbnailError::GenerationError(
            "Image processing not enabled".into(),
        ))
    }

    fn generate_raw_thumbnail(
        &self,
        source: &Path,
        dest: &Path,
        size: ThumbnailSize,
    ) -> Result<(), ThumbnailError> {
        use rawloader::RawLoader;

        let loader = RawLoader::new();
        let raw = loader
            .decode_file(source)
            .map_err(|e| ThumbnailError::GenerationError(e.to_string()))?;

        // Extract embedded thumbnail if available
        if let Some(thumb) = raw.preview() {
            #[cfg(feature = "image-processing")]
            {
                let img = image::load_from_memory(thumb)
                    .map_err(|e| ThumbnailError::ImageError(e.to_string()))?;
                let thumbnail = img.thumbnail(size.width, size.height);
                thumbnail
                    .save(dest)
                    .map_err(|e| ThumbnailError::ImageError(e.to_string()))?;
                return Ok(());
            }
        }

        warn!("No embedded thumbnail in RAW file: {:?}", source);
        Err(ThumbnailError::GenerationError(
            "No embedded thumbnail".into(),
        ))
    }

    #[cfg(feature = "ffmpeg")]
    fn generate_video_thumbnail(
        &self,
        source: &Path,
        dest: &Path,
        size: ThumbnailSize,
    ) -> Result<(), ThumbnailError> {
        use ffmpeg_next::format::{input, Pixel};
        use ffmpeg_next::media::Type;
        use ffmpeg_next::software::scaling::{context::Context as ScalingContext, flag::Flags};
        use ffmpeg_next::util::frame::video::Video;

        let mut ictx = input(&source)
            .map_err(|e| ThumbnailError::GenerationError(e.to_string()))?;

        let input = ictx
            .streams()
            .best(Type::Video)
            .ok_or_else(|| ThumbnailError::GenerationError("No video stream".into()))?;

        let video_stream_index = input.index();

        let context_decoder =
            ffmpeg_next::codec::context::Context::from_parameters(input.parameters())
                .map_err(|e| ThumbnailError::GenerationError(e.to_string()))?;

        let mut decoder = context_decoder
            .decoder()
            .video()
            .map_err(|e| ThumbnailError::GenerationError(e.to_string()))?;

        // Seek to 10% of duration for thumbnail
        let duration = ictx.duration();
        if duration > 0 {
            let seek_pos = duration / 10;
            ictx.seek(seek_pos, ..)
                .map_err(|e| ThumbnailError::GenerationError(e.to_string()))?;
        }

        let mut scaler = ScalingContext::get(
            decoder.format(),
            decoder.width(),
            decoder.height(),
            Pixel::RGB24,
            size.width,
            size.height,
            Flags::BILINEAR,
        )
        .map_err(|e| ThumbnailError::GenerationError(e.to_string()))?;

        let mut frame_count = 0;
        for (stream, packet) in ictx.packets() {
            if stream.index() == video_stream_index {
                decoder
                    .send_packet(&packet)
                    .map_err(|e| ThumbnailError::GenerationError(e.to_string()))?;

                let mut decoded = Video::empty();
                while decoder.receive_frame(&mut decoded).is_ok() {
                    frame_count += 1;
                    if frame_count >= 5 {
                        // Skip first few frames
                        let mut rgb_frame = Video::empty();
                        scaler
                            .run(&decoded, &mut rgb_frame)
                            .map_err(|e| ThumbnailError::GenerationError(e.to_string()))?;

                        #[cfg(feature = "image-processing")]
                        {
                            let img = image::RgbImage::from_raw(
                                rgb_frame.width(),
                                rgb_frame.height(),
                                rgb_frame.data(0).to_vec(),
                            )
                            .ok_or_else(|| {
                                ThumbnailError::GenerationError("Failed to create image".into())
                            })?;

                            img.save(dest)
                                .map_err(|e| ThumbnailError::ImageError(e.to_string()))?;
                        }

                        debug!("Generated video thumbnail: {:?}", dest);
                        return Ok(());
                    }
                }
            }
        }

        Err(ThumbnailError::GenerationError(
            "Failed to extract frame".into(),
        ))
    }

    #[cfg(not(feature = "ffmpeg"))]
    fn generate_video_thumbnail(
        &self,
        _source: &Path,
        _dest: &Path,
        _size: ThumbnailSize,
    ) -> Result<(), ThumbnailError> {
        Err(ThumbnailError::GenerationError(
            "FFmpeg not available".into(),
        ))
    }

    fn generate_audio_thumbnail(
        &self,
        _source: &Path,
        _dest: &Path,
        _size: ThumbnailSize,
    ) -> Result<(), ThumbnailError> {
        // TODO: Generate waveform visualization
        Err(ThumbnailError::GenerationError(
            "Audio waveform not implemented".into(),
        ))
    }

    fn generate_3d_thumbnail(
        &self,
        _source: &Path,
        _dest: &Path,
        _size: ThumbnailSize,
    ) -> Result<(), ThumbnailError> {
        // TODO: Render 3D model preview
        Err(ThumbnailError::GenerationError(
            "3D preview not implemented".into(),
        ))
    }

    fn generate_pdf_thumbnail(
        &self,
        _source: &Path,
        _dest: &Path,
        _size: ThumbnailSize,
    ) -> Result<(), ThumbnailError> {
        // TODO: Render PDF first page
        Err(ThumbnailError::GenerationError(
            "PDF preview not implemented".into(),
        ))
    }

    fn cache_key(&self, source: &Path, size: ThumbnailSize) -> String {
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};

        let mut hasher = DefaultHasher::new();
        source.hash(&mut hasher);

        // Include modification time in hash
        if let Ok(metadata) = source.metadata() {
            if let Ok(modified) = metadata.modified() {
                modified.hash(&mut hasher);
            }
        }

        format!("{:x}_{}x{}.png", hasher.finish(), size.width, size.height)
    }

    pub fn clear_cache(&self) -> Result<(), ThumbnailError> {
        if self.cache_dir.exists() {
            std::fs::remove_dir_all(&self.cache_dir)?;
            std::fs::create_dir_all(&self.cache_dir)?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[test]
    fn test_thumbnail_cache_key() {
        let dir = tempdir().unwrap();
        let gen = ThumbnailGenerator::new(dir.path().to_path_buf());

        let key1 = gen.cache_key(Path::new("/test/file.jpg"), ThumbnailSize::SMALL);
        let key2 = gen.cache_key(Path::new("/test/file.jpg"), ThumbnailSize::MEDIUM);

        assert_ne!(key1, key2);
    }
}
