use crate::app::Message;
use iced::widget::{button, column, row, text, Space};
use iced::{Element, Length};

pub struct KeyboardPage {
    pub layouts: Vec<String>,
    pub current_layout: String,
    pub shortcuts: Vec<(String, String, String)>, // (name, keys, action)
}

impl KeyboardPage {
    pub fn new() -> Self {
        Self {
            layouts: vec!["US".to_string(), "RU".to_string()],
            current_layout: "US".to_string(),
            shortcuts: vec![
                ("Terminal".to_string(), "Super+Return".to_string(), "Open terminal".to_string()),
                ("Files".to_string(), "Super+N".to_string(), "Open file manager".to_string()),
                ("GIMP".to_string(), "Super+G".to_string(), "Open GIMP".to_string()),
                ("Blender".to_string(), "Super+Shift+B".to_string(), "Open Blender".to_string()),
                ("Screenshot".to_string(), "Print".to_string(), "Take screenshot".to_string()),
                ("Area Screenshot".to_string(), "Super+Shift+Print".to_string(), "Area screenshot".to_string()),
            ],
        }
    }

    pub fn view(&self) -> Element<Message> {
        let layout_items: Vec<Element<Message>> = self
            .layouts
            .iter()
            .map(|layout| {
                row![
                    text(layout),
                    Space::with_width(Length::Fill),
                    button(text("Remove"))
                        .style(iced::theme::Button::Destructive)
                        .on_press(Message::LayoutRemoved(layout.clone())),
                ]
                .align_items(iced::Alignment::Center)
                .padding(8)
                .into()
            })
            .collect();

        let shortcut_items: Vec<Element<Message>> = self
            .shortcuts
            .iter()
            .map(|(name, keys, action)| {
                row![
                    column![
                        text(name).size(14),
                        text(action).size(11),
                    ]
                    .width(Length::FillPortion(2)),
                    Space::with_width(Length::Fill),
                    text(keys)
                        .style(iced::theme::Text::Color(iced::Color::from_rgb(0.5, 0.7, 0.9))),
                ]
                .align_items(iced::Alignment::Center)
                .padding(8)
                .into()
            })
            .collect();

        column![
            // Layouts section
            text("Keyboard Layouts").size(16),
            Space::with_height(Length::Fixed(8.0)),
            column(layout_items).spacing(4),
            Space::with_height(Length::Fixed(8.0)),
            button(text("+ Add Layout"))
                .style(iced::theme::Button::Secondary)
                .on_press(Message::LayoutAdded("DE".to_string())),

            Space::with_height(Length::Fixed(24.0)),

            // Shortcuts section
            text("Creative Shortcuts").size(16),
            Space::with_height(Length::Fixed(8.0)),
            column(shortcut_items).spacing(4),

            Space::with_height(Length::Fixed(24.0)),

            // Options
            text("Options").size(16),
            Space::with_height(Length::Fixed(8.0)),

            row![
                text("Switch layout"),
                Space::with_width(Length::Fill),
                text("Alt+Shift"),
            ]
            .padding(8),

            row![
                text("Caps Lock behavior"),
                Space::with_width(Length::Fill),
                text("Escape"),
            ]
            .padding(8),

            row![
                text("Key repeat delay"),
                Space::with_width(Length::Fill),
                text("200ms"),
            ]
            .padding(8),

            row![
                text("Key repeat rate"),
                Space::with_width(Length::Fill),
                text("50/sec"),
            ]
            .padding(8),
        ]
        .spacing(4)
        .into()
    }
}
