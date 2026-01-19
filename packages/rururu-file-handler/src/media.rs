use std::path::Path;
use std::time::Duration;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum MediaError {
    #[error("FFmpeg not available")]
    FfmpegNotAvailable,
    #[error("Failed to open media: {0}")]
    OpenError(String),
    #[error("Failed to extract metadata: {0}")]
    MetadataError(String),
    #[error("Unsupported format: {0}")]
    UnsupportedFormat(String),
    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct VideoInfo {
    pub width: u32,
    pub height: u32,
    pub duration: Option<Duration>,
    pub frame_rate: Option<f64>,
    pub codec: Option<String>,
    pub bitrate: Option<u64>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct AudioInfo {
    pub channels: u32,
    pub sample_rate: u32,
    pub duration: Option<Duration>,
    pub codec: Option<String>,
    pub bitrate: Option<u64>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MediaInfo {
    pub video: Option<VideoInfo>,
    pub audio: Option<AudioInfo>,
    pub container: Option<String>,
    pub title: Option<String>,
    pub artist: Option<String>,
    pub album: Option<String>,
}

pub struct MediaHandler {
    #[cfg(feature = "ffmpeg")]
    _ffmpeg_initialized: bool,
}

impl MediaHandler {
    pub fn new() -> Result<Self, MediaError> {
        #[cfg(feature = "ffmpeg")]
        {
            ffmpeg_next::init().map_err(|e| MediaError::OpenError(e.to_string()))?;
            Ok(Self {
                _ffmpeg_initialized: true,
            })
        }

        #[cfg(not(feature = "ffmpeg"))]
        {
            Ok(Self {})
        }
    }

    #[cfg(feature = "ffmpeg")]
    pub fn get_info(&self, path: &Path) -> Result<MediaInfo, MediaError> {
        use ffmpeg_next::format::context::Input;
        use ffmpeg_next::media::Type;

        let context = ffmpeg_next::format::input(&path)
            .map_err(|e| MediaError::OpenError(e.to_string()))?;

        let mut video_info = None;
        let mut audio_info = None;

        for stream in context.streams() {
            match stream.parameters().medium() {
                Type::Video => {
                    let decoder = ffmpeg_next::codec::context::Context::from_parameters(
                        stream.parameters(),
                    )
                    .map_err(|e| MediaError::MetadataError(e.to_string()))?;

                    if let Ok(video) = decoder.decoder().video() {
                        video_info = Some(VideoInfo {
                            width: video.width(),
                            height: video.height(),
                            duration: stream.duration().map(|d| {
                                let time_base = stream.time_base();
                                Duration::from_secs_f64(
                                    d as f64 * time_base.numerator() as f64
                                        / time_base.denominator() as f64,
                                )
                            }),
                            frame_rate: stream.avg_frame_rate().map(|r| {
                                r.numerator() as f64 / r.denominator() as f64
                            }),
                            codec: stream
                                .parameters()
                                .id()
                                .name()
                                .map(String::from),
                            bitrate: Some(stream.parameters().bit_rate() as u64),
                        });
                    }
                }
                Type::Audio => {
                    let decoder = ffmpeg_next::codec::context::Context::from_parameters(
                        stream.parameters(),
                    )
                    .map_err(|e| MediaError::MetadataError(e.to_string()))?;

                    if let Ok(audio) = decoder.decoder().audio() {
                        audio_info = Some(AudioInfo {
                            channels: audio.channels() as u32,
                            sample_rate: audio.rate(),
                            duration: stream.duration().map(|d| {
                                let time_base = stream.time_base();
                                Duration::from_secs_f64(
                                    d as f64 * time_base.numerator() as f64
                                        / time_base.denominator() as f64,
                                )
                            }),
                            codec: stream
                                .parameters()
                                .id()
                                .name()
                                .map(String::from),
                            bitrate: Some(stream.parameters().bit_rate() as u64),
                        });
                    }
                }
                _ => {}
            }
        }

        let metadata = context.metadata();
        
        Ok(MediaInfo {
            video: video_info,
            audio: audio_info,
            container: context.format().name().map(String::from),
            title: metadata.get("title").map(String::from),
            artist: metadata.get("artist").map(String::from),
            album: metadata.get("album").map(String::from),
        })
    }

    #[cfg(not(feature = "ffmpeg"))]
    pub fn get_info(&self, _path: &Path) -> Result<MediaInfo, MediaError> {
        Err(MediaError::FfmpegNotAvailable)
    }

    pub fn get_audio_metadata(&self, path: &Path) -> Result<AudioInfo, MediaError> {
        let ext = path
            .extension()
            .and_then(|e| e.to_str())
            .unwrap_or("")
            .to_lowercase();

        match ext.as_str() {
            "mp3" => self.get_mp3_info(path),
            _ => {
                #[cfg(feature = "ffmpeg")]
                {
                    self.get_info(path)?
                        .audio
                        .ok_or_else(|| MediaError::UnsupportedFormat("No audio stream".into()))
                }
                #[cfg(not(feature = "ffmpeg"))]
                {
                    Err(MediaError::UnsupportedFormat(ext))
                }
            }
        }
    }

    fn get_mp3_info(&self, path: &Path) -> Result<AudioInfo, MediaError> {
        let tag = id3::Tag::read_from_path(path)
            .map_err(|e| MediaError::MetadataError(e.to_string()))?;

        Ok(AudioInfo {
            channels: 2, // MP3 is typically stereo
            sample_rate: 44100, // Common default
            duration: tag.duration().map(Duration::from_secs),
            codec: Some("MP3".to_string()),
            bitrate: None,
        })
    }
}

impl Default for MediaHandler {
    fn default() -> Self {
        Self::new().expect("Failed to initialize media handler")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_media_handler_creation() {
        let handler = MediaHandler::new();
        assert!(handler.is_ok());
    }
}
