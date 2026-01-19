use crate::{ColorError, Result};
use std::path::{Path, PathBuf};

#[derive(Debug, Clone)]
pub struct OcioManager {
    config_path: Option<PathBuf>,
    config: Option<OcioConfig>,
}

#[derive(Debug, Clone)]
pub struct OcioConfig {
    pub path: PathBuf,
    pub description: String,
    pub color_spaces: Vec<OcioColorSpace>,
    pub displays: Vec<OcioDisplay>,
    pub views: Vec<OcioView>,
    pub looks: Vec<OcioLook>,
    pub roles: OcioRoles,
}

#[derive(Debug, Clone)]
pub struct OcioColorSpace {
    pub name: String,
    pub family: String,
    pub description: String,
    pub is_data: bool,
}

#[derive(Debug, Clone)]
pub struct OcioDisplay {
    pub name: String,
    pub views: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct OcioView {
    pub name: String,
    pub display: String,
    pub color_space: String,
}

#[derive(Debug, Clone)]
pub struct OcioLook {
    pub name: String,
    pub process_space: String,
    pub description: Option<String>,
}

#[derive(Debug, Clone, Default)]
pub struct OcioRoles {
    pub default: Option<String>,
    pub reference: Option<String>,
    pub data: Option<String>,
    pub compositing_log: Option<String>,
    pub color_timing: Option<String>,
    pub color_picking: Option<String>,
    pub matte_paint: Option<String>,
    pub texture_paint: Option<String>,
    pub scene_linear: Option<String>,
}

impl OcioManager {
    pub fn new() -> Self {
        Self {
            config_path: None,
            config: None,
        }
    }

    pub fn load_config(&mut self, path: &Path) -> Result<()> {
        if !path.exists() {
            return Err(ColorError::OcioError(format!(
                "Config not found: {}",
                path.display()
            )));
        }

        // Parse OCIO config file
        let content = std::fs::read_to_string(path)?;
        let config = Self::parse_config(&content, path)?;

        self.config_path = Some(path.to_path_buf());
        self.config = Some(config);

        // Set environment variable for applications
        std::env::set_var("OCIO", path);

        Ok(())
    }

    fn parse_config(content: &str, path: &Path) -> Result<OcioConfig> {
        // Simplified OCIO config parsing
        // Real implementation would use ocio-rs or similar

        let mut color_spaces = Vec::new();
        let mut displays = Vec::new();
        let mut views = Vec::new();
        let mut looks = Vec::new();
        let mut roles = OcioRoles::default();
        let mut description = String::new();

        let mut current_section = "";
        let mut current_colorspace: Option<OcioColorSpace> = None;

        for line in content.lines() {
            let line = line.trim();

            if line.starts_with("description:") {
                description = line.trim_start_matches("description:").trim().to_string();
            } else if line.starts_with("colorspaces:") {
                current_section = "colorspaces";
            } else if line.starts_with("displays:") {
                current_section = "displays";
            } else if line.starts_with("looks:") {
                current_section = "looks";
            } else if line.starts_with("roles:") {
                current_section = "roles";
            } else if current_section == "colorspaces" && line.starts_with("- !<ColorSpace>") {
                if let Some(cs) = current_colorspace.take() {
                    color_spaces.push(cs);
                }
                current_colorspace = Some(OcioColorSpace {
                    name: String::new(),
                    family: String::new(),
                    description: String::new(),
                    is_data: false,
                });
            } else if let Some(ref mut cs) = current_colorspace {
                if line.starts_with("name:") {
                    cs.name = line.trim_start_matches("name:").trim().to_string();
                } else if line.starts_with("family:") {
                    cs.family = line.trim_start_matches("family:").trim().to_string();
                } else if line.starts_with("description:") {
                    cs.description = line.trim_start_matches("description:").trim().to_string();
                } else if line.starts_with("isdata:") {
                    cs.is_data = line.contains("true");
                }
            } else if current_section == "roles" {
                if line.starts_with("default:") {
                    roles.default = Some(line.trim_start_matches("default:").trim().to_string());
                } else if line.starts_with("reference:") {
                    roles.reference =
                        Some(line.trim_start_matches("reference:").trim().to_string());
                } else if line.starts_with("scene_linear:") {
                    roles.scene_linear =
                        Some(line.trim_start_matches("scene_linear:").trim().to_string());
                }
            }
        }

        if let Some(cs) = current_colorspace {
            color_spaces.push(cs);
        }

        Ok(OcioConfig {
            path: path.to_path_buf(),
            description,
            color_spaces,
            displays,
            views,
            looks,
            roles,
        })
    }

    pub fn get_config(&self) -> Option<&OcioConfig> {
        self.config.as_ref()
    }

    pub fn list_color_spaces(&self) -> Vec<&str> {
        self.config
            .as_ref()
            .map(|c| c.color_spaces.iter().map(|cs| cs.name.as_str()).collect())
            .unwrap_or_default()
    }

    pub fn list_displays(&self) -> Vec<&str> {
        self.config
            .as_ref()
            .map(|c| c.displays.iter().map(|d| d.name.as_str()).collect())
            .unwrap_or_default()
    }

    pub fn get_scene_linear(&self) -> Option<&str> {
        self.config
            .as_ref()
            .and_then(|c| c.roles.scene_linear.as_deref())
    }

    pub fn unload_config(&mut self) {
        self.config = None;
        self.config_path = None;
        std::env::remove_var("OCIO");
    }
}

impl Default for OcioManager {
    fn default() -> Self {
        Self::new()
    }
}

pub fn find_ocio_configs() -> Vec<PathBuf> {
    let mut configs = Vec::new();

    let search_paths = ["/usr/share/ocio", "/usr/local/share/ocio", "/opt/ocio"];

    for base in search_paths {
        let base_path = Path::new(base);
        if base_path.exists() {
            if let Ok(entries) = std::fs::read_dir(base_path) {
                for entry in entries.flatten() {
                    let config_path = entry.path().join("config.ocio");
                    if config_path.exists() {
                        configs.push(config_path);
                    }
                }
            }
        }
    }

    // Check user directory
    if let Some(data_dir) = dirs::data_dir() {
        let user_ocio = data_dir.join("ocio");
        if user_ocio.exists() {
            if let Ok(entries) = std::fs::read_dir(user_ocio) {
                for entry in entries.flatten() {
                    let config_path = entry.path().join("config.ocio");
                    if config_path.exists() {
                        configs.push(config_path);
                    }
                }
            }
        }
    }

    configs
}

#[derive(Debug, Clone)]
pub struct OcioPreset {
    pub name: String,
    pub description: String,
    pub config_path: PathBuf,
    pub workflow: String,
}

pub fn builtin_presets() -> Vec<OcioPreset> {
    vec![
        OcioPreset {
            name: "ACES 1.2".to_string(),
            description: "Academy Color Encoding System - Industry standard for VFX/Film"
                .to_string(),
            config_path: PathBuf::from("/usr/share/ocio/aces_1.2/config.ocio"),
            workflow: "vfx".to_string(),
        },
        OcioPreset {
            name: "ACES 1.3".to_string(),
            description: "Latest ACES with improved gamut mapping".to_string(),
            config_path: PathBuf::from("/usr/share/ocio/aces_1.3/config.ocio"),
            workflow: "vfx".to_string(),
        },
        OcioPreset {
            name: "Filmic Blender".to_string(),
            description: "Blender's filmic color transform for realistic renders".to_string(),
            config_path: PathBuf::from("/usr/share/ocio/filmic-blender/config.ocio"),
            workflow: "3d".to_string(),
        },
        OcioPreset {
            name: "sRGB Linear".to_string(),
            description: "Simple sRGB workflow for web and general use".to_string(),
            config_path: PathBuf::from("/usr/share/ocio/srgb/config.ocio"),
            workflow: "web".to_string(),
        },
        OcioPreset {
            name: "Rec.709 Video".to_string(),
            description: "Standard HD video color space".to_string(),
            config_path: PathBuf::from("/usr/share/ocio/rec709/config.ocio"),
            workflow: "video".to_string(),
        },
    ]
}
