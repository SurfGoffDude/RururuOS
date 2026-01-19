use crate::app::Message;
use iced::widget::{column, pick_list, row, slider, text, toggler, Space};
use iced::{Element, Length};

pub struct DisplaysPage {
    pub resolution: String,
    pub refresh_rate: u32,
    pub scale: f32,
    pub night_light: bool,
    pub night_light_temp: u32,
    pub vrr_enabled: bool,
}

impl DisplaysPage {
    pub fn new() -> Self {
        Self {
            resolution: "3840x2160".to_string(),
            refresh_rate: 60,
            scale: 1.5,
            night_light: true,
            night_light_temp: 4000,
            vrr_enabled: true,
        }
    }

    pub fn set_night_light(&mut self, enabled: bool) {
        self.night_light = enabled;
    }

    pub fn set_scale(&mut self, scale: f32) {
        self.scale = scale;
    }

    pub fn view(&self) -> Element<Message> {
        let resolutions = vec![
            "3840x2160".to_string(),
            "2560x1440".to_string(),
            "1920x1080".to_string(),
            "1920x1200".to_string(),
        ];

        let refresh_rates = vec![
            "60 Hz".to_string(),
            "120 Hz".to_string(),
            "144 Hz".to_string(),
            "165 Hz".to_string(),
            "240 Hz".to_string(),
        ];

        column![
            // Resolution
            text("Display").size(16),
            Space::with_height(Length::Fixed(8.0)),

            row![
                text("Resolution"),
                Space::with_width(Length::Fill),
                pick_list(
                    resolutions,
                    Some(self.resolution.clone()),
                    Message::ResolutionChanged
                ),
            ]
            .align_items(iced::Alignment::Center)
            .padding(8),

            row![
                text("Refresh rate"),
                Space::with_width(Length::Fill),
                pick_list(
                    refresh_rates,
                    Some(format!("{} Hz", self.refresh_rate)),
                    |s| {
                        let rate = s.trim_end_matches(" Hz").parse().unwrap_or(60);
                        Message::RefreshRateChanged(rate)
                    }
                ),
            ]
            .align_items(iced::Alignment::Center)
            .padding(8),

            row![
                text("Scale"),
                Space::with_width(Length::Fill),
                slider(1.0..=3.0, self.scale, Message::ScaleChanged)
                    .step(0.25)
                    .width(Length::Fixed(200.0)),
                Space::with_width(Length::Fixed(8.0)),
                text(format!("{}%", (self.scale * 100.0) as u32)),
            ]
            .align_items(iced::Alignment::Center)
            .padding(8),

            Space::with_height(Length::Fixed(24.0)),

            // Night Light
            text("Night Light").size(16),
            Space::with_height(Length::Fixed(8.0)),

            row![
                text("Enable Night Light"),
                Space::with_width(Length::Fill),
                toggler(None, self.night_light, Message::NightLightToggled),
            ]
            .align_items(iced::Alignment::Center)
            .padding(8),

            row![
                text("Color temperature"),
                Space::with_width(Length::Fill),
                text(format!("{}K", self.night_light_temp)),
            ]
            .align_items(iced::Alignment::Center)
            .padding(8),

            Space::with_height(Length::Fixed(24.0)),

            // Advanced
            text("Advanced").size(16),
            Space::with_height(Length::Fixed(8.0)),

            row![
                text("Variable Refresh Rate (VRR)"),
                Space::with_width(Length::Fill),
                text(if self.vrr_enabled { "Enabled" } else { "Disabled" })
                    .style(iced::theme::Text::Color(
                        if self.vrr_enabled {
                            iced::Color::from_rgb(0.6, 0.8, 0.6)
                        } else {
                            iced::Color::from_rgb(0.8, 0.6, 0.6)
                        }
                    )),
            ]
            .align_items(iced::Alignment::Center)
            .padding(8),

            row![
                text("Color depth"),
                Space::with_width(Length::Fill),
                text("10-bit"),
            ]
            .padding(8),
        ]
        .spacing(4)
        .into()
    }
}
