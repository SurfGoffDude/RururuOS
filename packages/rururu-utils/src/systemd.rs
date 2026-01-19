use std::collections::HashMap;
use thiserror::Error;
use tracing::{debug, info, warn};
use zbus::{blocking::Connection, proxy};

#[derive(Error, Debug)]
pub enum SystemdError {
    #[error("D-Bus error: {0}")]
    DbusError(String),
    #[error("Unit not found: {0}")]
    UnitNotFound(String),
    #[error("Operation failed: {0}")]
    OperationFailed(String),
}

impl From<zbus::Error> for SystemdError {
    fn from(e: zbus::Error) -> Self {
        SystemdError::DbusError(e.to_string())
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum UnitState {
    Active,
    Inactive,
    Failed,
    Activating,
    Deactivating,
    Reloading,
    Unknown,
}

impl From<&str> for UnitState {
    fn from(s: &str) -> Self {
        match s {
            "active" => UnitState::Active,
            "inactive" => UnitState::Inactive,
            "failed" => UnitState::Failed,
            "activating" => UnitState::Activating,
            "deactivating" => UnitState::Deactivating,
            "reloading" => UnitState::Reloading,
            _ => UnitState::Unknown,
        }
    }
}

#[derive(Debug, Clone)]
pub struct UnitInfo {
    pub name: String,
    pub description: String,
    pub load_state: String,
    pub active_state: UnitState,
    pub sub_state: String,
}

#[proxy(
    interface = "org.freedesktop.systemd1.Manager",
    default_service = "org.freedesktop.systemd1",
    default_path = "/org/freedesktop/systemd1"
)]
trait SystemdManager {
    fn start_unit(&self, name: &str, mode: &str) -> zbus::Result<zbus::zvariant::OwnedObjectPath>;
    fn stop_unit(&self, name: &str, mode: &str) -> zbus::Result<zbus::zvariant::OwnedObjectPath>;
    fn restart_unit(&self, name: &str, mode: &str) -> zbus::Result<zbus::zvariant::OwnedObjectPath>;
    fn reload_unit(&self, name: &str, mode: &str) -> zbus::Result<zbus::zvariant::OwnedObjectPath>;
    fn enable_unit_files(&self, files: &[&str], runtime: bool, force: bool) -> zbus::Result<(bool, Vec<(String, String, String)>)>;
    fn disable_unit_files(&self, files: &[&str], runtime: bool) -> zbus::Result<Vec<(String, String, String)>>;
    fn get_unit(&self, name: &str) -> zbus::Result<zbus::zvariant::OwnedObjectPath>;
    fn list_units(&self) -> zbus::Result<Vec<(String, String, String, String, String, String, zbus::zvariant::OwnedObjectPath, u32, String, zbus::zvariant::OwnedObjectPath)>>;
    fn reload(&self) -> zbus::Result<()>;
}

pub struct SystemdManager {
    connection: Connection,
}

impl SystemdManager {
    pub fn new() -> Result<Self, SystemdError> {
        let connection = Connection::system()?;
        debug!("Connected to systemd via D-Bus");
        Ok(Self { connection })
    }

    pub fn new_user() -> Result<Self, SystemdError> {
        let connection = Connection::session()?;
        debug!("Connected to user systemd via D-Bus");
        Ok(Self { connection })
    }

    fn get_proxy(&self) -> Result<SystemdManagerProxyBlocking<'_>, SystemdError> {
        Ok(SystemdManagerProxyBlocking::new(&self.connection)?)
    }

    pub fn start(&self, unit: &str) -> Result<(), SystemdError> {
        let proxy = self.get_proxy()?;
        info!("Starting unit: {}", unit);
        proxy.start_unit(unit, "replace")?;
        Ok(())
    }

    pub fn stop(&self, unit: &str) -> Result<(), SystemdError> {
        let proxy = self.get_proxy()?;
        info!("Stopping unit: {}", unit);
        proxy.stop_unit(unit, "replace")?;
        Ok(())
    }

    pub fn restart(&self, unit: &str) -> Result<(), SystemdError> {
        let proxy = self.get_proxy()?;
        info!("Restarting unit: {}", unit);
        proxy.restart_unit(unit, "replace")?;
        Ok(())
    }

    pub fn reload(&self, unit: &str) -> Result<(), SystemdError> {
        let proxy = self.get_proxy()?;
        info!("Reloading unit: {}", unit);
        proxy.reload_unit(unit, "replace")?;
        Ok(())
    }

    pub fn enable(&self, unit: &str) -> Result<(), SystemdError> {
        let proxy = self.get_proxy()?;
        info!("Enabling unit: {}", unit);
        proxy.enable_unit_files(&[unit], false, false)?;
        Ok(())
    }

    pub fn disable(&self, unit: &str) -> Result<(), SystemdError> {
        let proxy = self.get_proxy()?;
        info!("Disabling unit: {}", unit);
        proxy.disable_unit_files(&[unit], false)?;
        Ok(())
    }

    pub fn daemon_reload(&self) -> Result<(), SystemdError> {
        let proxy = self.get_proxy()?;
        info!("Reloading systemd daemon");
        proxy.reload()?;
        Ok(())
    }

    pub fn list_units(&self) -> Result<Vec<UnitInfo>, SystemdError> {
        let proxy = self.get_proxy()?;
        let units = proxy.list_units()?;

        Ok(units
            .into_iter()
            .map(|(name, description, load_state, active_state, sub_state, _, _, _, _, _)| {
                UnitInfo {
                    name,
                    description,
                    load_state,
                    active_state: UnitState::from(active_state.as_str()),
                    sub_state,
                }
            })
            .collect())
    }

    pub fn get_unit_state(&self, unit: &str) -> Result<UnitState, SystemdError> {
        let units = self.list_units()?;
        units
            .iter()
            .find(|u| u.name == unit)
            .map(|u| u.active_state.clone())
            .ok_or_else(|| SystemdError::UnitNotFound(unit.to_string()))
    }

    pub fn is_active(&self, unit: &str) -> Result<bool, SystemdError> {
        match self.get_unit_state(unit) {
            Ok(state) => Ok(state == UnitState::Active),
            Err(SystemdError::UnitNotFound(_)) => Ok(false),
            Err(e) => Err(e),
        }
    }

    pub fn list_rururu_services(&self) -> Result<Vec<UnitInfo>, SystemdError> {
        let units = self.list_units()?;
        Ok(units
            .into_iter()
            .filter(|u| u.name.starts_with("rururu"))
            .collect())
    }
}

pub fn create_service_unit(
    name: &str,
    description: &str,
    exec_start: &str,
    options: HashMap<String, String>,
) -> String {
    let mut unit = format!(
        r#"[Unit]
Description={}
After=network.target

[Service]
Type=simple
ExecStart={}
Restart=on-failure
RestartSec=5
"#,
        description, exec_start
    );

    for (key, value) in options {
        unit.push_str(&format!("{}={}\n", key, value));
    }

    unit.push_str(
        r#"
[Install]
WantedBy=multi-user.target
"#,
    );

    unit
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_unit_state_from_str() {
        assert_eq!(UnitState::from("active"), UnitState::Active);
        assert_eq!(UnitState::from("inactive"), UnitState::Inactive);
        assert_eq!(UnitState::from("failed"), UnitState::Failed);
        assert_eq!(UnitState::from("unknown_state"), UnitState::Unknown);
    }

    #[test]
    fn test_create_service_unit() {
        let mut opts = HashMap::new();
        opts.insert("User".to_string(), "rururu".to_string());

        let unit = create_service_unit(
            "rururu-test",
            "Test Service",
            "/usr/bin/test",
            opts,
        );

        assert!(unit.contains("Description=Test Service"));
        assert!(unit.contains("ExecStart=/usr/bin/test"));
        assert!(unit.contains("User=rururu"));
    }
}
