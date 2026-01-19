use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkInfo {
    pub name: String,
    pub interface_type: NetworkType,
    pub mac_address: Option<String>,
    pub speed_mbps: Option<u32>,
    pub is_up: bool,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum NetworkType {
    Ethernet,
    Wifi,
    Loopback,
    Virtual,
    Unknown,
}

pub fn detect() -> Vec<NetworkInfo> {
    let mut interfaces = Vec::new();
    
    let net_path = Path::new("/sys/class/net");
    if let Ok(entries) = fs::read_dir(net_path) {
        for entry in entries.flatten() {
            let name = entry.file_name().to_string_lossy().to_string();
            let iface_path = entry.path();
            
            // Determine type
            let interface_type = if name == "lo" {
                NetworkType::Loopback
            } else if name.starts_with("wl") {
                NetworkType::Wifi
            } else if name.starts_with("en") || name.starts_with("eth") {
                NetworkType::Ethernet
            } else if name.starts_with("veth") || name.starts_with("br") || 
                      name.starts_with("docker") || name.starts_with("virbr") {
                NetworkType::Virtual
            } else {
                NetworkType::Unknown
            };
            
            // Get MAC address
            let mac_address = fs::read_to_string(iface_path.join("address"))
                .ok()
                .map(|s| s.trim().to_string())
                .filter(|s| s != "00:00:00:00:00:00");
            
            // Get speed (only for ethernet)
            let speed_mbps = if interface_type == NetworkType::Ethernet {
                fs::read_to_string(iface_path.join("speed"))
                    .ok()
                    .and_then(|s| s.trim().parse().ok())
            } else {
                None
            };
            
            // Check if up
            let operstate = fs::read_to_string(iface_path.join("operstate"))
                .ok()
                .map(|s| s.trim().to_string())
                .unwrap_or_default();
            let is_up = operstate == "up";
            
            // Skip loopback and virtual in main list
            if interface_type != NetworkType::Loopback && 
               interface_type != NetworkType::Virtual {
                interfaces.push(NetworkInfo {
                    name,
                    interface_type,
                    mac_address,
                    speed_mbps,
                    is_up,
                });
            }
        }
    }
    
    interfaces
}
