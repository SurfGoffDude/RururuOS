//! Example plugin for RururuOS File Handler
//!
//! This demonstrates how to create a plugin that adds support
//! for custom file formats.

use std::ffi::{c_char, CStr, CString};
use std::ptr;

#[repr(C)]
pub struct PluginInfo {
    pub name: *const c_char,
    pub version: *const c_char,
    pub description: *const c_char,
    pub supported_extensions: *const *const c_char,
    pub extension_count: usize,
}

#[repr(C)]
pub struct FileMetadata {
    pub mime_type: *const c_char,
    pub width: u32,
    pub height: u32,
    pub duration_ms: u64,
    pub extra_json: *const c_char,
}

static PLUGIN_NAME: &[u8] = b"Example Plugin\0";
static PLUGIN_VERSION: &[u8] = b"0.1.0\0";
static PLUGIN_DESC: &[u8] = b"Example plugin demonstrating the RururuOS plugin API\0";

static EXT_EXAMPLE: &[u8] = b"example\0";
static EXT_TEST: &[u8] = b"test\0";

/// Thread-safe wrapper for extension pointers
struct ExtensionsWrapper([*const c_char; 2]);
unsafe impl Sync for ExtensionsWrapper {}

static EXTENSIONS: ExtensionsWrapper = ExtensionsWrapper([
    EXT_EXAMPLE.as_ptr() as *const c_char,
    EXT_TEST.as_ptr() as *const c_char,
]);

#[no_mangle]
pub extern "C" fn rururu_plugin_info() -> PluginInfo {
    PluginInfo {
        name: PLUGIN_NAME.as_ptr() as *const c_char,
        version: PLUGIN_VERSION.as_ptr() as *const c_char,
        description: PLUGIN_DESC.as_ptr() as *const c_char,
        supported_extensions: EXTENSIONS.0.as_ptr(),
        extension_count: EXTENSIONS.0.len(),
    }
}

#[no_mangle]
pub extern "C" fn rururu_plugin_init() -> i32 {
    // Initialize plugin resources
    // Return 0 on success, non-zero on failure
    0
}

#[no_mangle]
pub extern "C" fn rururu_plugin_deinit() {
    // Cleanup plugin resources
}

/// Get metadata for a file.
///
/// # Safety
/// - `path` must be a valid null-terminated C string pointer.
/// - The returned pointer must be freed using `rururu_free_metadata`.
#[no_mangle]
pub unsafe extern "C" fn rururu_get_metadata(path: *const c_char) -> *mut FileMetadata {
    if path.is_null() {
        return ptr::null_mut();
    }

    let path_str = match CStr::from_ptr(path).to_str() {
        Ok(s) => s,
        Err(_) => return ptr::null_mut(),
    };

    // Example: extract metadata from file
    // In real plugin, you would parse the actual file format

    let mime_type = CString::new("application/x-example").unwrap();
    let extra = CString::new(format!(r#"{{"source": "{}"}}"#, path_str)).unwrap();

    let metadata = Box::new(FileMetadata {
        mime_type: mime_type.into_raw(),
        width: 1920,
        height: 1080,
        duration_ms: 0,
        extra_json: extra.into_raw(),
    });

    Box::into_raw(metadata)
}

/// Free metadata previously returned by `rururu_get_metadata`.
///
/// # Safety
/// - `metadata` must be a pointer returned by `rururu_get_metadata`, or null.
/// - Each pointer must only be freed once.
#[no_mangle]
pub unsafe extern "C" fn rururu_free_metadata(metadata: *mut FileMetadata) {
    if metadata.is_null() {
        return;
    }

    let metadata = Box::from_raw(metadata);

    // Free strings
    if !metadata.mime_type.is_null() {
        drop(CString::from_raw(metadata.mime_type as *mut c_char));
    }
    if !metadata.extra_json.is_null() {
        drop(CString::from_raw(metadata.extra_json as *mut c_char));
    }
}

#[no_mangle]
pub extern "C" fn rururu_generate_thumbnail(
    _source: *const c_char,
    _dest: *const c_char,
    _width: u32,
    _height: u32,
) -> i32 {
    // Example: generate thumbnail
    // Return 0 on success, non-zero on failure

    // Not implemented in this example
    -1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_plugin_info() {
        let info = rururu_plugin_info();
        assert!(!info.name.is_null());
        assert_eq!(info.extension_count, 2);
    }

    #[test]
    fn test_init_deinit() {
        assert_eq!(rururu_plugin_init(), 0);
        rururu_plugin_deinit();
    }
}
