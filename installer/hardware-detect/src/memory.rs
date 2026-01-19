use serde::{Deserialize, Serialize};
use std::fs;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemoryInfo {
    pub total_gb: u32,
    pub memory_type: MemoryType,
    pub channels: Option<u32>,
    pub speed_mhz: Option<u32>,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum MemoryType {
    Ddr3,
    Ddr4,
    Ddr5,
    Lpddr4,
    Lpddr5,
    Unknown,
}

pub fn detect() -> MemoryInfo {
    let mut info = MemoryInfo {
        total_gb: 0,
        memory_type: MemoryType::Unknown,
        channels: None,
        speed_mhz: None,
    };
    
    // Read from /proc/meminfo
    if let Ok(content) = fs::read_to_string("/proc/meminfo") {
        for line in content.lines() {
            if line.starts_with("MemTotal:") {
                if let Some(kb_str) = line.split_whitespace().nth(1) {
                    if let Ok(kb) = kb_str.parse::<u64>() {
                        info.total_gb = (kb / 1024 / 1024) as u32;
                    }
                }
            }
        }
    }
    
    // Try dmidecode for detailed info (requires root)
    if let Ok(output) = std::process::Command::new("dmidecode")
        .args(["-t", "memory"])
        .output()
    {
        let text = String::from_utf8_lossy(&output.stdout);
        
        for line in text.lines() {
            let line = line.trim();
            
            if line.starts_with("Type:") {
                let type_str = line.split(':').nth(1).map(|s| s.trim()).unwrap_or("");
                info.memory_type = match type_str {
                    "DDR3" => MemoryType::Ddr3,
                    "DDR4" => MemoryType::Ddr4,
                    "DDR5" => MemoryType::Ddr5,
                    "LPDDR4" => MemoryType::Lpddr4,
                    "LPDDR5" => MemoryType::Lpddr5,
                    _ => MemoryType::Unknown,
                };
            } else if line.starts_with("Speed:") {
                if let Some(speed_str) = line.split(':').nth(1) {
                    info.speed_mhz = speed_str
                        .trim()
                        .split_whitespace()
                        .next()
                        .and_then(|s| s.parse().ok());
                }
            }
        }
    }
    
    info
}

pub fn get_recommendations(memory: &MemoryInfo) -> Vec<super::Recommendation> {
    let mut recs = Vec::new();
    
    if memory.total_gb < 8 {
        recs.push(super::Recommendation {
            category: super::RecommendationCategory::Performance,
            title: "Low Memory".to_string(),
            description: format!(
                "Only {} GB RAM detected. Consider 16+ GB for creative work.",
                memory.total_gb
            ),
            action: None,
            priority: super::Priority::High,
        });
    } else if memory.total_gb < 16 {
        recs.push(super::Recommendation {
            category: super::RecommendationCategory::Performance,
            title: "Limited Memory".to_string(),
            description: format!(
                "{} GB RAM. 16+ GB recommended for video/3D work.",
                memory.total_gb
            ),
            action: None,
            priority: super::Priority::Medium,
        });
    }
    
    if memory.total_gb >= 32 {
        recs.push(super::Recommendation {
            category: super::RecommendationCategory::Configuration,
            title: "Ample Memory".to_string(),
            description: "Consider reducing swap usage for better performance.".to_string(),
            action: Some("sudo sysctl vm.swappiness=10".to_string()),
            priority: super::Priority::Low,
        });
    }
    
    recs
}
