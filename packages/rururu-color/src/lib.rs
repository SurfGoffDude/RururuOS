pub mod config;
pub mod icc;
pub mod ocio;
pub mod monitor;
pub mod hdr;
pub mod dbus;

pub use config::ColorConfig;
pub use icc::IccManager;
pub use monitor::MonitorProfile;
pub use hdr::HdrSupport;

use thiserror::Error;

#[derive(Error, Debug)]
pub enum ColorError {
    #[error("ICC profile error: {0}")]
    IccError(String),
    
    #[error("OpenColorIO error: {0}")]
    OcioError(String),
    
    #[error("Monitor not found: {0}")]
    MonitorNotFound(String),
    
    #[error("HDR not supported")]
    HdrNotSupported,
    
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    
    #[error("Config error: {0}")]
    Config(String),
}

pub type Result<T> = std::result::Result<T, ColorError>;
