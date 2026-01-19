use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IccProfile {
    pub name: String,
    pub description: String,
    pub path: String,
    pub created: String,
}

impl IccProfile {
    pub fn create(
        display_name: &str,
        brightness: f32,
        contrast: f32,
        gamma: f32,
        white_point: u32,
    ) -> Self {
        let name = format!(
            "{}_{}K_g{:.1}",
            display_name.replace("-", "_"),
            white_point,
            gamma
        );

        let timestamp = chrono_lite_timestamp();

        Self {
            name: name.clone(),
            description: format!(
                "Calibrated profile: brightness {:.0}%, contrast {:.0}%, gamma {:.1}, white point {}K",
                brightness, contrast, gamma, white_point
            ),
            path: format!(
                "{}/.local/share/icc/{}.icc",
                std::env::var("HOME").unwrap_or_default(),
                name
            ),
            created: timestamp,
        }
    }

    pub fn save(&self) -> std::io::Result<()> {
        let path = PathBuf::from(&self.path);
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent)?;
        }

        // In real implementation, would create actual ICC profile using lcms2
        // For now, just create a placeholder file
        let metadata = serde_json::to_string_pretty(self)?;
        std::fs::write(&self.path, metadata)?;
        
        Ok(())
    }

    pub fn load(path: &str) -> std::io::Result<Self> {
        let content = std::fs::read_to_string(path)?;
        let profile: Self = serde_json::from_str(&content)?;
        Ok(profile)
    }

    pub fn list_system_profiles() -> Vec<PathBuf> {
        let mut profiles = Vec::new();

        // System profiles
        if let Ok(entries) = std::fs::read_dir("/usr/share/color/icc") {
            for entry in entries.flatten() {
                let path = entry.path();
                if path.extension().map(|e| e == "icc").unwrap_or(false) {
                    profiles.push(path);
                }
            }
        }

        // User profiles
        if let Ok(home) = std::env::var("HOME") {
            let user_path = PathBuf::from(home).join(".local/share/icc");
            if let Ok(entries) = std::fs::read_dir(user_path) {
                for entry in entries.flatten() {
                    let path = entry.path();
                    if path.extension().map(|e| e == "icc").unwrap_or(false) {
                        profiles.push(path);
                    }
                }
            }
        }

        profiles
    }
}

fn chrono_lite_timestamp() -> String {
    use std::time::{SystemTime, UNIX_EPOCH};
    
    let duration = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default();
    
    let secs = duration.as_secs();
    let days = secs / 86400;
    let years = 1970 + days / 365;
    let remaining_days = days % 365;
    let months = remaining_days / 30 + 1;
    let day = remaining_days % 30 + 1;
    
    format!("{:04}-{:02}-{:02}", years, months, day)
}

#[derive(Debug, Clone)]
pub struct ColorProfile {
    pub red: ColorChannel,
    pub green: ColorChannel,
    pub blue: ColorChannel,
    pub white_point: WhitePoint,
    pub gamma: f32,
}

#[derive(Debug, Clone)]
pub struct ColorChannel {
    pub x: f32,
    pub y: f32,
}

#[derive(Debug, Clone)]
pub struct WhitePoint {
    pub temperature: u32,
    pub x: f32,
    pub y: f32,
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

    pub fn from_temperature(kelvin: u32) -> Self {
        // Approximate calculation
        let temp = kelvin as f32;
        let x = if temp <= 4000.0 {
            0.27475 + 0.99910e-04 * temp + 0.86070e-08 * temp * temp - 0.90911e-12 * temp * temp * temp
        } else {
            0.24039 + 0.22682e-03 * temp - 0.15614e-06 * temp * temp + 0.31775e-10 * temp * temp * temp
        };
        
        let y = -3.0 * x * x + 2.87 * x - 0.275;

        Self {
            temperature: kelvin,
            x,
            y,
        }
    }
}

impl Default for ColorProfile {
    fn default() -> Self {
        // sRGB primaries
        Self {
            red: ColorChannel { x: 0.64, y: 0.33 },
            green: ColorChannel { x: 0.30, y: 0.60 },
            blue: ColorChannel { x: 0.15, y: 0.06 },
            white_point: WhitePoint::d65(),
            gamma: 2.2,
        }
    }
}
