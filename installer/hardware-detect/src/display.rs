use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DisplayInfo {
    pub name: String,
    pub connector: String,
    pub resolution: Option<(u32, u32)>,
    pub refresh_rate: Option<f32>,
    pub hdr_capable: bool,
    pub wide_gamut: bool,
}

pub fn detect() -> Vec<DisplayInfo> {
    let mut displays = Vec::new();
    
    let drm_path = Path::new("/sys/class/drm");
    if let Ok(entries) = fs::read_dir(drm_path) {
        for entry in entries.flatten() {
            let name = entry.file_name().to_string_lossy().to_string();
            
            // Only process card*-* entries (connectors)
            if !name.starts_with("card") || !name.contains('-') {
                continue;
            }
            
            let connector_path = entry.path();
            
            // Check if connected
            let status = fs::read_to_string(connector_path.join("status"))
                .ok()
                .map(|s| s.trim().to_string())
                .unwrap_or_default();
            
            if status != "connected" {
                continue;
            }
            
            // Parse connector type
            let connector = name.split('-').nth(1).unwrap_or("Unknown").to_string();
            
            // Get modes
            let (resolution, refresh_rate) = if let Ok(modes) = 
                fs::read_to_string(connector_path.join("modes")) 
            {
                parse_mode(modes.lines().next().unwrap_or(""))
            } else {
                (None, None)
            };
            
            // Check EDID for HDR/wide gamut
            let (hdr_capable, wide_gamut) = if let Ok(edid) = 
                fs::read(connector_path.join("edid")) 
            {
                parse_edid_capabilities(&edid)
            } else {
                (false, false)
            };
            
            displays.push(DisplayInfo {
                name,
                connector,
                resolution,
                refresh_rate,
                hdr_capable,
                wide_gamut,
            });
        }
    }
    
    displays
}

fn parse_mode(mode: &str) -> (Option<(u32, u32)>, Option<f32>) {
    // Mode format: 1920x1080 or 1920x1080@60
    let parts: Vec<&str> = mode.split('@').collect();
    
    let resolution = parts.first().and_then(|res| {
        let dims: Vec<&str> = res.split('x').collect();
        if dims.len() == 2 {
            let w = dims[0].parse().ok()?;
            let h = dims[1].parse().ok()?;
            Some((w, h))
        } else {
            None
        }
    });
    
    let refresh = parts.get(1).and_then(|r| r.parse().ok());
    
    (resolution, refresh)
}

fn parse_edid_capabilities(edid: &[u8]) -> (bool, bool) {
    // Simplified EDID parsing
    // Real implementation would parse extension blocks for HDR metadata
    
    let hdr = edid.len() > 128; // Has extension blocks (might contain HDR info)
    let wide_gamut = edid.len() > 128; // Placeholder
    
    (hdr, wide_gamut)
}
