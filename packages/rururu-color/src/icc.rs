use crate::{ColorError, Result};
use std::collections::HashMap;
use std::path::{Path, PathBuf};

#[derive(Debug, Clone)]
pub struct IccProfile {
    pub path: PathBuf,
    pub name: String,
    pub description: String,
    pub color_space: ColorSpace,
    pub profile_class: ProfileClass,
    pub white_point: (f64, f64, f64),
    pub copyright: Option<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ColorSpace {
    RGB,
    CMYK,
    Gray,
    Lab,
    XYZ,
    Unknown,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProfileClass {
    Input,
    Display,
    Output,
    DeviceLink,
    ColorSpace,
    Abstract,
    NamedColor,
    Unknown,
}

pub struct IccManager {
    profiles: HashMap<String, IccProfile>,
    system_paths: Vec<PathBuf>,
    user_path: PathBuf,
}

impl IccManager {
    pub fn new() -> Self {
        let system_paths = vec![
            PathBuf::from("/usr/share/color/icc"),
            PathBuf::from("/usr/local/share/color/icc"),
            PathBuf::from("/var/lib/colord/icc"),
        ];

        let user_path = dirs::data_dir()
            .unwrap_or_else(|| PathBuf::from("."))
            .join("icc");

        let mut manager = Self {
            profiles: HashMap::new(),
            system_paths,
            user_path,
        };

        manager.scan_profiles();
        manager
    }

    pub fn scan_profiles(&mut self) {
        self.profiles.clear();

        // Collect paths first to avoid borrow conflict
        let paths_to_scan: Vec<PathBuf> = self
            .system_paths
            .iter()
            .cloned()
            .chain(std::iter::once(self.user_path.clone()))
            .collect();

        for path in paths_to_scan {
            self.scan_directory(&path);
        }
    }

    fn scan_directory(&mut self, dir: &Path) {
        if !dir.exists() {
            return;
        }

        if let Ok(entries) = std::fs::read_dir(dir) {
            for entry in entries.flatten() {
                let path = entry.path();
                if path
                    .extension()
                    .map(|e| e == "icc" || e == "icm")
                    .unwrap_or(false)
                {
                    if let Ok(profile) = self.load_profile(&path) {
                        self.profiles.insert(profile.name.clone(), profile);
                    }
                }
            }
        }
    }

    fn load_profile(&self, path: &Path) -> Result<IccProfile> {
        // Read ICC profile header to extract basic info
        let data = std::fs::read(path)?;

        if data.len() < 128 {
            return Err(ColorError::IccError("Profile too small".to_string()));
        }

        // Parse ICC header
        let profile_size = u32::from_be_bytes([data[0], data[1], data[2], data[3]]) as usize;

        if data.len() < profile_size {
            return Err(ColorError::IccError("Incomplete profile".to_string()));
        }

        // Color space signature at offset 16
        let color_space = match &data[16..20] {
            b"RGB " => ColorSpace::RGB,
            b"CMYK" => ColorSpace::CMYK,
            b"GRAY" => ColorSpace::Gray,
            b"Lab " => ColorSpace::Lab,
            b"XYZ " => ColorSpace::XYZ,
            _ => ColorSpace::Unknown,
        };

        // Profile class at offset 12
        let profile_class = match &data[12..16] {
            b"scnr" => ProfileClass::Input,
            b"mntr" => ProfileClass::Display,
            b"prtr" => ProfileClass::Output,
            b"link" => ProfileClass::DeviceLink,
            b"spac" => ProfileClass::ColorSpace,
            b"abst" => ProfileClass::Abstract,
            b"nmcl" => ProfileClass::NamedColor,
            _ => ProfileClass::Unknown,
        };

        // Extract description from filename for now
        let name = path
            .file_stem()
            .and_then(|s| s.to_str())
            .unwrap_or("Unknown")
            .to_string();

        Ok(IccProfile {
            path: path.to_path_buf(),
            name: name.clone(),
            description: name,
            color_space,
            profile_class,
            white_point: (0.9505, 1.0, 1.0890), // D65 default
            copyright: None,
        })
    }

    pub fn get_profile(&self, name: &str) -> Option<&IccProfile> {
        self.profiles.get(name)
    }

    pub fn list_profiles(&self) -> Vec<&IccProfile> {
        self.profiles.values().collect()
    }

    pub fn list_display_profiles(&self) -> Vec<&IccProfile> {
        self.profiles
            .values()
            .filter(|p| p.profile_class == ProfileClass::Display)
            .collect()
    }

    pub fn list_rgb_profiles(&self) -> Vec<&IccProfile> {
        self.profiles
            .values()
            .filter(|p| p.color_space == ColorSpace::RGB)
            .collect()
    }

    pub fn install_profile(&mut self, source: &Path) -> Result<()> {
        std::fs::create_dir_all(&self.user_path)?;

        let filename = source
            .file_name()
            .ok_or_else(|| ColorError::IccError("Invalid filename".to_string()))?;

        let dest = self.user_path.join(filename);
        std::fs::copy(source, &dest)?;

        if let Ok(profile) = self.load_profile(&dest) {
            self.profiles.insert(profile.name.clone(), profile);
        }

        Ok(())
    }

    pub fn remove_profile(&mut self, name: &str) -> Result<()> {
        if let Some(profile) = self.profiles.get(name) {
            // Only allow removing user profiles
            if profile.path.starts_with(&self.user_path) {
                std::fs::remove_file(&profile.path)?;
                self.profiles.remove(name);
            } else {
                return Err(ColorError::IccError(
                    "Cannot remove system profile".to_string(),
                ));
            }
        }
        Ok(())
    }
}

impl Default for IccManager {
    fn default() -> Self {
        Self::new()
    }
}

pub fn apply_profile_to_monitor(_profile: &IccProfile, _monitor_name: &str) -> Result<()> {
    // Use colord or direct gamma ramp setting
    #[cfg(target_os = "linux")]
    {
        // Try colord first
        let output = std::process::Command::new("colormgr")
            .args([
                "device-add-profile",
                monitor_name,
                profile.path.to_str().unwrap_or(""),
            ])
            .output();

        match output {
            Ok(o) if o.status.success() => return Ok(()),
            _ => {
                // Fallback to xcalib/dispwin
                let _ = std::process::Command::new("dispwin")
                    .args(["-d", "1", "-I", profile.path.to_str().unwrap_or("")])
                    .output();
            }
        }
    }

    Ok(())
}
