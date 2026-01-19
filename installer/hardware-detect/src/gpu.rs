use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;
use std::process::Command;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GpuInfo {
    pub name: String,
    pub vendor: GpuVendor,
    pub pci_id: Option<String>,
    pub driver: Option<String>,
    pub vram_mb: Option<u32>,
    pub features: GpuFeatures,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum GpuVendor {
    Nvidia,
    Amd,
    Intel,
    Apple,
    VirtIO,
    Unknown,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GpuFeatures {
    pub vulkan: bool,
    pub opencl: bool,
    pub cuda: bool,
    pub rocm: bool,
    pub vaapi: bool,
    pub vdpau: bool,
}

pub fn detect() -> Vec<GpuInfo> {
    let mut gpus = Vec::new();
    
    // Parse lspci output
    if let Ok(output) = Command::new("lspci").args(["-nnk"]).output() {
        let text = String::from_utf8_lossy(&output.stdout);
        let mut current_gpu: Option<GpuInfo> = None;
        
        for line in text.lines() {
            if line.contains("VGA") || line.contains("3D") || line.contains("Display") {
                if let Some(gpu) = current_gpu.take() {
                    gpus.push(gpu);
                }
                
                let vendor = if line.contains("NVIDIA") {
                    GpuVendor::Nvidia
                } else if line.contains("AMD") || line.contains("ATI") {
                    GpuVendor::Amd
                } else if line.contains("Intel") {
                    GpuVendor::Intel
                } else if line.contains("virtio") {
                    GpuVendor::VirtIO
                } else {
                    GpuVendor::Unknown
                };
                
                let pci_id = line.split('[').nth(1)
                    .and_then(|s| s.split(']').next())
                    .map(String::from);
                
                let name = line.split(':').nth(2)
                    .map(|s| s.trim().to_string())
                    .unwrap_or_else(|| "Unknown GPU".to_string());
                
                current_gpu = Some(GpuInfo {
                    name,
                    vendor,
                    pci_id,
                    driver: None,
                    vram_mb: None,
                    features: GpuFeatures::default(),
                });
            } else if line.contains("Kernel driver") {
                if let Some(ref mut gpu) = current_gpu {
                    gpu.driver = line.split(':').nth(1).map(|s| s.trim().to_string());
                }
            }
        }
        
        if let Some(gpu) = current_gpu {
            gpus.push(gpu);
        }
    }
    
    // Detect VRAM for NVIDIA
    for gpu in &mut gpus {
        if gpu.vendor == GpuVendor::Nvidia {
            if let Ok(output) = Command::new("nvidia-smi")
                .args(["--query-gpu=memory.total", "--format=csv,noheader,nounits"])
                .output()
            {
                if output.status.success() {
                    if let Ok(vram) = String::from_utf8_lossy(&output.stdout).trim().parse() {
                        gpu.vram_mb = Some(vram);
                    }
                }
            }
        }
    }
    
    // Detect features
    for gpu in &mut gpus {
        gpu.features = detect_features(gpu);
    }
    
    gpus
}

fn detect_features(gpu: &GpuInfo) -> GpuFeatures {
    let mut features = GpuFeatures::default();
    
    // Vulkan
    if Path::new("/usr/share/vulkan/icd.d").exists() {
        if let Ok(entries) = fs::read_dir("/usr/share/vulkan/icd.d") {
            features.vulkan = entries.count() > 0;
        }
    }
    
    // CUDA (NVIDIA)
    if gpu.vendor == GpuVendor::Nvidia {
        features.cuda = Path::new("/usr/lib/libcuda.so").exists() ||
                       Path::new("/usr/lib64/libcuda.so").exists();
    }
    
    // ROCm (AMD)
    if gpu.vendor == GpuVendor::Amd {
        features.rocm = Path::new("/opt/rocm").exists();
    }
    
    // VA-API
    features.vaapi = Path::new("/usr/lib/dri").exists();
    
    // VDPAU
    features.vdpau = Path::new("/usr/lib/vdpau").exists();
    
    features
}

pub fn get_recommendations(gpu: &GpuInfo) -> Vec<super::Recommendation> {
    let mut recs = Vec::new();
    
    match gpu.vendor {
        GpuVendor::Nvidia => {
            if gpu.driver.as_deref() != Some("nvidia") {
                recs.push(super::Recommendation {
                    category: super::RecommendationCategory::Driver,
                    title: "NVIDIA Proprietary Driver".to_string(),
                    description: "Install NVIDIA proprietary driver for best performance.".to_string(),
                    action: Some("sudo pacman -S nvidia nvidia-utils".to_string()),
                    priority: super::Priority::High,
                });
            }
            
            if !gpu.features.cuda {
                recs.push(super::Recommendation {
                    category: super::RecommendationCategory::Package,
                    title: "CUDA Support".to_string(),
                    description: "Install CUDA for GPU acceleration in creative apps.".to_string(),
                    action: Some("sudo pacman -S cuda".to_string()),
                    priority: super::Priority::Medium,
                });
            }
        }
        GpuVendor::Amd => {
            if !gpu.features.rocm {
                recs.push(super::Recommendation {
                    category: super::RecommendationCategory::Package,
                    title: "ROCm Support".to_string(),
                    description: "Install ROCm for GPU compute on AMD.".to_string(),
                    action: Some("sudo pacman -S rocm-hip-sdk".to_string()),
                    priority: super::Priority::Medium,
                });
            }
        }
        GpuVendor::Intel => {
            recs.push(super::Recommendation {
                category: super::RecommendationCategory::Package,
                title: "Intel Media Driver".to_string(),
                description: "Ensure Intel media driver is installed for hardware video.".to_string(),
                action: Some("sudo pacman -S intel-media-driver".to_string()),
                priority: super::Priority::Medium,
            });
        }
        _ => {}
    }
    
    if !gpu.features.vulkan {
        recs.push(super::Recommendation {
            category: super::RecommendationCategory::Driver,
            title: "Vulkan Support Missing".to_string(),
            description: "Vulkan is not detected. Some apps may not work correctly.".to_string(),
            action: Some("sudo pacman -S vulkan-icd-loader".to_string()),
            priority: super::Priority::High,
        });
    }
    
    recs
}
