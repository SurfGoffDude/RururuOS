use crate::{Result, WorkflowError, WorkflowProfile, WorkflowType};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkflowConfig {
    pub version: u32,
    pub active_workflow: WorkflowType,
    pub profiles: HashMap<String, WorkflowProfile>,
    pub auto_switch: AutoSwitchConfig,
    pub package_manager: PackageManager,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AutoSwitchConfig {
    pub enabled: bool,
    pub rules: Vec<AutoSwitchRule>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AutoSwitchRule {
    pub app_pattern: String,
    pub workflow: WorkflowType,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum PackageManager {
    Pacman,
    Apt,
    Dnf,
    Zypper,
    Flatpak,
}

impl Default for WorkflowConfig {
    fn default() -> Self {
        let mut profiles = HashMap::new();

        for workflow_type in WorkflowType::all() {
            let profile = WorkflowProfile::get_profile(*workflow_type);
            profiles.insert(workflow_type.name().to_string(), profile);
        }

        Self {
            version: 1,
            active_workflow: WorkflowType::General,
            profiles,
            auto_switch: AutoSwitchConfig {
                enabled: true,
                rules: vec![
                    AutoSwitchRule {
                        app_pattern: "resolve|kdenlive".to_string(),
                        workflow: WorkflowType::VideoEditor,
                    },
                    AutoSwitchRule {
                        app_pattern: "blender|freecad".to_string(),
                        workflow: WorkflowType::ThreeDArtist,
                    },
                    AutoSwitchRule {
                        app_pattern: "krita|gimp|inkscape".to_string(),
                        workflow: WorkflowType::TwoDDesigner,
                    },
                    AutoSwitchRule {
                        app_pattern: "ardour|bitwig|audacity".to_string(),
                        workflow: WorkflowType::AudioProducer,
                    },
                    AutoSwitchRule {
                        app_pattern: "darktable|rawtherapee|digikam".to_string(),
                        workflow: WorkflowType::Photographer,
                    },
                ],
            },
            package_manager: detect_package_manager(),
        }
    }
}

impl WorkflowConfig {
    pub fn load() -> Result<Self> {
        let config_path = Self::config_path();

        if config_path.exists() {
            let content = std::fs::read_to_string(&config_path)?;
            toml::from_str(&content).map_err(|e| WorkflowError::Config(e.to_string()))
        } else {
            Ok(Self::default())
        }
    }

    pub fn save(&self) -> Result<()> {
        let config_path = Self::config_path();

        if let Some(parent) = config_path.parent() {
            std::fs::create_dir_all(parent)?;
        }

        let content =
            toml::to_string_pretty(self).map_err(|e| WorkflowError::Config(e.to_string()))?;

        std::fs::write(config_path, content)?;
        Ok(())
    }

    fn config_path() -> PathBuf {
        dirs::config_dir()
            .unwrap_or_else(|| PathBuf::from("."))
            .join("rururu")
            .join("workflows.toml")
    }

    pub fn get_active_profile(&self) -> Option<&WorkflowProfile> {
        self.profiles.get(self.active_workflow.name())
    }

    pub fn set_active_workflow(&mut self, workflow: WorkflowType) {
        self.active_workflow = workflow;
    }

    pub fn get_profile(&self, name: &str) -> Option<&WorkflowProfile> {
        self.profiles.get(name)
    }
}

fn detect_package_manager() -> PackageManager {
    if std::path::Path::new("/usr/bin/pacman").exists() {
        PackageManager::Pacman
    } else if std::path::Path::new("/usr/bin/apt").exists() {
        PackageManager::Apt
    } else if std::path::Path::new("/usr/bin/dnf").exists() {
        PackageManager::Dnf
    } else if std::path::Path::new("/usr/bin/zypper").exists() {
        PackageManager::Zypper
    } else {
        PackageManager::Flatpak
    }
}
