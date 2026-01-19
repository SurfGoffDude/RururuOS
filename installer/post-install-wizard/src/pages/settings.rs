use iced::{
    widget::{checkbox, column, container, row, text, toggler, vertical_space},
    Element, Length,
};
use crate::wizard::Message;

pub struct SettingsPage {
    pub dark_mode: bool,
    pub auto_updates: bool,
    pub telemetry: bool,
}

impl SettingsPage {
    pub fn new() -> Self {
        Self {
            dark_mode: true,
            auto_updates: true,
            telemetry: false,
        }
    }
    
    pub fn view(&self) -> Element<Message> {
        container(
            column![
                text("System Settings").size(24),
                vertical_space().height(30),
                
                text("Appearance").size(18),
                row![
                    text("Dark Mode").width(200),
                    toggler(
                        String::new(),
                        self.dark_mode,
                        Message::ToggleDarkMode,
                    ),
                ]
                .spacing(20),
                
                vertical_space().height(30),
                
                text("Updates").size(18),
                row![
                    text("Automatic Updates").width(200),
                    toggler(
                        String::new(),
                        self.auto_updates,
                        Message::ToggleAutoUpdates,
                    ),
                ]
                .spacing(20),
                text("Keep your system secure with automatic updates").size(12),
                
                vertical_space().height(30),
                
                text("Privacy").size(18),
                row![
                    text("Usage Statistics").width(200),
                    toggler(
                        String::new(),
                        self.telemetry,
                        Message::ToggleTelemetry,
                    ),
                ]
                .spacing(20),
                text("Help improve RururuOS by sending anonymous usage data").size(12),
                
                vertical_space().height(30),
                
                text("These settings can be changed later in System Settings.").size(12),
            ]
            .spacing(10)
        )
        .width(Length::Fill)
        .into()
    }
}
