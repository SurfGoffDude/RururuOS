mod app;
mod calibration;
mod icc;
mod patterns;

use app::ColorCalApp;
use iced::{Application, Settings};

fn main() -> iced::Result {
    ColorCalApp::run(Settings {
        window: iced::window::Settings {
            size: iced::Size::new(900.0, 650.0),
            min_size: Some(iced::Size::new(800.0, 600.0)),
            ..Default::default()
        },
        antialiasing: true,
        ..Default::default()
    })
}
