mod app;
mod batch;
mod file_list;
mod preview;
mod sidebar;
mod tags;
mod toolbar;

use app::RururuFiles;
use iced::{Application, Settings};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

fn main() -> iced::Result {
    tracing_subscriber::registry()
        .with(tracing_subscriber::fmt::layer())
        .with(tracing_subscriber::EnvFilter::from_default_env())
        .init();

    RururuFiles::run(Settings {
        window: iced::window::Settings {
            size: iced::Size::new(1200.0, 800.0),
            min_size: Some(iced::Size::new(800.0, 600.0)),
            ..Default::default()
        },
        antialiasing: true,
        ..Default::default()
    })
}
