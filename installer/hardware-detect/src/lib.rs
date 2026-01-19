pub mod cpu;
pub mod gpu;
pub mod storage;
pub mod memory;
pub mod display;
pub mod audio;
pub mod network;

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HardwareInfo {
    pub cpu: cpu::CpuInfo,
    pub gpu: Vec<gpu::GpuInfo>,
    pub memory: memory::MemoryInfo,
    pub storage: Vec<storage::StorageInfo>,
    pub displays: Vec<display::DisplayInfo>,
    pub audio: audio::AudioInfo,
    pub network: Vec<network::NetworkInfo>,
    pub recommendations: Vec<Recommendation>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Recommendation {
    pub category: RecommendationCategory,
    pub title: String,
    pub description: String,
    pub action: Option<String>,
    pub priority: Priority,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum RecommendationCategory {
    Driver,
    Performance,
    Workflow,
    Package,
    Configuration,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum Priority {
    Critical,
    High,
    Medium,
    Low,
}

pub fn detect_all() -> HardwareInfo {
    let cpu = cpu::detect();
    let gpu = gpu::detect();
    let memory = memory::detect();
    let storage = storage::detect();
    let displays = display::detect();
    let audio = audio::detect();
    let network = network::detect();
    
    let mut recommendations = Vec::new();
    
    // GPU recommendations
    for g in &gpu {
        recommendations.extend(gpu::get_recommendations(g));
    }
    
    // Memory recommendations
    recommendations.extend(memory::get_recommendations(&memory));
    
    // CPU recommendations
    recommendations.extend(cpu::get_recommendations(&cpu));
    
    // Workflow recommendations based on hardware
    recommendations.extend(suggest_workflows(&cpu, &gpu, &memory));
    
    HardwareInfo {
        cpu,
        gpu,
        memory,
        storage,
        displays,
        audio,
        network,
        recommendations,
    }
}

fn suggest_workflows(
    cpu: &cpu::CpuInfo,
    gpus: &[gpu::GpuInfo],
    memory: &memory::MemoryInfo,
) -> Vec<Recommendation> {
    let mut recs = Vec::new();
    
    let has_powerful_gpu = gpus.iter().any(|g| {
        g.vram_mb.unwrap_or(0) >= 8192 || 
        g.vendor == gpu::GpuVendor::Nvidia
    });
    
    let high_memory = memory.total_gb >= 32;
    let many_cores = cpu.cores >= 8;
    
    if has_powerful_gpu && high_memory {
        recs.push(Recommendation {
            category: RecommendationCategory::Workflow,
            title: "3D/Video Production Ready".to_string(),
            description: "Your hardware is suitable for 3D rendering and video editing.".to_string(),
            action: Some("rururu-workflow activate video".to_string()),
            priority: Priority::Medium,
        });
    }
    
    if many_cores && high_memory {
        recs.push(Recommendation {
            category: RecommendationCategory::Workflow,
            title: "Audio Production Ready".to_string(),
            description: "Your CPU is suitable for real-time audio processing.".to_string(),
            action: Some("rururu-workflow activate audio".to_string()),
            priority: Priority::Medium,
        });
    }
    
    if has_powerful_gpu {
        recs.push(Recommendation {
            category: RecommendationCategory::Workflow,
            title: "GPU Compute Available".to_string(),
            description: "GPU acceleration available for rendering and compute tasks.".to_string(),
            action: None,
            priority: Priority::Low,
        });
    }
    
    recs
}

pub fn generate_report(info: &HardwareInfo) -> String {
    let mut report = String::new();
    
    report.push_str("# RururuOS Hardware Detection Report\n\n");
    
    report.push_str("## CPU\n");
    report.push_str(&format!("- Model: {}\n", info.cpu.model));
    report.push_str(&format!("- Cores: {} (Threads: {})\n", info.cpu.cores, info.cpu.threads));
    report.push_str(&format!("- Architecture: {:?}\n\n", info.cpu.arch));
    
    report.push_str("## GPU\n");
    for gpu in &info.gpu {
        report.push_str(&format!("- {} ({:?})\n", gpu.name, gpu.vendor));
        if let Some(vram) = gpu.vram_mb {
            report.push_str(&format!("  VRAM: {} MB\n", vram));
        }
    }
    report.push('\n');
    
    report.push_str("## Memory\n");
    report.push_str(&format!("- Total: {} GB\n", info.memory.total_gb));
    report.push_str(&format!("- Type: {:?}\n\n", info.memory.memory_type));
    
    report.push_str("## Storage\n");
    for disk in &info.storage {
        report.push_str(&format!("- {} ({:?}): {} GB\n", 
            disk.name, disk.storage_type, disk.size_gb));
    }
    report.push('\n');
    
    if !info.recommendations.is_empty() {
        report.push_str("## Recommendations\n");
        for rec in &info.recommendations {
            let priority = match rec.priority {
                Priority::Critical => "🔴",
                Priority::High => "🟠",
                Priority::Medium => "🟡",
                Priority::Low => "🟢",
            };
            report.push_str(&format!("{} **{}**: {}\n", priority, rec.title, rec.description));
            if let Some(action) = &rec.action {
                report.push_str(&format!("   Action: `{}`\n", action));
            }
        }
    }
    
    report
}
