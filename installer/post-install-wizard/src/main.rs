mod wizard;
mod pages;

use iced::{Application, Settings, window};

fn main() -> iced::Result {
    tracing_subscriber::fmt::init();
    
    wizard::SetupWizard::run(Settings {
        window: window::Settings {
            size: iced::Size::new(900.0, 650.0),
            resizable: false,
            decorations: true,
            ..Default::default()
        },
        antialiasing: true,
        ..Default::default()
    })
}
