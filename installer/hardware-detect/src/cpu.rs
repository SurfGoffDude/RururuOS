use serde::{Deserialize, Serialize};
use std::fs;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CpuInfo {
    pub model: String,
    pub vendor: CpuVendor,
    pub arch: CpuArch,
    pub cores: u32,
    pub threads: u32,
    pub freq_mhz: Option<u32>,
    pub features: Vec<String>,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum CpuVendor {
    Intel,
    Amd,
    Arm,
    Apple,
    Unknown,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum CpuArch {
    X86_64,
    Aarch64,
    Unknown,
}

pub fn detect() -> CpuInfo {
    let mut info = CpuInfo {
        model: "Unknown".to_string(),
        vendor: CpuVendor::Unknown,
        arch: detect_arch(),
        cores: 1,
        threads: 1,
        freq_mhz: None,
        features: Vec::new(),
    };
    
    if let Ok(cpuinfo) = fs::read_to_string("/proc/cpuinfo") {
        for line in cpuinfo.lines() {
            if line.starts_with("model name") {
                if let Some(value) = line.split(':').nth(1) {
                    info.model = value.trim().to_string();
                }
            } else if line.starts_with("vendor_id") {
                if let Some(value) = line.split(':').nth(1) {
                    info.vendor = match value.trim() {
                        "GenuineIntel" => CpuVendor::Intel,
                        "AuthenticAMD" => CpuVendor::Amd,
                        _ => CpuVendor::Unknown,
                    };
                }
            } else if line.starts_with("cpu cores") {
                if let Some(value) = line.split(':').nth(1) {
                    info.cores = value.trim().parse().unwrap_or(1);
                }
            } else if line.starts_with("siblings") {
                if let Some(value) = line.split(':').nth(1) {
                    info.threads = value.trim().parse().unwrap_or(1);
                }
            } else if line.starts_with("cpu MHz") {
                if let Some(value) = line.split(':').nth(1) {
                    info.freq_mhz = value.trim().parse::<f64>().ok().map(|f| f as u32);
                }
            } else if line.starts_with("flags") {
                if let Some(value) = line.split(':').nth(1) {
                    info.features = value.split_whitespace()
                        .map(String::from)
                        .collect();
                }
            }
        }
    }
    
    // ARM detection
    if info.arch == CpuArch::Aarch64 {
        if let Ok(content) = fs::read_to_string("/sys/firmware/devicetree/base/model") {
            info.model = content.trim_end_matches('\0').to_string();
            if info.model.contains("Apple") {
                info.vendor = CpuVendor::Apple;
            } else {
                info.vendor = CpuVendor::Arm;
            }
        }
        
        // Count cores from sysfs
        if let Ok(entries) = fs::read_dir("/sys/devices/system/cpu") {
            info.cores = entries
                .filter_map(|e| e.ok())
                .filter(|e| {
                    e.file_name().to_string_lossy().starts_with("cpu") &&
                    e.file_name().to_string_lossy().chars().nth(3).map(|c| c.is_ascii_digit()).unwrap_or(false)
                })
                .count() as u32;
            info.threads = info.cores;
        }
    }
    
    info
}

fn detect_arch() -> CpuArch {
    #[cfg(target_arch = "x86_64")]
    return CpuArch::X86_64;
    
    #[cfg(target_arch = "aarch64")]
    return CpuArch::Aarch64;
    
    #[cfg(not(any(target_arch = "x86_64", target_arch = "aarch64")))]
    return CpuArch::Unknown;
}

pub fn get_recommendations(cpu: &CpuInfo) -> Vec<super::Recommendation> {
    let mut recs = Vec::new();
    
    if cpu.cores >= 8 {
        recs.push(super::Recommendation {
            category: super::RecommendationCategory::Performance,
            title: "High Core Count".to_string(),
            description: format!("{} cores detected. Parallel processing will be efficient.", cpu.cores),
            action: None,
            priority: super::Priority::Low,
        });
    }
    
    // Check for virtualization
    if cpu.features.contains(&"vmx".to_string()) || cpu.features.contains(&"svm".to_string()) {
        recs.push(super::Recommendation {
            category: super::RecommendationCategory::Configuration,
            title: "Virtualization Available".to_string(),
            description: "Hardware virtualization is supported.".to_string(),
            action: None,
            priority: super::Priority::Low,
        });
    }
    
    recs
}
