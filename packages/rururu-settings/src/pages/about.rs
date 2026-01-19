use crate::app::Message;
use iced::widget::{button, column, row, text, Space};
use iced::{Element, Length};

pub struct AboutPage {
    pub os_name: String,
    pub os_version: String,
    pub kernel: String,
    pub desktop: String,
    pub cpu: String,
    pub memory: String,
    pub gpu: String,
}

impl AboutPage {
    pub fn new() -> Self {
        Self {
            os_name: "RururuOS".to_string(),
            os_version: "0.1.0 (Alpha)".to_string(),
            kernel: "Linux 6.7.0-rururu".to_string(),
            desktop: "Sway 1.9".to_string(),
            cpu: "AMD Ryzen 9 7950X".to_string(),
            memory: "64 GB DDR5-6000".to_string(),
            gpu: "NVIDIA RTX 4090".to_string(),
        }
    }

    pub fn view(&self) -> Element<Message> {
        column![
            // Logo and name
            row![
                text("🦊").size(64),
                Space::with_width(Length::Fixed(16.0)),
                column![
                    text(&self.os_name).size(32),
                    text(&self.os_version).size(14),
                    text("Creative Workstation OS").size(12)
                        .style(iced::theme::Text::Color(iced::Color::from_rgb(0.6, 0.6, 0.6))),
                ],
            ]
            .align_items(iced::Alignment::Center)
            .padding(16),

            Space::with_height(Length::Fixed(24.0)),

            // System info
            text("System Information").size(16),
            Space::with_height(Length::Fixed(8.0)),

            Self::info_row("Operating System", &self.os_name),
            Self::info_row("Version", &self.os_version),
            Self::info_row("Kernel", &self.kernel),
            Self::info_row("Desktop", &self.desktop),

            Space::with_height(Length::Fixed(24.0)),

            // Hardware
            text("Hardware").size(16),
            Space::with_height(Length::Fixed(8.0)),

            Self::info_row("Processor", &self.cpu),
            Self::info_row("Memory", &self.memory),
            Self::info_row("Graphics", &self.gpu),

            Space::with_height(Length::Fixed(24.0)),

            // Actions
            row![
                button(text("Copy System Info"))
                    .style(iced::theme::Button::Secondary)
                    .on_press(Message::CopySystemInfo),
                Space::with_width(Length::Fixed(8.0)),
                button(text("Check for Updates"))
                    .style(iced::theme::Button::Primary),
            ],

            Space::with_height(Length::Fixed(24.0)),

            // Credits
            text("Credits").size(16),
            Space::with_height(Length::Fixed(8.0)),

            text("Built with Rust, Iced, and ❤️").size(12)
                .style(iced::theme::Text::Color(iced::Color::from_rgb(0.6, 0.6, 0.6))),
            text("Based on Arch Linux").size(12)
                .style(iced::theme::Text::Color(iced::Color::from_rgb(0.6, 0.6, 0.6))),
            text("© 2024 RururuOS Team").size(12)
                .style(iced::theme::Text::Color(iced::Color::from_rgb(0.6, 0.6, 0.6))),
        ]
        .spacing(4)
        .into()
    }

    fn info_row<'a>(label: &'a str, value: &'a str) -> Element<'a, Message> {
        row![
            text(label).width(Length::Fixed(150.0)),
            text(value),
        ]
        .padding(8)
        .into()
    }
}
