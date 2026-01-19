use std::path::Path;
use tracing::{info, Level};
use tracing_subscriber::FmtSubscriber;

mod codec_registry;
mod file_detector;

pub use codec_registry::CodecRegistry;
pub use file_detector::FileDetector;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::INFO)
        .init();

    info!("RururuOS File Handler starting...");

    let registry = CodecRegistry::new();
    info!("Loaded {} codec handlers", registry.handler_count());

    let detector = FileDetector::new();
    info!("File detector initialized");

    // TODO: Start D-Bus service
    // TODO: Listen for file detection requests

    info!("File Handler ready");

    // Keep running
    tokio::signal::ctrl_c().await?;
    info!("Shutting down...");

    Ok(())
}
