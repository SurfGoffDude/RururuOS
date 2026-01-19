use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum WorkflowType {
    VideoEditor,
    ThreeDArtist,
    TwoDDesigner,
    AudioProducer,
    Photographer,
    Developer,
    General,
}

impl WorkflowType {
    pub fn all() -> &'static [WorkflowType] {
        &[
            WorkflowType::VideoEditor,
            WorkflowType::ThreeDArtist,
            WorkflowType::TwoDDesigner,
            WorkflowType::AudioProducer,
            WorkflowType::Photographer,
            WorkflowType::Developer,
            WorkflowType::General,
        ]
    }
    
    pub fn name(&self) -> &'static str {
        match self {
            WorkflowType::VideoEditor => "Video Editor",
            WorkflowType::ThreeDArtist => "3D Artist",
            WorkflowType::TwoDDesigner => "2D Designer",
            WorkflowType::AudioProducer => "Audio Producer",
            WorkflowType::Photographer => "Photographer",
            WorkflowType::Developer => "Developer",
            WorkflowType::General => "General",
        }
    }
    
    pub fn icon(&self) -> &'static str {
        match self {
            WorkflowType::VideoEditor => "video-x-generic",
            WorkflowType::ThreeDArtist => "applications-graphics-3d",
            WorkflowType::TwoDDesigner => "applications-graphics",
            WorkflowType::AudioProducer => "audio-x-generic",
            WorkflowType::Photographer => "camera-photo",
            WorkflowType::Developer => "utilities-terminal",
            WorkflowType::General => "applications-other",
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkflowProfile {
    pub workflow_type: WorkflowType,
    pub name: String,
    pub description: String,
    pub applications: Vec<AppConfig>,
    pub system_settings: SystemSettings,
    pub color_config: ColorWorkflowConfig,
    pub keyboard_shortcuts: Vec<KeyboardShortcut>,
    pub startup_apps: Vec<String>,
    pub environment: HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppConfig {
    pub name: String,
    pub executable: String,
    pub package: String,
    pub flatpak_id: Option<String>,
    pub config_path: Option<PathBuf>,
    pub priority: AppPriority,
    pub settings: HashMap<String, String>,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum AppPriority {
    Primary,
    Secondary,
    Optional,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemSettings {
    pub cpu_governor: CpuGovernor,
    pub gpu_performance_mode: bool,
    pub swap_usage: SwapUsage,
    pub io_scheduler: IoScheduler,
    pub realtime_audio: bool,
    pub high_priority_processes: Vec<String>,
    pub memory_pressure_threshold: u8,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum CpuGovernor {
    Performance,
    Powersave,
    Schedutil,
    Ondemand,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum SwapUsage {
    Minimal,    // swappiness = 10
    Balanced,   // swappiness = 60
    Aggressive, // swappiness = 100
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum IoScheduler {
    Bfq,
    Mq_Deadline,
    Kyber,
    None,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ColorWorkflowConfig {
    pub working_space: String,
    pub ocio_config: Option<PathBuf>,
    pub soft_proof_profile: Option<PathBuf>,
    pub default_intent: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KeyboardShortcut {
    pub action: String,
    pub keys: String,
    pub description: String,
}

impl WorkflowProfile {
    pub fn video_editor() -> Self {
        Self {
            workflow_type: WorkflowType::VideoEditor,
            name: "Video Editor".to_string(),
            description: "Optimized for video editing with DaVinci Resolve, Kdenlive".to_string(),
            applications: vec![
                AppConfig {
                    name: "DaVinci Resolve".to_string(),
                    executable: "resolve".to_string(),
                    package: "davinci-resolve".to_string(),
                    flatpak_id: None,
                    config_path: None,
                    priority: AppPriority::Primary,
                    settings: HashMap::new(),
                },
                AppConfig {
                    name: "Kdenlive".to_string(),
                    executable: "kdenlive".to_string(),
                    package: "kdenlive".to_string(),
                    flatpak_id: Some("org.kde.kdenlive".to_string()),
                    config_path: None,
                    priority: AppPriority::Secondary,
                    settings: HashMap::new(),
                },
                AppConfig {
                    name: "Handbrake".to_string(),
                    executable: "ghb".to_string(),
                    package: "handbrake".to_string(),
                    flatpak_id: Some("fr.handbrake.ghb".to_string()),
                    config_path: None,
                    priority: AppPriority::Optional,
                    settings: HashMap::new(),
                },
            ],
            system_settings: SystemSettings {
                cpu_governor: CpuGovernor::Performance,
                gpu_performance_mode: true,
                swap_usage: SwapUsage::Minimal,
                io_scheduler: IoScheduler::Bfq,
                realtime_audio: false,
                high_priority_processes: vec!["resolve".to_string(), "kdenlive".to_string()],
                memory_pressure_threshold: 90,
            },
            color_config: ColorWorkflowConfig {
                working_space: "Rec.709".to_string(),
                ocio_config: Some(PathBuf::from("/usr/share/ocio/aces_1.2/config.ocio")),
                soft_proof_profile: None,
                default_intent: "RelativeColorimetric".to_string(),
            },
            keyboard_shortcuts: vec![
                KeyboardShortcut {
                    action: "Launch DaVinci Resolve".to_string(),
                    keys: "Super+Shift+V".to_string(),
                    description: "Open video editor".to_string(),
                },
            ],
            startup_apps: vec![],
            environment: [
                ("RESOLVE_SCRIPT_API".to_string(), "/opt/resolve/libs/Fusion/".to_string()),
            ].into_iter().collect(),
        }
    }
    
    pub fn three_d_artist() -> Self {
        Self {
            workflow_type: WorkflowType::ThreeDArtist,
            name: "3D Artist".to_string(),
            description: "Optimized for 3D modeling and rendering with Blender".to_string(),
            applications: vec![
                AppConfig {
                    name: "Blender".to_string(),
                    executable: "blender".to_string(),
                    package: "blender".to_string(),
                    flatpak_id: Some("org.blender.Blender".to_string()),
                    config_path: Some(PathBuf::from("~/.config/blender")),
                    priority: AppPriority::Primary,
                    settings: [
                        ("cycles.device".to_string(), "GPU".to_string()),
                    ].into_iter().collect(),
                },
                AppConfig {
                    name: "FreeCAD".to_string(),
                    executable: "freecad".to_string(),
                    package: "freecad".to_string(),
                    flatpak_id: Some("org.freecadweb.FreeCAD".to_string()),
                    config_path: None,
                    priority: AppPriority::Secondary,
                    settings: HashMap::new(),
                },
            ],
            system_settings: SystemSettings {
                cpu_governor: CpuGovernor::Performance,
                gpu_performance_mode: true,
                swap_usage: SwapUsage::Minimal,
                io_scheduler: IoScheduler::Mq_Deadline,
                realtime_audio: false,
                high_priority_processes: vec!["blender".to_string()],
                memory_pressure_threshold: 95,
            },
            color_config: ColorWorkflowConfig {
                working_space: "ACEScg".to_string(),
                ocio_config: Some(PathBuf::from("/usr/share/ocio/aces_1.2/config.ocio")),
                soft_proof_profile: None,
                default_intent: "RelativeColorimetric".to_string(),
            },
            keyboard_shortcuts: vec![
                KeyboardShortcut {
                    action: "Launch Blender".to_string(),
                    keys: "Super+Shift+B".to_string(),
                    description: "Open 3D software".to_string(),
                },
            ],
            startup_apps: vec![],
            environment: [
                ("BLENDER_USER_CONFIG".to_string(), "~/.config/blender".to_string()),
                ("CYCLES_CUDA_EXTRA_CFLAGS".to_string(), "-ccbin=/usr/bin/gcc".to_string()),
            ].into_iter().collect(),
        }
    }
    
    pub fn two_d_designer() -> Self {
        Self {
            workflow_type: WorkflowType::TwoDDesigner,
            name: "2D Designer".to_string(),
            description: "Optimized for 2D graphics with Krita, GIMP, Inkscape".to_string(),
            applications: vec![
                AppConfig {
                    name: "Krita".to_string(),
                    executable: "krita".to_string(),
                    package: "krita".to_string(),
                    flatpak_id: Some("org.kde.krita".to_string()),
                    config_path: Some(PathBuf::from("~/.config/krita")),
                    priority: AppPriority::Primary,
                    settings: HashMap::new(),
                },
                AppConfig {
                    name: "GIMP".to_string(),
                    executable: "gimp".to_string(),
                    package: "gimp".to_string(),
                    flatpak_id: Some("org.gimp.GIMP".to_string()),
                    config_path: None,
                    priority: AppPriority::Secondary,
                    settings: HashMap::new(),
                },
                AppConfig {
                    name: "Inkscape".to_string(),
                    executable: "inkscape".to_string(),
                    package: "inkscape".to_string(),
                    flatpak_id: Some("org.inkscape.Inkscape".to_string()),
                    config_path: None,
                    priority: AppPriority::Secondary,
                    settings: HashMap::new(),
                },
            ],
            system_settings: SystemSettings {
                cpu_governor: CpuGovernor::Schedutil,
                gpu_performance_mode: true,
                swap_usage: SwapUsage::Balanced,
                io_scheduler: IoScheduler::Bfq,
                realtime_audio: false,
                high_priority_processes: vec!["krita".to_string()],
                memory_pressure_threshold: 85,
            },
            color_config: ColorWorkflowConfig {
                working_space: "Adobe RGB".to_string(),
                ocio_config: None,
                soft_proof_profile: Some(PathBuf::from("/usr/share/color/icc/Fogra39.icc")),
                default_intent: "Perceptual".to_string(),
            },
            keyboard_shortcuts: vec![
                KeyboardShortcut {
                    action: "Launch Krita".to_string(),
                    keys: "Super+Shift+K".to_string(),
                    description: "Open drawing app".to_string(),
                },
            ],
            startup_apps: vec![],
            environment: HashMap::new(),
        }
    }
    
    pub fn audio_producer() -> Self {
        Self {
            workflow_type: WorkflowType::AudioProducer,
            name: "Audio Producer".to_string(),
            description: "Optimized for audio production with Ardour, Bitwig".to_string(),
            applications: vec![
                AppConfig {
                    name: "Ardour".to_string(),
                    executable: "ardour8".to_string(),
                    package: "ardour".to_string(),
                    flatpak_id: None,
                    config_path: Some(PathBuf::from("~/.config/ardour8")),
                    priority: AppPriority::Primary,
                    settings: HashMap::new(),
                },
                AppConfig {
                    name: "Bitwig Studio".to_string(),
                    executable: "bitwig-studio".to_string(),
                    package: "bitwig-studio".to_string(),
                    flatpak_id: Some("com.bitwig.BitwigStudio".to_string()),
                    config_path: None,
                    priority: AppPriority::Primary,
                    settings: HashMap::new(),
                },
                AppConfig {
                    name: "Audacity".to_string(),
                    executable: "audacity".to_string(),
                    package: "audacity".to_string(),
                    flatpak_id: Some("org.audacityteam.Audacity".to_string()),
                    config_path: None,
                    priority: AppPriority::Secondary,
                    settings: HashMap::new(),
                },
            ],
            system_settings: SystemSettings {
                cpu_governor: CpuGovernor::Performance,
                gpu_performance_mode: false,
                swap_usage: SwapUsage::Minimal,
                io_scheduler: IoScheduler::Mq_Deadline,
                realtime_audio: true,
                high_priority_processes: vec![
                    "ardour".to_string(),
                    "bitwig-studio".to_string(),
                    "pipewire".to_string(),
                    "wireplumber".to_string(),
                ],
                memory_pressure_threshold: 80,
            },
            color_config: ColorWorkflowConfig {
                working_space: "sRGB".to_string(),
                ocio_config: None,
                soft_proof_profile: None,
                default_intent: "Perceptual".to_string(),
            },
            keyboard_shortcuts: vec![
                KeyboardShortcut {
                    action: "Launch Ardour".to_string(),
                    keys: "Super+Shift+A".to_string(),
                    description: "Open DAW".to_string(),
                },
            ],
            startup_apps: vec!["qpwgraph".to_string()],
            environment: [
                ("PIPEWIRE_LATENCY".to_string(), "64/48000".to_string()),
                ("PIPEWIRE_QUANTUM".to_string(), "64/48000".to_string()),
            ].into_iter().collect(),
        }
    }
    
    pub fn photographer() -> Self {
        Self {
            workflow_type: WorkflowType::Photographer,
            name: "Photographer".to_string(),
            description: "Optimized for photo editing with Darktable, RawTherapee".to_string(),
            applications: vec![
                AppConfig {
                    name: "Darktable".to_string(),
                    executable: "darktable".to_string(),
                    package: "darktable".to_string(),
                    flatpak_id: Some("org.darktable.Darktable".to_string()),
                    config_path: Some(PathBuf::from("~/.config/darktable")),
                    priority: AppPriority::Primary,
                    settings: HashMap::new(),
                },
                AppConfig {
                    name: "RawTherapee".to_string(),
                    executable: "rawtherapee".to_string(),
                    package: "rawtherapee".to_string(),
                    flatpak_id: Some("com.rawtherapee.RawTherapee".to_string()),
                    config_path: None,
                    priority: AppPriority::Secondary,
                    settings: HashMap::new(),
                },
                AppConfig {
                    name: "digiKam".to_string(),
                    executable: "digikam".to_string(),
                    package: "digikam".to_string(),
                    flatpak_id: Some("org.kde.digikam".to_string()),
                    config_path: None,
                    priority: AppPriority::Secondary,
                    settings: HashMap::new(),
                },
            ],
            system_settings: SystemSettings {
                cpu_governor: CpuGovernor::Performance,
                gpu_performance_mode: true,
                swap_usage: SwapUsage::Balanced,
                io_scheduler: IoScheduler::Bfq,
                realtime_audio: false,
                high_priority_processes: vec!["darktable".to_string()],
                memory_pressure_threshold: 85,
            },
            color_config: ColorWorkflowConfig {
                working_space: "ProPhoto RGB".to_string(),
                ocio_config: None,
                soft_proof_profile: Some(PathBuf::from("/usr/share/color/icc/sRGB.icc")),
                default_intent: "Perceptual".to_string(),
            },
            keyboard_shortcuts: vec![
                KeyboardShortcut {
                    action: "Launch Darktable".to_string(),
                    keys: "Super+Shift+D".to_string(),
                    description: "Open photo editor".to_string(),
                },
            ],
            startup_apps: vec![],
            environment: HashMap::new(),
        }
    }
    
    pub fn get_profile(workflow_type: WorkflowType) -> Self {
        match workflow_type {
            WorkflowType::VideoEditor => Self::video_editor(),
            WorkflowType::ThreeDArtist => Self::three_d_artist(),
            WorkflowType::TwoDDesigner => Self::two_d_designer(),
            WorkflowType::AudioProducer => Self::audio_producer(),
            WorkflowType::Photographer => Self::photographer(),
            WorkflowType::Developer | WorkflowType::General => Self::general(),
        }
    }
    
    fn general() -> Self {
        Self {
            workflow_type: WorkflowType::General,
            name: "General".to_string(),
            description: "Balanced settings for general creative work".to_string(),
            applications: vec![],
            system_settings: SystemSettings {
                cpu_governor: CpuGovernor::Schedutil,
                gpu_performance_mode: false,
                swap_usage: SwapUsage::Balanced,
                io_scheduler: IoScheduler::Bfq,
                realtime_audio: false,
                high_priority_processes: vec![],
                memory_pressure_threshold: 80,
            },
            color_config: ColorWorkflowConfig {
                working_space: "sRGB".to_string(),
                ocio_config: None,
                soft_proof_profile: None,
                default_intent: "Perceptual".to_string(),
            },
            keyboard_shortcuts: vec![],
            startup_apps: vec![],
            environment: HashMap::new(),
        }
    }
}
