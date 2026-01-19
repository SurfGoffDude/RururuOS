#![allow(clippy::excessive_precision)]

use crate::monitor::HdrCapability;
use crate::{ColorError, Result};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone)]
pub struct HdrSupport {
    pub enabled: bool,
    pub monitors: Vec<HdrMonitorState>,
}

#[derive(Debug, Clone)]
pub struct HdrMonitorState {
    pub name: String,
    pub hdr_active: bool,
    pub capability: HdrCapability,
    pub metadata: Option<HdrMetadata>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HdrMetadata {
    pub format: HdrFormat,
    pub max_luminance: u32,
    pub max_frame_average: u32,
    pub min_luminance: f32,
    pub primaries: ColorPrimaries,
    pub white_point: (f32, f32),
    pub transfer_function: TransferFunction,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum HdrFormat {
    Sdr,
    Hdr10,
    Hdr10Plus,
    DolbyVision,
    Hlg,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct ColorPrimaries {
    pub red: (f32, f32),
    pub green: (f32, f32),
    pub blue: (f32, f32),
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum TransferFunction {
    Srgb,
    Bt1886,
    Pq,  // Perceptual Quantizer (ST 2084)
    Hlg, // Hybrid Log-Gamma
    Linear,
}

impl Default for ColorPrimaries {
    fn default() -> Self {
        Self::bt709()
    }
}

impl ColorPrimaries {
    pub fn bt709() -> Self {
        Self {
            red: (0.64, 0.33),
            green: (0.30, 0.60),
            blue: (0.15, 0.06),
        }
    }

    pub fn bt2020() -> Self {
        Self {
            red: (0.708, 0.292),
            green: (0.170, 0.797),
            blue: (0.131, 0.046),
        }
    }

    pub fn dci_p3() -> Self {
        Self {
            red: (0.680, 0.320),
            green: (0.265, 0.690),
            blue: (0.150, 0.060),
        }
    }
}

impl HdrSupport {
    pub fn new() -> Self {
        Self {
            enabled: false,
            monitors: Vec::new(),
        }
    }

    pub fn detect() -> Result<Self> {
        let monitors = crate::monitor::detect_monitors()?;

        let hdr_monitors: Vec<HdrMonitorState> = monitors
            .iter()
            .filter(|m| m.capabilities.hdr_support != HdrCapability::None)
            .map(|m| HdrMonitorState {
                name: m.name.clone(),
                hdr_active: false,
                capability: m.capabilities.hdr_support,
                metadata: None,
            })
            .collect();

        let any_hdr = !hdr_monitors.is_empty();

        Ok(Self {
            enabled: any_hdr,
            monitors: hdr_monitors,
        })
    }

    pub fn enable_hdr(&mut self, monitor_name: &str) -> Result<()> {
        if let Some(monitor) = self.monitors.iter_mut().find(|m| m.name == monitor_name) {
            if monitor.capability == HdrCapability::None {
                return Err(ColorError::HdrNotSupported);
            }

            #[cfg(target_os = "linux")]
            {
                // Enable HDR through KMS/DRM
                // This requires compositor support (wlroots, KWin, etc.)
                let _ = enable_hdr_drm(monitor_name);
            }

            monitor.hdr_active = true;
            monitor.metadata = Some(default_hdr10_metadata());

            Ok(())
        } else {
            Err(ColorError::MonitorNotFound(monitor_name.to_string()))
        }
    }

    pub fn disable_hdr(&mut self, monitor_name: &str) -> Result<()> {
        if let Some(monitor) = self.monitors.iter_mut().find(|m| m.name == monitor_name) {
            #[cfg(target_os = "linux")]
            {
                let _ = disable_hdr_drm(monitor_name);
            }

            monitor.hdr_active = false;
            monitor.metadata = None;

            Ok(())
        } else {
            Err(ColorError::MonitorNotFound(monitor_name.to_string()))
        }
    }

    pub fn set_hdr_metadata(&mut self, monitor_name: &str, metadata: HdrMetadata) -> Result<()> {
        if let Some(monitor) = self.monitors.iter_mut().find(|m| m.name == monitor_name) {
            if !monitor.hdr_active {
                return Err(ColorError::HdrNotSupported);
            }

            monitor.metadata = Some(metadata);
            Ok(())
        } else {
            Err(ColorError::MonitorNotFound(monitor_name.to_string()))
        }
    }

    pub fn is_hdr_active(&self, monitor_name: &str) -> bool {
        self.monitors
            .iter()
            .find(|m| m.name == monitor_name)
            .map(|m| m.hdr_active)
            .unwrap_or(false)
    }
}

impl Default for HdrSupport {
    fn default() -> Self {
        Self::new()
    }
}

fn default_hdr10_metadata() -> HdrMetadata {
    HdrMetadata {
        format: HdrFormat::Hdr10,
        max_luminance: 1000,
        max_frame_average: 400,
        min_luminance: 0.001,
        primaries: ColorPrimaries::bt2020(),
        white_point: (0.3127, 0.3290), // D65
        transfer_function: TransferFunction::Pq,
    }
}

#[cfg(target_os = "linux")]
fn enable_hdr_drm(_monitor: &str) -> Result<()> {
    // Would interact with DRM/KMS to enable HDR
    // Requires kernel 5.18+ and compatible compositor
    Ok(())
}

#[cfg(target_os = "linux")]
fn disable_hdr_drm(_monitor: &str) -> Result<()> {
    Ok(())
}

pub fn tone_map_pq_to_sdr(value: f32, max_content_luminance: f32, target_luminance: f32) -> f32 {
    // Simple Reinhard tone mapping
    let normalized = value / max_content_luminance;
    let mapped = normalized / (1.0 + normalized);
    mapped * target_luminance
}

pub fn pq_eotf(value: f32) -> f32 {
    // PQ (ST 2084) electro-optical transfer function
    let m1 = 0.1593017578125;
    let m2 = 78.84375;
    let c1 = 0.8359375;
    let c2 = 18.8515625;
    let c3 = 18.6875;

    let v_pow_1_m2 = value.powf(1.0 / m2);
    let numerator = (v_pow_1_m2 - c1).max(0.0);
    let denominator = c2 - c3 * v_pow_1_m2;

    10000.0 * (numerator / denominator).powf(1.0 / m1)
}

pub fn pq_oetf(luminance: f32) -> f32 {
    // PQ (ST 2084) opto-electrical transfer function
    let m1 = 0.1593017578125;
    let m2 = 78.84375;
    let c1 = 0.8359375;
    let c2 = 18.8515625;
    let c3 = 18.6875;

    let y = (luminance / 10000.0).max(0.0);
    let y_pow_m1 = y.powf(m1);

    ((c1 + c2 * y_pow_m1) / (1.0 + c3 * y_pow_m1)).powf(m2)
}
