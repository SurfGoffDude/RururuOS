use std::collections::HashMap;
use std::ffi::OsStr;
use std::path::{Path, PathBuf};
use libloading::{Library, Symbol};
use thiserror::Error;
use tracing::{debug, error, info, warn};

#[derive(Error, Debug)]
pub enum PluginError {
    #[error("Failed to load plugin: {0}")]
    LoadError(String),
    #[error("Plugin not found: {0}")]
    NotFound(String),
    #[error("Invalid plugin: {0}")]
    InvalidPlugin(String),
    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),
}

#[repr(C)]
pub struct PluginInfo {
    pub name: *const std::ffi::c_char,
    pub version: *const std::ffi::c_char,
    pub description: *const std::ffi::c_char,
    pub supported_extensions: *const *const std::ffi::c_char,
    pub extension_count: usize,
}

#[repr(C)]
pub struct FileMetadata {
    pub mime_type: *const std::ffi::c_char,
    pub width: u32,
    pub height: u32,
    pub duration_ms: u64,
    pub extra_json: *const std::ffi::c_char,
}

type PluginInfoFn = unsafe extern "C" fn() -> PluginInfo;
type PluginInitFn = unsafe extern "C" fn() -> i32;
type PluginDeinitFn = unsafe extern "C" fn();
type GetMetadataFn = unsafe extern "C" fn(*const std::ffi::c_char) -> *mut FileMetadata;
type FreeMetadataFn = unsafe extern "C" fn(*mut FileMetadata);
type GenerateThumbnailFn = unsafe extern "C" fn(
    *const std::ffi::c_char,
    *const std::ffi::c_char,
    u32,
    u32,
) -> i32;

pub struct LoadedPlugin {
    _library: Library,
    pub name: String,
    pub version: String,
    pub description: String,
    pub extensions: Vec<String>,
    get_metadata: Option<GetMetadataFn>,
    free_metadata: Option<FreeMetadataFn>,
    generate_thumbnail: Option<GenerateThumbnailFn>,
}

impl LoadedPlugin {
    pub fn get_metadata(&self, path: &Path) -> Result<serde_json::Value, PluginError> {
        let get_fn = self
            .get_metadata
            .ok_or_else(|| PluginError::InvalidPlugin("No get_metadata function".into()))?;
        let free_fn = self.free_metadata;

        let path_cstr = std::ffi::CString::new(path.to_string_lossy().as_bytes())
            .map_err(|e| PluginError::InvalidPlugin(e.to_string()))?;

        unsafe {
            let metadata_ptr = get_fn(path_cstr.as_ptr());
            if metadata_ptr.is_null() {
                return Err(PluginError::InvalidPlugin("Metadata extraction failed".into()));
            }

            let metadata = &*metadata_ptr;
            let result = serde_json::json!({
                "mime_type": if metadata.mime_type.is_null() {
                    None
                } else {
                    Some(std::ffi::CStr::from_ptr(metadata.mime_type).to_string_lossy().to_string())
                },
                "width": metadata.width,
                "height": metadata.height,
                "duration_ms": metadata.duration_ms,
            });

            if let Some(free) = free_fn {
                free(metadata_ptr);
            }

            Ok(result)
        }
    }

    pub fn generate_thumbnail(
        &self,
        source: &Path,
        dest: &Path,
        width: u32,
        height: u32,
    ) -> Result<(), PluginError> {
        let gen_fn = self.generate_thumbnail.ok_or_else(|| {
            PluginError::InvalidPlugin("No generate_thumbnail function".into())
        })?;

        let source_cstr = std::ffi::CString::new(source.to_string_lossy().as_bytes())
            .map_err(|e| PluginError::InvalidPlugin(e.to_string()))?;
        let dest_cstr = std::ffi::CString::new(dest.to_string_lossy().as_bytes())
            .map_err(|e| PluginError::InvalidPlugin(e.to_string()))?;

        unsafe {
            let result = gen_fn(source_cstr.as_ptr(), dest_cstr.as_ptr(), width, height);
            if result != 0 {
                return Err(PluginError::InvalidPlugin(format!(
                    "Thumbnail generation failed with code {}",
                    result
                )));
            }
        }

        Ok(())
    }
}

pub struct PluginManager {
    plugin_dir: PathBuf,
    plugins: HashMap<String, LoadedPlugin>,
    extension_map: HashMap<String, String>, // extension -> plugin name
}

impl PluginManager {
    pub fn new(plugin_dir: PathBuf) -> Self {
        Self {
            plugin_dir,
            plugins: HashMap::new(),
            extension_map: HashMap::new(),
        }
    }

    pub fn load_all(&mut self) -> Result<(), PluginError> {
        if !self.plugin_dir.exists() {
            warn!("Plugin directory does not exist: {:?}", self.plugin_dir);
            return Ok(());
        }

        let entries = std::fs::read_dir(&self.plugin_dir)?;

        for entry in entries.flatten() {
            let path = entry.path();
            if self.is_plugin_file(&path) {
                match self.load_plugin(&path) {
                    Ok(()) => info!("Loaded plugin: {:?}", path),
                    Err(e) => error!("Failed to load plugin {:?}: {}", path, e),
                }
            }
        }

        Ok(())
    }

    fn is_plugin_file(&self, path: &Path) -> bool {
        let ext = path.extension().and_then(OsStr::to_str);
        
        #[cfg(target_os = "linux")]
        return ext == Some("so");
        
        #[cfg(target_os = "macos")]
        return ext == Some("dylib");
        
        #[cfg(target_os = "windows")]
        return ext == Some("dll");
        
        #[cfg(not(any(target_os = "linux", target_os = "macos", target_os = "windows")))]
        return false;
    }

    pub fn load_plugin(&mut self, path: &Path) -> Result<(), PluginError> {
        unsafe {
            let library = Library::new(path)
                .map_err(|e| PluginError::LoadError(e.to_string()))?;

            // Get plugin info
            let info_fn: Symbol<PluginInfoFn> = library
                .get(b"rururu_plugin_info")
                .map_err(|e| PluginError::InvalidPlugin(e.to_string()))?;

            let info = info_fn();

            let name = std::ffi::CStr::from_ptr(info.name)
                .to_string_lossy()
                .to_string();
            let version = std::ffi::CStr::from_ptr(info.version)
                .to_string_lossy()
                .to_string();
            let description = std::ffi::CStr::from_ptr(info.description)
                .to_string_lossy()
                .to_string();

            let mut extensions = Vec::new();
            for i in 0..info.extension_count {
                let ext_ptr = *info.supported_extensions.add(i);
                let ext = std::ffi::CStr::from_ptr(ext_ptr)
                    .to_string_lossy()
                    .to_string();
                extensions.push(ext);
            }

            // Initialize plugin
            if let Ok(init_fn) = library.get::<PluginInitFn>(b"rururu_plugin_init") {
                let result = init_fn();
                if result != 0 {
                    return Err(PluginError::InvalidPlugin(format!(
                        "Plugin init failed with code {}",
                        result
                    )));
                }
            }

            // Get optional functions
            let get_metadata = library
                .get::<GetMetadataFn>(b"rururu_get_metadata")
                .ok()
                .map(|s| *s);
            let free_metadata = library
                .get::<FreeMetadataFn>(b"rururu_free_metadata")
                .ok()
                .map(|s| *s);
            let generate_thumbnail = library
                .get::<GenerateThumbnailFn>(b"rururu_generate_thumbnail")
                .ok()
                .map(|s| *s);

            // Register extensions
            for ext in &extensions {
                self.extension_map.insert(ext.clone(), name.clone());
            }

            let plugin = LoadedPlugin {
                _library: library,
                name: name.clone(),
                version,
                description,
                extensions,
                get_metadata,
                free_metadata,
                generate_thumbnail,
            };

            debug!("Registered plugin: {} with {} extensions", name, plugin.extensions.len());
            self.plugins.insert(name, plugin);
        }

        Ok(())
    }

    pub fn get_plugin_for_extension(&self, ext: &str) -> Option<&LoadedPlugin> {
        self.extension_map
            .get(&ext.to_lowercase())
            .and_then(|name| self.plugins.get(name))
    }

    pub fn list_plugins(&self) -> Vec<(&str, &str, &[String])> {
        self.plugins
            .values()
            .map(|p| (p.name.as_str(), p.version.as_str(), p.extensions.as_slice()))
            .collect()
    }

    pub fn plugin_count(&self) -> usize {
        self.plugins.len()
    }
}

impl Drop for PluginManager {
    fn drop(&mut self) {
        for (name, plugin) in &self.plugins {
            unsafe {
                if let Ok(deinit_fn) = plugin._library.get::<PluginDeinitFn>(b"rururu_plugin_deinit")
                {
                    debug!("Deinitializing plugin: {}", name);
                    deinit_fn();
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[test]
    fn test_plugin_manager_empty() {
        let dir = tempdir().unwrap();
        let mut manager = PluginManager::new(dir.path().to_path_buf());
        assert!(manager.load_all().is_ok());
        assert_eq!(manager.plugin_count(), 0);
    }
}
