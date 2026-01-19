use std::path::Path;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum DetectorError {
    #[error("Failed to read file: {0}")]
    IoError(#[from] std::io::Error),
    #[error("Unknown file format")]
    UnknownFormat,
}

#[derive(Debug, Clone, PartialEq)]
pub enum FileCategory {
    Video,
    Audio,
    Image,
    Document,
    Model3D,
    Archive,
    Code,
    Unknown,
}

#[derive(Debug, Clone)]
pub struct FileInfo {
    pub mime_type: String,
    pub category: FileCategory,
    pub extension: Option<String>,
    pub codec: Option<String>,
}

pub struct FileDetector {
    // Using infer crate for magic byte detection
}

impl FileDetector {
    pub fn new() -> Self {
        Self {}
    }

    pub fn detect(&self, path: &Path) -> Result<FileInfo, DetectorError> {
        let data = std::fs::read(path)?;
        self.detect_from_bytes(&data, path.extension().and_then(|e| e.to_str()))
    }

    pub fn detect_from_bytes(
        &self,
        data: &[u8],
        extension: Option<&str>,
    ) -> Result<FileInfo, DetectorError> {
        // Try magic byte detection first
        if let Some(kind) = infer::get(data) {
            let category = self.categorize_mime(kind.mime_type());
            return Ok(FileInfo {
                mime_type: kind.mime_type().to_string(),
                category,
                extension: extension.map(String::from),
                codec: self.detect_codec(kind.mime_type(), data),
            });
        }

        // Fallback to extension-based detection
        if let Some(ext) = extension {
            return self.detect_by_extension(ext);
        }

        Err(DetectorError::UnknownFormat)
    }

    fn categorize_mime(&self, mime: &str) -> FileCategory {
        match mime.split('/').next() {
            Some("video") => FileCategory::Video,
            Some("audio") => FileCategory::Audio,
            Some("image") => FileCategory::Image,
            Some("application") => {
                if mime.contains("pdf") || mime.contains("document") {
                    FileCategory::Document
                } else if mime.contains("zip") || mime.contains("tar") || mime.contains("gzip") {
                    FileCategory::Archive
                } else if mime.contains("gltf") || mime.contains("obj") {
                    FileCategory::Model3D
                } else {
                    FileCategory::Unknown
                }
            }
            Some("text") => FileCategory::Code,
            Some("model") => FileCategory::Model3D,
            _ => FileCategory::Unknown,
        }
    }

    fn detect_codec(&self, mime: &str, _data: &[u8]) -> Option<String> {
        // Basic codec detection from MIME type
        // Full detection requires parsing container format
        match mime {
            "video/mp4" => Some("H.264/AAC".to_string()),
            "video/webm" => Some("VP9/Opus".to_string()),
            "video/x-matroska" => Some("MKV container".to_string()),
            "audio/mpeg" => Some("MP3".to_string()),
            "audio/flac" => Some("FLAC".to_string()),
            "audio/ogg" => Some("Vorbis".to_string()),
            "image/jpeg" => Some("JPEG".to_string()),
            "image/png" => Some("PNG".to_string()),
            "image/webp" => Some("WebP".to_string()),
            "image/avif" => Some("AVIF".to_string()),
            _ => None,
        }
    }

    fn detect_by_extension(&self, ext: &str) -> Result<FileInfo, DetectorError> {
        let (mime, category, codec) = match ext.to_lowercase().as_str() {
            // Video
            "mp4" | "m4v" => ("video/mp4", FileCategory::Video, Some("H.264")),
            "mkv" => ("video/x-matroska", FileCategory::Video, None),
            "webm" => ("video/webm", FileCategory::Video, Some("VP9")),
            "mov" => ("video/quicktime", FileCategory::Video, Some("ProRes")),
            "avi" => ("video/x-msvideo", FileCategory::Video, None),

            // Audio
            "mp3" => ("audio/mpeg", FileCategory::Audio, Some("MP3")),
            "flac" => ("audio/flac", FileCategory::Audio, Some("FLAC")),
            "wav" => ("audio/wav", FileCategory::Audio, Some("PCM")),
            "ogg" => ("audio/ogg", FileCategory::Audio, Some("Vorbis")),
            "opus" => ("audio/opus", FileCategory::Audio, Some("Opus")),
            "aac" => ("audio/aac", FileCategory::Audio, Some("AAC")),
            "m4a" => ("audio/mp4", FileCategory::Audio, Some("AAC")),

            // Image
            "jpg" | "jpeg" => ("image/jpeg", FileCategory::Image, Some("JPEG")),
            "png" => ("image/png", FileCategory::Image, Some("PNG")),
            "gif" => ("image/gif", FileCategory::Image, Some("GIF")),
            "webp" => ("image/webp", FileCategory::Image, Some("WebP")),
            "avif" => ("image/avif", FileCategory::Image, Some("AVIF")),
            "heic" => ("image/heic", FileCategory::Image, Some("HEVC")),
            "tiff" | "tif" => ("image/tiff", FileCategory::Image, Some("TIFF")),
            "exr" => ("image/x-exr", FileCategory::Image, Some("OpenEXR")),
            "hdr" => ("image/vnd.radiance", FileCategory::Image, Some("RGBE")),

            // RAW
            "cr2" | "cr3" => ("image/x-canon-cr2", FileCategory::Image, Some("Canon RAW")),
            "nef" => ("image/x-nikon-nef", FileCategory::Image, Some("Nikon RAW")),
            "arw" => ("image/x-sony-arw", FileCategory::Image, Some("Sony RAW")),
            "dng" => ("image/x-adobe-dng", FileCategory::Image, Some("DNG")),

            // 3D
            "gltf" | "glb" => ("model/gltf+json", FileCategory::Model3D, Some("glTF")),
            "obj" => ("model/obj", FileCategory::Model3D, None),
            "fbx" => (
                "application/octet-stream",
                FileCategory::Model3D,
                Some("FBX"),
            ),
            "blend" => (
                "application/x-blender",
                FileCategory::Model3D,
                Some("Blender"),
            ),
            "stl" => ("model/stl", FileCategory::Model3D, None),
            "usd" | "usda" | "usdc" | "usdz" => {
                ("model/vnd.usd+zip", FileCategory::Model3D, Some("USD"))
            }

            // Documents
            "pdf" => ("application/pdf", FileCategory::Document, None),
            "docx" => (
                "application/vnd.openxmlformats-officedocument.wordprocessingml.document",
                FileCategory::Document,
                None,
            ),
            "xlsx" => (
                "application/vnd.openxmlformats-officedocument.spreadsheetml.sheet",
                FileCategory::Document,
                None,
            ),
            "odt" => (
                "application/vnd.oasis.opendocument.text",
                FileCategory::Document,
                None,
            ),
            "md" => ("text/markdown", FileCategory::Document, None),
            "txt" => ("text/plain", FileCategory::Document, None),

            // Code
            "rs" => ("text/x-rust", FileCategory::Code, None),
            "py" => ("text/x-python", FileCategory::Code, None),
            "js" => ("text/javascript", FileCategory::Code, None),
            "ts" => ("text/typescript", FileCategory::Code, None),
            "go" => ("text/x-go", FileCategory::Code, None),
            "c" | "h" => ("text/x-c", FileCategory::Code, None),
            "cpp" | "hpp" => ("text/x-c++", FileCategory::Code, None),

            _ => return Err(DetectorError::UnknownFormat),
        };

        Ok(FileInfo {
            mime_type: mime.to_string(),
            category,
            extension: Some(ext.to_string()),
            codec: codec.map(String::from),
        })
    }
}

impl Default for FileDetector {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extension_detection() {
        let detector = FileDetector::new();

        let info = detector.detect_by_extension("mp4").unwrap();
        assert_eq!(info.category, FileCategory::Video);

        let info = detector.detect_by_extension("flac").unwrap();
        assert_eq!(info.category, FileCategory::Audio);

        let info = detector.detect_by_extension("exr").unwrap();
        assert_eq!(info.category, FileCategory::Image);

        let info = detector.detect_by_extension("gltf").unwrap();
        assert_eq!(info.category, FileCategory::Model3D);
    }
}
