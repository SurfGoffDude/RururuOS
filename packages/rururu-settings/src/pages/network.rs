use crate::app::Message;
use iced::widget::{button, column, row, text, toggler, Space};
use iced::{Element, Length};

pub struct NetworkPage {
    pub wifi_enabled: bool,
    pub connected_network: Option<String>,
    pub available_networks: Vec<(String, u8, bool)>, // (name, signal, secured)
}

impl NetworkPage {
    pub fn new() -> Self {
        Self {
            wifi_enabled: true,
            connected_network: Some("HomeNetwork".to_string()),
            available_networks: vec![
                ("HomeNetwork".to_string(), 90, true),
                ("Office_5G".to_string(), 75, true),
                ("Guest".to_string(), 60, false),
                ("Neighbor".to_string(), 30, true),
            ],
        }
    }

    pub fn view(&self) -> Element<Message> {
        let network_items: Vec<Element<Message>> =
            self.available_networks
                .iter()
                .map(|(name, signal, secured)| {
                    let is_connected = self.connected_network.as_ref() == Some(name);
                    let signal_icon = match signal {
                        80..=100 => "📶",
                        50..=79 => "📶",
                        20..=49 => "📶",
                        _ => "📶",
                    };

                    row![
                        text(signal_icon),
                        Space::with_width(Length::Fixed(8.0)),
                        column![
                            text(name).size(14),
                            text(if is_connected {
                                "Connected"
                            } else {
                                if *secured {
                                    "Secured"
                                } else {
                                    "Open"
                                }
                            })
                            .size(11)
                            .style(iced::theme::Text::Color(if is_connected {
                                iced::Color::from_rgb(0.6, 0.8, 0.6)
                            } else {
                                iced::Color::from_rgb(0.6, 0.6, 0.6)
                            })),
                        ],
                        Space::with_width(Length::Fill),
                        if *secured { text("🔒") } else { text("") },
                        Space::with_width(Length::Fixed(8.0)),
                        if is_connected {
                            button(text("Disconnect"))
                                .style(iced::theme::Button::Secondary)
                                .on_press(Message::WifiConnect(String::new()))
                        } else {
                            button(text("Connect"))
                                .style(iced::theme::Button::Primary)
                                .on_press(Message::WifiConnect(name.clone()))
                        },
                    ]
                    .align_items(iced::Alignment::Center)
                    .padding(8)
                    .into()
                })
                .collect();

        column![
            // WiFi toggle
            row![
                text("Wi-Fi"),
                Space::with_width(Length::Fill),
                toggler(None, self.wifi_enabled, Message::WifiToggled),
            ]
            .align_items(iced::Alignment::Center)
            .padding(8),
            Space::with_height(Length::Fixed(16.0)),
            // Networks list
            text("Available Networks").size(16),
            Space::with_height(Length::Fixed(8.0)),
            column(network_items).spacing(4),
            Space::with_height(Length::Fixed(24.0)),
            // Wired connection
            text("Wired").size(16),
            Space::with_height(Length::Fixed(8.0)),
            row![
                text("🔌"),
                Space::with_width(Length::Fixed(8.0)),
                column![
                    text("Ethernet").size(14),
                    text("Connected - 1 Gbps").size(11),
                ],
                Space::with_width(Length::Fill),
                text("192.168.1.100"),
            ]
            .align_items(iced::Alignment::Center)
            .padding(8),
        ]
        .spacing(4)
        .into()
    }
}
