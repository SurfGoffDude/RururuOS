use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ColorConfig {
    pub version: u32,
    pub global: GlobalColorSettings,
    pub monitors: HashMap<String, MonitorColorConfig>,
    pub ocio: Option<OcioConfig>,
    pub workflows: HashMap<String, WorkflowColorConfig>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GlobalColorSettings {
    pub enabled: bool,
    pub default_profile: String,
    pub rendering_intent: RenderingIntent,
    pub black_point_compensation: bool,
    pub gamut_warning: bool,
    pub soft_proofing: bool,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum RenderingIntent {
    Perceptual,
    RelativeColorimetric,
    Saturation,
    AbsoluteColorimetric,
}

impl Default for RenderingIntent {
    fn default() -> Self {
        Self::Perceptual
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MonitorColorConfig {
    pub edid_name: String,
    pub icc_profile: Option<PathBuf>,
    pub calibration_date: Option<String>,
    pub brightness: f32,
    pub contrast: f32,
    pub gamma: f32,
    pub white_point: u32,
    pub hdr_enabled: bool,
    pub hdr_peak_luminance: Option<u32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OcioConfig {
    pub config_path: PathBuf,
    pub working_space: String,
    pub display_space: String,
    pub view_transform: String,
    pub look: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkflowColorConfig {
    pub name: String,
    pub working_space: String,
    pub ocio_config: Option<PathBuf>,
    pub default_intent: RenderingIntent,
    pub soft_proof_profile: Option<PathBuf>,
}

impl Default for ColorConfig {
    fn default() -> Self {
        Self {
            version: 1,
            global: GlobalColorSettings {
                enabled: true,
                default_profile: "sRGB".to_string(),
                rendering_intent: RenderingIntent::Perceptual,
                black_point_compensation: true,
                gamut_warning: false,
                soft_proofing: false,
            },
            monitors: HashMap::new(),
            ocio: None,
            workflows: default_workflows(),
        }
    }
}

fn default_workflows() -> HashMap<String, WorkflowColorConfig> {
    let mut workflows = HashMap::new();

    workflows.insert(
        "photography".to_string(),
        WorkflowColorConfig {
            name: "Photography".to_string(),
            working_space: "ProPhoto RGB".to_string(),
            ocio_config: None,
            default_intent: RenderingIntent::Perceptual,
            soft_proof_profile: None,
        },
    );

    workflows.insert(
        "video".to_string(),
        WorkflowColorConfig {
            name: "Video Editing".to_string(),
            working_space: "Rec.709".to_string(),
            ocio_config: Some(PathBuf::from("/usr/share/ocio/aces_1.2/config.ocio")),
            default_intent: RenderingIntent::RelativeColorimetric,
            soft_proof_profile: None,
        },
    );

    workflows.insert(
        "vfx".to_string(),
        WorkflowColorConfig {
            name: "VFX / 3D".to_string(),
            working_space: "ACEScg".to_string(),
            ocio_config: Some(PathBuf::from("/usr/share/ocio/aces_1.2/config.ocio")),
            default_intent: RenderingIntent::RelativeColorimetric,
            soft_proof_profile: None,
        },
    );

    workflows.insert(
        "print".to_string(),
        WorkflowColorConfig {
            name: "Print Design".to_string(),
            working_space: "Adobe RGB".to_string(),
            ocio_config: None,
            default_intent: RenderingIntent::RelativeColorimetric,
            soft_proof_profile: Some(PathBuf::from("/usr/share/color/icc/Fogra39.icc")),
        },
    );

    workflows.insert(
        "web".to_string(),
        WorkflowColorConfig {
            name: "Web Design".to_string(),
            working_space: "sRGB".to_string(),
            ocio_config: None,
            default_intent: RenderingIntent::Perceptual,
            soft_proof_profile: None,
        },
    );

    workflows
}

impl ColorConfig {
    pub fn load() -> crate::Result<Self> {
        let config_path = Self::config_path();

        if config_path.exists() {
            let content = std::fs::read_to_string(&config_path)?;
            toml::from_str(&content).map_err(|e| crate::ColorError::Config(e.to_string()))
        } else {
            Ok(Self::default())
        }
    }

    pub fn save(&self) -> crate::Result<()> {
        let config_path = Self::config_path();

        if let Some(parent) = config_path.parent() {
            std::fs::create_dir_all(parent)?;
        }

        let content =
            toml::to_string_pretty(self).map_err(|e| crate::ColorError::Config(e.to_string()))?;

        std::fs::write(config_path, content)?;
        Ok(())
    }

    fn config_path() -> PathBuf {
        dirs::config_dir()
            .unwrap_or_else(|| PathBuf::from("."))
            .join("rururu")
            .join("color.toml")
    }
}
