use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StorageInfo {
    pub name: String,
    pub device: String,
    pub storage_type: StorageType,
    pub size_gb: u64,
    pub model: Option<String>,
    pub removable: bool,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum StorageType {
    Nvme,
    Ssd,
    Hdd,
    Usb,
    MmcSd,
    Unknown,
}

pub fn detect() -> Vec<StorageInfo> {
    let mut devices = Vec::new();
    
    let block_dir = Path::new("/sys/block");
    if let Ok(entries) = fs::read_dir(block_dir) {
        for entry in entries.flatten() {
            let name = entry.file_name().to_string_lossy().to_string();
            
            // Skip loop, ram, dm devices
            if name.starts_with("loop") || 
               name.starts_with("ram") || 
               name.starts_with("dm-") ||
               name.starts_with("zram") {
                continue;
            }
            
            let device_path = entry.path();
            
            // Get size
            let size_sectors = fs::read_to_string(device_path.join("size"))
                .ok()
                .and_then(|s| s.trim().parse::<u64>().ok())
                .unwrap_or(0);
            let size_gb = size_sectors * 512 / 1024 / 1024 / 1024;
            
            if size_gb == 0 {
                continue;
            }
            
            // Determine type
            let rotational = fs::read_to_string(device_path.join("queue/rotational"))
                .ok()
                .and_then(|s| s.trim().parse::<u32>().ok())
                .unwrap_or(1);
            
            let removable = fs::read_to_string(device_path.join("removable"))
                .ok()
                .and_then(|s| s.trim().parse::<u32>().ok())
                .unwrap_or(0) == 1;
            
            let storage_type = if name.starts_with("nvme") {
                StorageType::Nvme
            } else if name.starts_with("mmcblk") {
                StorageType::MmcSd
            } else if removable {
                StorageType::Usb
            } else if rotational == 0 {
                StorageType::Ssd
            } else {
                StorageType::Hdd
            };
            
            // Get model
            let model = fs::read_to_string(device_path.join("device/model"))
                .ok()
                .map(|s| s.trim().to_string());
            
            devices.push(StorageInfo {
                name: name.clone(),
                device: format!("/dev/{}", name),
                storage_type,
                size_gb,
                model,
                removable,
            });
        }
    }
    
    devices.sort_by(|a, b| b.size_gb.cmp(&a.size_gb));
    devices
}
