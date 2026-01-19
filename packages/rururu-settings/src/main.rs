mod app;
mod pages;

use app::SettingsApp;
use iced::{Application, Settings};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

fn main() -> iced::Result {
    tracing_subscriber::registry()
        .with(tracing_subscriber::fmt::layer())
        .with(tracing_subscriber::EnvFilter::from_default_env())
        .init();

    SettingsApp::run(Settings {
        window: iced::window::Settings {
            size: iced::Size::new(900.0, 600.0),
            min_size: Some(iced::Size::new(700.0, 500.0)),
            ..Default::default()
        },
        antialiasing: true,
        ..Default::default()
    })
}
