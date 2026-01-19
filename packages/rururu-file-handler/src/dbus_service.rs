use std::path::PathBuf;
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::RwLock;
use zbus::{interface, Connection};

use crate::cache::{CachedMetadata, MetadataCache};
use crate::codec_registry::CodecRegistry;
use crate::file_detector::FileDetector;
use crate::media::MediaHandler;
use crate::plugin::PluginManager;
use crate::thumbnail::{ThumbnailGenerator, ThumbnailSize};

pub struct FileHandlerService {
    detector: FileDetector,
    registry: Arc<RwLock<CodecRegistry>>,
    media_handler: MediaHandler,
    thumbnail_gen: ThumbnailGenerator,
    cache: MetadataCache,
    plugin_manager: Arc<RwLock<PluginManager>>,
}

impl FileHandlerService {
    pub fn new(
        cache_dir: PathBuf,
        plugin_dir: PathBuf,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        let detector = FileDetector::new();
        let registry = Arc::new(RwLock::new(CodecRegistry::new()));
        let media_handler = MediaHandler::new()?;
        let thumbnail_gen = ThumbnailGenerator::new(cache_dir.join("thumbnails"));
        let cache = MetadataCache::new(&cache_dir.join("metadata"), Duration::from_secs(3600))?;

        let mut plugin_manager = PluginManager::new(plugin_dir);
        plugin_manager.load_all()?;

        Ok(Self {
            detector,
            registry,
            media_handler,
            thumbnail_gen,
            cache,
            plugin_manager: Arc::new(RwLock::new(plugin_manager)),
        })
    }
}

#[interface(name = "org.rururu.FileHandler1")]
impl FileHandlerService {
    async fn detect_file(&self, path: &str) -> String {
        match self.detector.detect(std::path::Path::new(path)) {
            Ok(info) => serde_json::to_string(&info).unwrap_or_else(|_| "{}".to_string()),
            Err(e) => format!(r#"{{"error": "{}"}}"#, e),
        }
    }

    async fn get_metadata(&self, path: &str) -> String {
        let path_buf = PathBuf::from(path);

        // Check cache first
        if let Some(cached) = self.cache.get(&path_buf) {
            return serde_json::to_string(&cached.metadata).unwrap_or_default();
        }

        // Try media handler
        #[cfg(feature = "ffmpeg")]
        if let Ok(info) = self.media_handler.get_info(&path_buf) {
            let metadata = serde_json::to_value(&info).unwrap_or_default();

            // Cache result
            if let Ok(file_meta) = path_buf.metadata() {
                let cached = CachedMetadata {
                    mime_type: "media".to_string(),
                    size: file_meta.len(),
                    modified: file_meta.modified().unwrap_or(std::time::SystemTime::now()),
                    metadata: metadata.clone(),
                    cached_at: std::time::SystemTime::now(),
                };
                let _ = self.cache.set(&path_buf, cached);
            }

            return serde_json::to_string(&metadata).unwrap_or_default();
        }

        // Try plugins
        let ext = path_buf
            .extension()
            .and_then(|e| e.to_str())
            .unwrap_or("")
            .to_lowercase();

        let plugin_manager = self.plugin_manager.read().await;
        if let Some(plugin) = plugin_manager.get_plugin_for_extension(&ext) {
            if let Ok(metadata) = plugin.get_metadata(&path_buf) {
                return serde_json::to_string(&metadata).unwrap_or_default();
            }
        }

        r#"{"error": "Unable to extract metadata"}"#.to_string()
    }

    async fn generate_thumbnail(&self, path: &str, size: &str) -> String {
        let path_buf = PathBuf::from(path);
        let thumb_size = match size {
            "small" => ThumbnailSize::SMALL,
            "large" => ThumbnailSize::LARGE,
            _ => ThumbnailSize::MEDIUM,
        };

        match self.thumbnail_gen.generate(&path_buf, thumb_size) {
            Ok(thumb_path) => {
                format!(r#"{{"path": "{}"}}"#, thumb_path.display())
            }
            Err(e) => format!(r#"{{"error": "{}"}}"#, e),
        }
    }

    async fn list_codecs(&self) -> String {
        let registry = self.registry.read().await;
        let codecs: Vec<_> = registry.list_all().collect();
        serde_json::to_string(&codecs).unwrap_or_else(|_| "[]".to_string())
    }

    async fn list_plugins(&self) -> String {
        let plugin_manager = self.plugin_manager.read().await;
        let plugins: Vec<_> = plugin_manager
            .list_plugins()
            .iter()
            .map(|(name, version, exts)| {
                serde_json::json!({
                    "name": name,
                    "version": version,
                    "extensions": exts,
                })
            })
            .collect();
        serde_json::to_string(&plugins).unwrap_or_else(|_| "[]".to_string())
    }

    async fn clear_cache(&self) -> bool {
        self.cache.clear().is_ok() && self.thumbnail_gen.clear_cache().is_ok()
    }

    async fn cache_stats(&self) -> String {
        let stats = self.cache.stats();
        serde_json::json!({
            "entries": stats.entries,
            "size_bytes": stats.size_bytes,
        })
        .to_string()
    }

    async fn get_supported_formats(&self) -> String {
        let registry = self.registry.read().await;
        let formats: Vec<_> = registry
            .list_all()
            .flat_map(|c| c.extensions.clone())
            .collect();
        serde_json::to_string(&formats).unwrap_or_else(|_| "[]".to_string())
    }
}

pub async fn run_service(
    cache_dir: PathBuf,
    plugin_dir: PathBuf,
) -> Result<(), Box<dyn std::error::Error>> {
    let service = FileHandlerService::new(cache_dir, plugin_dir)?;

    let connection = Connection::session().await?;

    connection
        .object_server()
        .at("/org/rururu/FileHandler", service)
        .await?;

    connection.request_name("org.rururu.FileHandler").await?;

    tracing::info!("D-Bus service started: org.rururu.FileHandler");

    // Keep running
    loop {
        tokio::time::sleep(Duration::from_secs(3600)).await;
    }
}
