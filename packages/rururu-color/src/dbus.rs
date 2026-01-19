use crate::{ColorConfig, ColorError, IccManager, Result};
use crate::hdr::HdrSupport;
use crate::monitor::MonitorProfile;
use crate::ocio::OcioManager;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use zbus::{interface, Connection};

pub struct ColorService {
    config: Arc<RwLock<ColorConfig>>,
    icc_manager: Arc<RwLock<IccManager>>,
    ocio_manager: Arc<RwLock<OcioManager>>,
    hdr_support: Arc<RwLock<HdrSupport>>,
    monitors: Arc<RwLock<Vec<MonitorProfile>>>,
}

impl ColorService {
    pub fn new() -> Self {
        Self {
            config: Arc::new(RwLock::new(ColorConfig::default())),
            icc_manager: Arc::new(RwLock::new(IccManager::new())),
            ocio_manager: Arc::new(RwLock::new(OcioManager::new())),
            hdr_support: Arc::new(RwLock::new(HdrSupport::new())),
            monitors: Arc::new(RwLock::new(Vec::new())),
        }
    }
    
    pub async fn init(&self) -> Result<()> {
        // Load config
        let config = ColorConfig::load()?;
        *self.config.write().await = config;
        
        // Scan ICC profiles
        self.icc_manager.write().await.scan_profiles();
        
        // Detect monitors
        let monitors = crate::monitor::detect_monitors()?;
        *self.monitors.write().await = monitors;
        
        // Detect HDR support
        let hdr = HdrSupport::detect()?;
        *self.hdr_support.write().await = hdr;
        
        Ok(())
    }
}

#[interface(name = "org.rururu.ColorManagement1")]
impl ColorService {
    async fn get_version(&self) -> String {
        env!("CARGO_PKG_VERSION").to_string()
    }
    
    async fn is_enabled(&self) -> bool {
        self.config.read().await.global.enabled
    }
    
    async fn set_enabled(&self, enabled: bool) -> bool {
        let mut config = self.config.write().await;
        config.global.enabled = enabled;
        config.save().is_ok()
    }
    
    async fn list_monitors(&self) -> Vec<String> {
        self.monitors
            .read()
            .await
            .iter()
            .map(|m| m.name.clone())
            .collect()
    }
    
    async fn get_monitor_profile(&self, monitor: String) -> String {
        self.config
            .read()
            .await
            .monitors
            .get(&monitor)
            .and_then(|m| m.icc_profile.as_ref())
            .and_then(|p| p.to_str())
            .unwrap_or("")
            .to_string()
    }
    
    async fn set_monitor_profile(&self, monitor: String, profile_path: String) -> bool {
        let mut config = self.config.write().await;
        
        if let Some(mon_config) = config.monitors.get_mut(&monitor) {
            mon_config.icc_profile = if profile_path.is_empty() {
                None
            } else {
                Some(profile_path.into())
            };
            return config.save().is_ok();
        }
        
        false
    }
    
    async fn list_profiles(&self) -> Vec<String> {
        self.icc_manager
            .read()
            .await
            .list_profiles()
            .iter()
            .map(|p| p.name.clone())
            .collect()
    }
    
    async fn list_display_profiles(&self) -> Vec<String> {
        self.icc_manager
            .read()
            .await
            .list_display_profiles()
            .iter()
            .map(|p| p.name.clone())
            .collect()
    }
    
    async fn get_profile_path(&self, name: String) -> String {
        self.icc_manager
            .read()
            .await
            .get_profile(&name)
            .map(|p| p.path.to_string_lossy().to_string())
            .unwrap_or_default()
    }
    
    async fn install_profile(&self, path: String) -> bool {
        self.icc_manager
            .write()
            .await
            .install_profile(std::path::Path::new(&path))
            .is_ok()
    }
    
    async fn is_hdr_supported(&self) -> bool {
        !self.hdr_support.read().await.monitors.is_empty()
    }
    
    async fn is_hdr_active(&self, monitor: String) -> bool {
        self.hdr_support.read().await.is_hdr_active(&monitor)
    }
    
    async fn enable_hdr(&self, monitor: String) -> bool {
        self.hdr_support.write().await.enable_hdr(&monitor).is_ok()
    }
    
    async fn disable_hdr(&self, monitor: String) -> bool {
        self.hdr_support.write().await.disable_hdr(&monitor).is_ok()
    }
    
    async fn get_ocio_config(&self) -> String {
        self.ocio_manager
            .read()
            .await
            .get_config()
            .map(|c| c.path.to_string_lossy().to_string())
            .unwrap_or_default()
    }
    
    async fn set_ocio_config(&self, path: String) -> bool {
        if path.is_empty() {
            self.ocio_manager.write().await.unload_config();
            return true;
        }
        
        self.ocio_manager
            .write()
            .await
            .load_config(std::path::Path::new(&path))
            .is_ok()
    }
    
    async fn list_ocio_color_spaces(&self) -> Vec<String> {
        self.ocio_manager
            .read()
            .await
            .list_color_spaces()
            .iter()
            .map(|s| s.to_string())
            .collect()
    }
    
    async fn list_workflows(&self) -> Vec<String> {
        self.config
            .read()
            .await
            .workflows
            .keys()
            .cloned()
            .collect()
    }
    
    async fn get_workflow_config(&self, name: String) -> String {
        self.config
            .read()
            .await
            .workflows
            .get(&name)
            .map(|w| serde_json::to_string(w).unwrap_or_default())
            .unwrap_or_default()
    }
    
    async fn refresh(&self) -> bool {
        // Rescan profiles and monitors
        self.icc_manager.write().await.scan_profiles();
        
        if let Ok(monitors) = crate::monitor::detect_monitors() {
            *self.monitors.write().await = monitors;
        }
        
        if let Ok(hdr) = HdrSupport::detect() {
            *self.hdr_support.write().await = hdr;
        }
        
        true
    }
}

pub async fn run_service() -> Result<()> {
    let service = ColorService::new();
    service.init().await?;
    
    let connection = Connection::session()
        .await
        .map_err(|e| ColorError::Config(e.to_string()))?;
    
    connection
        .object_server()
        .at("/org/rururu/ColorManagement1", service)
        .await
        .map_err(|e| ColorError::Config(e.to_string()))?;
    
    connection
        .request_name("org.rururu.ColorManagement1")
        .await
        .map_err(|e| ColorError::Config(e.to_string()))?;
    
    tracing::info!("Color management D-Bus service started");
    
    // Keep running
    std::future::pending::<()>().await;
    
    Ok(())
}
