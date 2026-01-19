use crate::{ColorError, Result};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MonitorProfile {
    pub name: String,
    pub edid: EdidInfo,
    pub capabilities: MonitorCapabilities,
    pub calibration: Option<CalibrationData>,
    pub icc_profile: Option<PathBuf>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EdidInfo {
    pub manufacturer: String,
    pub model: String,
    pub serial: Option<String>,
    pub year: u16,
    pub resolution: (u32, u32),
    pub physical_size_mm: Option<(u32, u32)>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MonitorCapabilities {
    pub color_depth: ColorDepth,
    pub hdr_support: HdrCapability,
    pub wide_gamut: bool,
    pub native_gamma: f32,
    pub max_luminance: Option<u32>,
    pub min_luminance: Option<f32>,
    pub color_gamut: ColorGamut,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum ColorDepth {
    Bit8,
    Bit10,
    Bit12,
    Bit16,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum HdrCapability {
    None,
    Hdr10,
    Hdr10Plus,
    DolbyVision,
    HlgBt2100,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum ColorGamut {
    Srgb,
    AdobeRgb,
    DciP3,
    Bt2020,
    Unknown,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CalibrationData {
    pub date: String,
    pub white_point: WhitePoint,
    pub gamma: f32,
    pub brightness: f32,
    pub contrast: f32,
    pub rgb_gains: (f32, f32, f32),
    pub gamma_curve: Option<Vec<(f32, f32)>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WhitePoint {
    pub temperature: u32,
    pub x: f32,
    pub y: f32,
}

impl Default for WhitePoint {
    fn default() -> Self {
        Self::d65()
    }
}

impl WhitePoint {
    pub fn d65() -> Self {
        Self {
            temperature: 6500,
            x: 0.3127,
            y: 0.3290,
        }
    }
    
    pub fn d50() -> Self {
        Self {
            temperature: 5000,
            x: 0.3457,
            y: 0.3585,
        }
    }
    
    pub fn d93() -> Self {
        Self {
            temperature: 9300,
            x: 0.2848,
            y: 0.2932,
        }
    }
}

pub fn detect_monitors() -> Result<Vec<MonitorProfile>> {
    let mut monitors = Vec::new();
    
    #[cfg(target_os = "linux")]
    {
        // Try to read from /sys/class/drm
        let drm_path = std::path::Path::new("/sys/class/drm");
        
        if drm_path.exists() {
            if let Ok(entries) = std::fs::read_dir(drm_path) {
                for entry in entries.flatten() {
                    let name = entry.file_name().to_string_lossy().to_string();
                    
                    // Skip non-connector entries
                    if !name.starts_with("card") || !name.contains('-') {
                        continue;
                    }
                    
                    let status_path = entry.path().join("status");
                    if let Ok(status) = std::fs::read_to_string(&status_path) {
                        if status.trim() != "connected" {
                            continue;
                        }
                    }
                    
                    // Read EDID
                    let edid_path = entry.path().join("edid");
                    let edid = if edid_path.exists() {
                        parse_edid(&edid_path).unwrap_or_else(|_| default_edid(&name))
                    } else {
                        default_edid(&name)
                    };
                    
                    monitors.push(MonitorProfile {
                        name: name.clone(),
                        edid,
                        capabilities: detect_capabilities(&entry.path()),
                        calibration: None,
                        icc_profile: None,
                    });
                }
            }
        }
    }
    
    // If no monitors found, return a placeholder
    if monitors.is_empty() {
        monitors.push(MonitorProfile {
            name: "Default".to_string(),
            edid: EdidInfo {
                manufacturer: "Unknown".to_string(),
                model: "Unknown Monitor".to_string(),
                serial: None,
                year: 2024,
                resolution: (1920, 1080),
                physical_size_mm: None,
            },
            capabilities: MonitorCapabilities {
                color_depth: ColorDepth::Bit8,
                hdr_support: HdrCapability::None,
                wide_gamut: false,
                native_gamma: 2.2,
                max_luminance: Some(300),
                min_luminance: Some(0.5),
                color_gamut: ColorGamut::Srgb,
            },
            calibration: None,
            icc_profile: None,
        });
    }
    
    Ok(monitors)
}

fn parse_edid(path: &std::path::Path) -> Result<EdidInfo> {
    let data = std::fs::read(path)?;
    
    if data.len() < 128 {
        return Err(ColorError::IccError("EDID too small".to_string()));
    }
    
    // Parse EDID header
    let manufacturer_id = ((data[8] as u16) << 8) | (data[9] as u16);
    let manufacturer = decode_manufacturer_id(manufacturer_id);
    
    let year = 1990 + data[17] as u16;
    
    // Resolution from detailed timing descriptor
    let h_active = ((data[58] as u32 & 0xF0) << 4) | data[56] as u32;
    let v_active = ((data[61] as u32 & 0xF0) << 4) | data[59] as u32;
    
    // Physical size
    let h_size = ((data[68] as u32 & 0xF0) << 4) | data[66] as u32;
    let v_size = ((data[68] as u32 & 0x0F) << 8) | data[67] as u32;
    
    Ok(EdidInfo {
        manufacturer,
        model: "Monitor".to_string(),
        serial: None,
        year,
        resolution: (h_active.max(1920), v_active.max(1080)),
        physical_size_mm: if h_size > 0 && v_size > 0 {
            Some((h_size * 10, v_size * 10))
        } else {
            None
        },
    })
}

fn decode_manufacturer_id(id: u16) -> String {
    let c1 = ((id >> 10) & 0x1F) as u8 + b'A' - 1;
    let c2 = ((id >> 5) & 0x1F) as u8 + b'A' - 1;
    let c3 = (id & 0x1F) as u8 + b'A' - 1;
    
    format!("{}{}{}", c1 as char, c2 as char, c3 as char)
}

fn default_edid(name: &str) -> EdidInfo {
    EdidInfo {
        manufacturer: "Unknown".to_string(),
        model: name.to_string(),
        serial: None,
        year: 2024,
        resolution: (1920, 1080),
        physical_size_mm: None,
    }
}

fn detect_capabilities(_path: &std::path::Path) -> MonitorCapabilities {
    // Default capabilities - would need deeper inspection for accurate values
    MonitorCapabilities {
        color_depth: ColorDepth::Bit8,
        hdr_support: HdrCapability::None,
        wide_gamut: false,
        native_gamma: 2.2,
        max_luminance: Some(300),
        min_luminance: Some(0.5),
        color_gamut: ColorGamut::Srgb,
    }
}
