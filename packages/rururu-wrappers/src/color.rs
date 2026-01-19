use std::path::Path;
use thiserror::Error;
use tracing::{debug, warn};

#[derive(Error, Debug)]
pub enum ColorError {
    #[error("Failed to load color profile: {0}")]
    ProfileLoadError(String),
    #[error("Color transform error: {0}")]
    TransformError(String),
    #[error("Unsupported color space: {0}")]
    UnsupportedColorSpace(String),
    #[error("OpenColorIO not available")]
    OcioNotAvailable,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ColorSpace {
    SRGB,
    Linear,
    ACEScg,
    ACES2065_1,
    Rec709,
    Rec2020,
    DCI_P3,
    DisplayP3,
    AdobeRGB,
    ProPhotoRGB,
    XYZ,
    Raw,
    Custom,
}

impl ColorSpace {
    pub fn name(&self) -> &'static str {
        match self {
            ColorSpace::SRGB => "sRGB",
            ColorSpace::Linear => "Linear",
            ColorSpace::ACEScg => "ACEScg",
            ColorSpace::ACES2065_1 => "ACES2065-1",
            ColorSpace::Rec709 => "Rec.709",
            ColorSpace::Rec2020 => "Rec.2020",
            ColorSpace::DCI_P3 => "DCI-P3",
            ColorSpace::DisplayP3 => "Display P3",
            ColorSpace::AdobeRGB => "Adobe RGB",
            ColorSpace::ProPhotoRGB => "ProPhoto RGB",
            ColorSpace::XYZ => "CIE XYZ",
            ColorSpace::Raw => "Raw",
            ColorSpace::Custom => "Custom",
        }
    }

    pub fn from_name(name: &str) -> Option<Self> {
        match name.to_lowercase().as_str() {
            "srgb" => Some(ColorSpace::SRGB),
            "linear" | "linear srgb" => Some(ColorSpace::Linear),
            "acescg" => Some(ColorSpace::ACEScg),
            "aces2065-1" | "aces" => Some(ColorSpace::ACES2065_1),
            "rec709" | "rec.709" => Some(ColorSpace::Rec709),
            "rec2020" | "rec.2020" => Some(ColorSpace::Rec2020),
            "dci-p3" | "dcip3" => Some(ColorSpace::DCI_P3),
            "display p3" | "displayp3" => Some(ColorSpace::DisplayP3),
            "adobe rgb" | "adobergb" => Some(ColorSpace::AdobeRGB),
            "prophoto" | "prophoto rgb" => Some(ColorSpace::ProPhotoRGB),
            "xyz" | "cie xyz" => Some(ColorSpace::XYZ),
            "raw" => Some(ColorSpace::Raw),
            _ => None,
        }
    }
}

pub struct ColorManager {
    config_path: Option<String>,
    working_space: ColorSpace,
}

impl ColorManager {
    pub fn new() -> Self {
        Self {
            config_path: None,
            working_space: ColorSpace::Linear,
        }
    }

    pub fn with_config<P: AsRef<Path>>(path: P) -> Result<Self, ColorError> {
        let path_str = path.as_ref().to_string_lossy().to_string();
        if !path.as_ref().exists() {
            return Err(ColorError::ProfileLoadError(format!(
                "Config not found: {}",
                path_str
            )));
        }

        debug!("Loading OCIO config from: {}", path_str);

        Ok(Self {
            config_path: Some(path_str),
            working_space: ColorSpace::Linear,
        })
    }

    pub fn set_working_space(&mut self, space: ColorSpace) {
        self.working_space = space;
    }

    pub fn working_space(&self) -> ColorSpace {
        self.working_space
    }

    pub fn transform_rgb(
        &self,
        rgb: [f32; 3],
        from: ColorSpace,
        to: ColorSpace,
    ) -> Result<[f32; 3], ColorError> {
        if from == to {
            return Ok(rgb);
        }

        // Software fallback for common transforms
        match (from, to) {
            (ColorSpace::SRGB, ColorSpace::Linear) => Ok(self.srgb_to_linear(rgb)),
            (ColorSpace::Linear, ColorSpace::SRGB) => Ok(self.linear_to_srgb(rgb)),
            (ColorSpace::Linear, ColorSpace::ACEScg) => Ok(self.linear_to_acescg(rgb)),
            (ColorSpace::ACEScg, ColorSpace::Linear) => Ok(self.acescg_to_linear(rgb)),
            _ => {
                warn!(
                    "Transform from {} to {} using approximation",
                    from.name(),
                    to.name()
                );
                // Generic transform via XYZ
                let xyz = self.to_xyz(rgb, from)?;
                self.from_xyz(xyz, to)
            }
        }
    }

    fn srgb_to_linear(&self, rgb: [f32; 3]) -> [f32; 3] {
        rgb.map(|c| {
            if c <= 0.04045 {
                c / 12.92
            } else {
                ((c + 0.055) / 1.055).powf(2.4)
            }
        })
    }

    fn linear_to_srgb(&self, rgb: [f32; 3]) -> [f32; 3] {
        rgb.map(|c| {
            if c <= 0.0031308 {
                c * 12.92
            } else {
                1.055 * c.powf(1.0 / 2.4) - 0.055
            }
        })
    }

    fn linear_to_acescg(&self, rgb: [f32; 3]) -> [f32; 3] {
        // sRGB linear to ACEScg matrix (approximate)
        let m = [
            [0.6131, 0.3395, 0.0474],
            [0.0701, 0.9164, 0.0135],
            [0.0206, 0.1096, 0.8698],
        ];
        self.matrix_multiply(rgb, m)
    }

    fn acescg_to_linear(&self, rgb: [f32; 3]) -> [f32; 3] {
        // ACEScg to sRGB linear matrix (approximate)
        let m = [
            [1.7051, -0.6218, -0.0833],
            [-0.1302, 1.1408, -0.0106],
            [-0.0240, -0.1289, 1.1529],
        ];
        self.matrix_multiply(rgb, m)
    }

    fn to_xyz(&self, rgb: [f32; 3], from: ColorSpace) -> Result<[f32; 3], ColorError> {
        let linear = match from {
            ColorSpace::SRGB => self.srgb_to_linear(rgb),
            ColorSpace::Linear => rgb,
            ColorSpace::XYZ => return Ok(rgb),
            _ => {
                return Err(ColorError::UnsupportedColorSpace(from.name().to_string()));
            }
        };

        // sRGB/Linear to XYZ
        let m = [
            [0.4124564, 0.3575761, 0.1804375],
            [0.2126729, 0.7151522, 0.0721750],
            [0.0193339, 0.1191920, 0.9503041],
        ];
        Ok(self.matrix_multiply(linear, m))
    }

    fn from_xyz(&self, xyz: [f32; 3], to: ColorSpace) -> Result<[f32; 3], ColorError> {
        if to == ColorSpace::XYZ {
            return Ok(xyz);
        }

        // XYZ to sRGB/Linear
        let m = [
            [3.2404542, -1.5371385, -0.4985314],
            [-0.9692660, 1.8760108, 0.0415560],
            [0.0556434, -0.2040259, 1.0572252],
        ];
        let linear = self.matrix_multiply(xyz, m);

        match to {
            ColorSpace::Linear => Ok(linear),
            ColorSpace::SRGB => Ok(self.linear_to_srgb(linear)),
            _ => Err(ColorError::UnsupportedColorSpace(to.name().to_string())),
        }
    }

    fn matrix_multiply(&self, v: [f32; 3], m: [[f32; 3]; 3]) -> [f32; 3] {
        [
            m[0][0] * v[0] + m[0][1] * v[1] + m[0][2] * v[2],
            m[1][0] * v[0] + m[1][1] * v[1] + m[1][2] * v[2],
            m[2][0] * v[0] + m[2][1] * v[1] + m[2][2] * v[2],
        ]
    }

    pub fn list_color_spaces(&self) -> Vec<ColorSpace> {
        vec![
            ColorSpace::SRGB,
            ColorSpace::Linear,
            ColorSpace::ACEScg,
            ColorSpace::ACES2065_1,
            ColorSpace::Rec709,
            ColorSpace::Rec2020,
            ColorSpace::DCI_P3,
            ColorSpace::DisplayP3,
            ColorSpace::AdobeRGB,
            ColorSpace::ProPhotoRGB,
            ColorSpace::XYZ,
        ]
    }
}

impl Default for ColorManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_srgb_linear_roundtrip() {
        let cm = ColorManager::new();
        let original = [0.5, 0.3, 0.8];

        let linear = cm.srgb_to_linear(original);
        let back = cm.linear_to_srgb(linear);

        for i in 0..3 {
            assert!((original[i] - back[i]).abs() < 0.0001);
        }
    }

    #[test]
    fn test_color_space_from_name() {
        assert_eq!(ColorSpace::from_name("srgb"), Some(ColorSpace::SRGB));
        assert_eq!(ColorSpace::from_name("ACEScg"), Some(ColorSpace::ACEScg));
        assert_eq!(ColorSpace::from_name("unknown"), None);
    }

    #[test]
    fn test_transform_same_space() {
        let cm = ColorManager::new();
        let rgb = [0.5, 0.3, 0.8];
        let result = cm
            .transform_rgb(rgb, ColorSpace::SRGB, ColorSpace::SRGB)
            .unwrap();
        assert_eq!(rgb, result);
    }
}
