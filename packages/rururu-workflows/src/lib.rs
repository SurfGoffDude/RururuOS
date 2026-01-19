pub mod apps;
pub mod config;
pub mod profiles;
pub mod system;

pub use config::WorkflowConfig;
pub use profiles::{WorkflowProfile, WorkflowType};

use thiserror::Error;

#[derive(Error, Debug)]
pub enum WorkflowError {
    #[error("Profile not found: {0}")]
    ProfileNotFound(String),

    #[error("Application not found: {0}")]
    AppNotFound(String),

    #[error("Configuration error: {0}")]
    Config(String),

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("System error: {0}")]
    System(String),
}

pub type Result<T> = std::result::Result<T, WorkflowError>;
