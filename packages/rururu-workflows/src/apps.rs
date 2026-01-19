use crate::{Result, WorkflowError};
use crate::config::PackageManager;
use crate::profiles::AppConfig;
use std::process::Command;

pub fn is_app_installed(app: &AppConfig) -> bool {
    // Check native executable
    if Command::new("which")
        .arg(&app.executable)
        .output()
        .map(|o| o.status.success())
        .unwrap_or(false)
    {
        return true;
    }
    
    // Check flatpak
    if let Some(ref flatpak_id) = app.flatpak_id {
        if Command::new("flatpak")
            .args(["info", flatpak_id])
            .output()
            .map(|o| o.status.success())
            .unwrap_or(false)
        {
            return true;
        }
    }
    
    false
}

pub fn install_app(app: &AppConfig, pm: PackageManager) -> Result<()> {
    // Try flatpak first if available
    if let Some(ref flatpak_id) = app.flatpak_id {
        let result = Command::new("flatpak")
            .args(["install", "-y", "flathub", flatpak_id])
            .status();
        
        if result.map(|s| s.success()).unwrap_or(false) {
            return Ok(());
        }
    }
    
    // Fall back to native package manager
    let (cmd, args) = match pm {
        PackageManager::Pacman => ("sudo", vec!["pacman", "-S", "--noconfirm", &app.package]),
        PackageManager::Apt => ("sudo", vec!["apt", "install", "-y", &app.package]),
        PackageManager::Dnf => ("sudo", vec!["dnf", "install", "-y", &app.package]),
        PackageManager::Zypper => ("sudo", vec!["zypper", "install", "-y", &app.package]),
        PackageManager::Flatpak => {
            if let Some(ref flatpak_id) = app.flatpak_id {
                ("flatpak", vec!["install", "-y", "flathub", flatpak_id])
            } else {
                return Err(WorkflowError::AppNotFound(app.name.clone()));
            }
        }
    };
    
    let status = Command::new(cmd)
        .args(&args)
        .status()?;
    
    if status.success() {
        Ok(())
    } else {
        Err(WorkflowError::AppNotFound(format!(
            "Failed to install {}",
            app.name
        )))
    }
}

pub fn launch_app(app: &AppConfig) -> Result<()> {
    // Try native first
    if Command::new("which")
        .arg(&app.executable)
        .output()
        .map(|o| o.status.success())
        .unwrap_or(false)
    {
        Command::new(&app.executable)
            .spawn()
            .map_err(|e| WorkflowError::System(e.to_string()))?;
        return Ok(());
    }
    
    // Try flatpak
    if let Some(ref flatpak_id) = app.flatpak_id {
        Command::new("flatpak")
            .args(["run", flatpak_id])
            .spawn()
            .map_err(|e| WorkflowError::System(e.to_string()))?;
        return Ok(());
    }
    
    Err(WorkflowError::AppNotFound(app.name.clone()))
}

pub fn get_app_version(app: &AppConfig) -> Option<String> {
    // Try --version
    if let Ok(output) = Command::new(&app.executable)
        .arg("--version")
        .output()
    {
        if output.status.success() {
            return String::from_utf8(output.stdout)
                .ok()
                .map(|s| s.lines().next().unwrap_or("").to_string());
        }
    }
    
    None
}

pub fn list_installed_creative_apps() -> Vec<String> {
    let apps = [
        "blender",
        "gimp",
        "inkscape",
        "krita",
        "darktable",
        "rawtherapee",
        "digikam",
        "kdenlive",
        "resolve",
        "ardour",
        "audacity",
        "obs",
        "freecad",
        "scribus",
    ];
    
    apps.iter()
        .filter(|app| {
            Command::new("which")
                .arg(*app)
                .output()
                .map(|o| o.status.success())
                .unwrap_or(false)
        })
        .map(|s| s.to_string())
        .collect()
}
