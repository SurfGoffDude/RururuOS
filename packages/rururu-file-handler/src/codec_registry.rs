use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct CodecInfo {
    pub name: String,
    pub category: CodecCategory,
    pub library: String,
    pub supported: bool,
}

#[derive(Debug, Clone, PartialEq)]
pub enum CodecCategory {
    VideoEncoder,
    VideoDecoder,
    AudioEncoder,
    AudioDecoder,
    ImageEncoder,
    ImageDecoder,
    Container,
}

pub struct CodecRegistry {
    codecs: HashMap<String, CodecInfo>,
}

impl CodecRegistry {
    pub fn new() -> Self {
        let mut registry = Self {
            codecs: HashMap::new(),
        };
        registry.register_default_codecs();
        registry
    }

    pub fn handler_count(&self) -> usize {
        self.codecs.len()
    }

    pub fn get(&self, name: &str) -> Option<&CodecInfo> {
        self.codecs.get(name)
    }

    pub fn is_supported(&self, name: &str) -> bool {
        self.codecs.get(name).map(|c| c.supported).unwrap_or(false)
    }

    pub fn list_by_category(&self, category: CodecCategory) -> Vec<&CodecInfo> {
        self.codecs
            .values()
            .filter(|c| c.category == category)
            .collect()
    }

    fn register_default_codecs(&mut self) {
        // Video decoders (FFmpeg)
        self.register_ffmpeg_video_decoders();
        
        // Video encoders (FFmpeg)
        self.register_ffmpeg_video_encoders();
        
        // Audio decoders (FFmpeg)
        self.register_ffmpeg_audio_decoders();
        
        // Audio encoders (FFmpeg)
        self.register_ffmpeg_audio_encoders();
        
        // Image codecs
        self.register_image_codecs();
    }

    fn register_ffmpeg_video_decoders(&mut self) {
        let decoders = [
            ("h264", "H.264 / AVC"),
            ("hevc", "H.265 / HEVC"),
            ("vp8", "VP8"),
            ("vp9", "VP9"),
            ("av1", "AV1"),
            ("mpeg2video", "MPEG-2"),
            ("mpeg4", "MPEG-4"),
            ("prores", "Apple ProRes"),
            ("dnxhd", "Avid DNxHD/DNxHR"),
            ("mjpeg", "Motion JPEG"),
            ("theora", "Theora"),
            ("vvc", "H.266 / VVC"),
        ];

        for (id, name) in decoders {
            self.codecs.insert(
                format!("dec_{}", id),
                CodecInfo {
                    name: name.to_string(),
                    category: CodecCategory::VideoDecoder,
                    library: "ffmpeg".to_string(),
                    supported: true,
                },
            );
        }
    }

    fn register_ffmpeg_video_encoders(&mut self) {
        let encoders = [
            ("libx264", "H.264 (x264)"),
            ("libx265", "H.265 (x265)"),
            ("libvpx-vp9", "VP9"),
            ("libaom-av1", "AV1 (libaom)"),
            ("libsvtav1", "AV1 (SVT-AV1)"),
            ("prores_ks", "Apple ProRes"),
            ("dnxhd", "Avid DNxHD"),
        ];

        for (id, name) in encoders {
            self.codecs.insert(
                format!("enc_{}", id),
                CodecInfo {
                    name: name.to_string(),
                    category: CodecCategory::VideoEncoder,
                    library: "ffmpeg".to_string(),
                    supported: true,
                },
            );
        }
    }

    fn register_ffmpeg_audio_decoders(&mut self) {
        let decoders = [
            ("mp3", "MP3"),
            ("aac", "AAC"),
            ("flac", "FLAC"),
            ("vorbis", "Vorbis"),
            ("opus", "Opus"),
            ("pcm_s16le", "PCM 16-bit"),
            ("pcm_s24le", "PCM 24-bit"),
            ("pcm_f32le", "PCM 32-bit float"),
            ("alac", "Apple Lossless"),
            ("dts", "DTS"),
            ("ac3", "Dolby AC-3"),
            ("eac3", "Dolby E-AC-3"),
            ("truehd", "Dolby TrueHD"),
        ];

        for (id, name) in decoders {
            self.codecs.insert(
                format!("dec_{}", id),
                CodecInfo {
                    name: name.to_string(),
                    category: CodecCategory::AudioDecoder,
                    library: "ffmpeg".to_string(),
                    supported: true,
                },
            );
        }
    }

    fn register_ffmpeg_audio_encoders(&mut self) {
        let encoders = [
            ("libmp3lame", "MP3 (LAME)"),
            ("libfdk_aac", "AAC (FDK)"),
            ("aac", "AAC (native)"),
            ("flac", "FLAC"),
            ("libvorbis", "Vorbis"),
            ("libopus", "Opus"),
            ("pcm_s16le", "PCM 16-bit"),
            ("pcm_s24le", "PCM 24-bit"),
            ("alac", "Apple Lossless"),
        ];

        for (id, name) in encoders {
            self.codecs.insert(
                format!("enc_{}", id),
                CodecInfo {
                    name: name.to_string(),
                    category: CodecCategory::AudioEncoder,
                    library: "ffmpeg".to_string(),
                    supported: true,
                },
            );
        }
    }

    fn register_image_codecs(&mut self) {
        let image_codecs = [
            ("jpeg", "JPEG", "libjpeg-turbo"),
            ("png", "PNG", "libpng"),
            ("webp", "WebP", "libwebp"),
            ("avif", "AVIF", "libavif"),
            ("heic", "HEIC", "libheif"),
            ("tiff", "TIFF", "libtiff"),
            ("exr", "OpenEXR", "openexr"),
            ("jxl", "JPEG XL", "libjxl"),
            ("raw", "Camera RAW", "libraw"),
        ];

        for (id, name, lib) in image_codecs {
            self.codecs.insert(
                format!("img_{}", id),
                CodecInfo {
                    name: name.to_string(),
                    category: CodecCategory::ImageDecoder,
                    library: lib.to_string(),
                    supported: true,
                },
            );
        }
    }
}

impl Default for CodecRegistry {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_registry_initialization() {
        let registry = CodecRegistry::new();
        assert!(registry.handler_count() > 0);
    }

    #[test]
    fn test_codec_lookup() {
        let registry = CodecRegistry::new();
        
        assert!(registry.is_supported("dec_h264"));
        assert!(registry.is_supported("dec_hevc"));
        assert!(registry.is_supported("enc_libx264"));
        assert!(registry.is_supported("img_exr"));
    }

    #[test]
    fn test_category_filter() {
        let registry = CodecRegistry::new();
        
        let video_decoders = registry.list_by_category(CodecCategory::VideoDecoder);
        assert!(!video_decoders.is_empty());
        
        let audio_encoders = registry.list_by_category(CodecCategory::AudioEncoder);
        assert!(!audio_encoders.is_empty());
    }
}
