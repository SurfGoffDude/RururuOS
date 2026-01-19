use crate::app::Message;
use iced::widget::{button, column, container, pick_list, row, text, toggler, Space};
use iced::{Element, Length};

pub struct AppearancePage {
    pub theme: String,
    pub accent_color: [u8; 3],
    pub font: String,
    pub icon_theme: String,
    pub dark_mode: bool,
}

impl AppearancePage {
    pub fn new() -> Self {
        Self {
            theme: "Tokyo Night".to_string(),
            accent_color: [122, 162, 247], // Blue
            font: "Inter".to_string(),
            icon_theme: "Papirus-Dark".to_string(),
            dark_mode: true,
        }
    }

    pub fn set_theme(&mut self, theme: &str) {
        self.theme = theme.to_string();
    }

    pub fn set_accent_color(&mut self, color: [u8; 3]) {
        self.accent_color = color;
    }

    pub fn set_font(&mut self, font: &str) {
        self.font = font.to_string();
    }

    pub fn set_icon_theme(&mut self, theme: &str) {
        self.icon_theme = theme.to_string();
    }

    pub fn view(&self) -> Element<Message> {
        let themes = vec![
            "Tokyo Night".to_string(),
            "Dracula".to_string(),
            "Nord".to_string(),
            "Catppuccin".to_string(),
            "Adwaita Dark".to_string(),
            "Breeze Dark".to_string(),
        ];

        let fonts = vec![
            "Inter".to_string(),
            "Roboto".to_string(),
            "Noto Sans".to_string(),
            "Ubuntu".to_string(),
            "Cantarell".to_string(),
        ];

        let icon_themes = vec![
            "Papirus-Dark".to_string(),
            "Papirus".to_string(),
            "Adwaita".to_string(),
            "Breeze".to_string(),
            "Numix".to_string(),
        ];

        let accent_colors = [
            ("Blue", [122, 162, 247]),
            ("Purple", [187, 154, 247]),
            ("Cyan", [125, 207, 255]),
            ("Green", [158, 206, 106]),
            ("Yellow", [224, 175, 104]),
            ("Red", [247, 118, 142]),
            ("Pink", [255, 121, 198]),
        ];

        let color_buttons: Vec<Element<Message>> = accent_colors
            .iter()
            .map(|(name, color)| {
                let is_selected = self.accent_color == *color;
                let color_clone = *color;

                button(
                    container(Space::new(Length::Fixed(24.0), Length::Fixed(24.0)))
                        .style(iced::theme::Container::Box),
                )
                .on_press(Message::AccentColorChanged(color_clone))
                .into()
            })
            .collect();

        column![
            // Theme section
            Self::section("Theme"),
            row![
                text("Color scheme"),
                Space::with_width(Length::Fill),
                pick_list(
                    themes.clone(),
                    Some(self.theme.clone()),
                    Message::ThemeChanged
                ),
            ]
            .align_items(iced::Alignment::Center)
            .padding(8),
            Space::with_height(Length::Fixed(8.0)),
            // Accent color
            row![
                text("Accent color"),
                Space::with_width(Length::Fill),
                row(color_buttons).spacing(8),
            ]
            .align_items(iced::Alignment::Center)
            .padding(8),
            Space::with_height(Length::Fixed(16.0)),
            // Font section
            Self::section("Fonts"),
            row![
                text("Interface font"),
                Space::with_width(Length::Fill),
                pick_list(fonts, Some(self.font.clone()), Message::FontChanged),
            ]
            .align_items(iced::Alignment::Center)
            .padding(8),
            Space::with_height(Length::Fixed(16.0)),
            // Icons section
            Self::section("Icons"),
            row![
                text("Icon theme"),
                Space::with_width(Length::Fill),
                pick_list(
                    icon_themes,
                    Some(self.icon_theme.clone()),
                    Message::IconThemeChanged
                ),
            ]
            .align_items(iced::Alignment::Center)
            .padding(8),
        ]
        .spacing(4)
        .into()
    }

    fn section(title: &str) -> Element<'static, Message> {
        text(title).size(16).into()
    }
}
