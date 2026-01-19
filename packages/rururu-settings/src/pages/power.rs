use crate::app::Message;
use iced::widget::{column, pick_list, row, slider, text, toggler, Space};
use iced::{Element, Length};

pub struct PowerPage {
    pub profile: String,
    pub battery_level: u8,
    pub charging: bool,
    pub auto_suspend: u32,
    pub screen_off: u32,
}

impl PowerPage {
    pub fn new() -> Self {
        Self {
            profile: "Balanced".to_string(),
            battery_level: 85,
            charging: true,
            auto_suspend: 30,
            screen_off: 10,
        }
    }

    pub fn set_profile(&mut self, profile: &str) {
        self.profile = profile.to_string();
    }

    pub fn view(&self) -> Element<Message> {
        let profiles = vec![
            "Performance".to_string(),
            "Balanced".to_string(),
            "Power Saver".to_string(),
        ];

        let battery_icon = match (self.battery_level, self.charging) {
            (_, true) => "🔌",
            (80..=100, _) => "🔋",
            (50..=79, _) => "🔋",
            (20..=49, _) => "🪫",
            _ => "🪫",
        };

        column![
            // Battery status
            text("Battery").size(16),
            Space::with_height(Length::Fixed(8.0)),

            row![
                text(battery_icon).size(32),
                Space::with_width(Length::Fixed(16.0)),
                column![
                    text(format!("{}%", self.battery_level)).size(24),
                    text(if self.charging { "Charging" } else { "On battery" }).size(12),
                ],
                Space::with_width(Length::Fill),
            ]
            .align_items(iced::Alignment::Center)
            .padding(8),

            Space::with_height(Length::Fixed(24.0)),

            // Power profile
            text("Power Profile").size(16),
            Space::with_height(Length::Fixed(8.0)),

            row![
                text("Profile"),
                Space::with_width(Length::Fill),
                pick_list(profiles, Some(self.profile.clone()), Message::PowerProfileChanged),
            ]
            .align_items(iced::Alignment::Center)
            .padding(8),

            // Profile descriptions
            text(match self.profile.as_str() {
                "Performance" => "Maximum performance for demanding creative tasks. Higher power consumption.",
                "Balanced" => "Good balance between performance and battery life.",
                "Power Saver" => "Extended battery life. May reduce performance.",
                _ => "",
            })
            .size(12)
            .style(iced::theme::Text::Color(iced::Color::from_rgb(0.6, 0.6, 0.6))),

            Space::with_height(Length::Fixed(24.0)),

            // Power saving
            text("Power Saving").size(16),
            Space::with_height(Length::Fixed(8.0)),

            row![
                text("Turn off screen after"),
                Space::with_width(Length::Fill),
                text(format!("{} minutes", self.screen_off)),
            ]
            .align_items(iced::Alignment::Center)
            .padding(8),

            row![
                text("Automatic suspend after"),
                Space::with_width(Length::Fill),
                text(format!("{} minutes", self.auto_suspend)),
            ]
            .align_items(iced::Alignment::Center)
            .padding(8),

            Space::with_height(Length::Fixed(24.0)),

            // Advanced
            text("Advanced").size(16),
            Space::with_height(Length::Fixed(8.0)),

            row![
                text("CPU Governor"),
                Space::with_width(Length::Fill),
                text("schedutil"),
            ]
            .padding(8),

            row![
                text("GPU Power Management"),
                Space::with_width(Length::Fill),
                text("Adaptive"),
            ]
            .padding(8),
        ]
        .spacing(4)
        .into()
    }
}
