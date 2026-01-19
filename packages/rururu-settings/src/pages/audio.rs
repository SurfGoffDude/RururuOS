use crate::app::Message;
use iced::widget::{column, pick_list, row, slider, text, toggler, Space};
use iced::{Element, Length};

pub struct AudioPage {
    pub output_volume: f32,
    pub input_volume: f32,
    pub output_device: String,
    pub input_device: String,
    pub output_muted: bool,
    pub input_muted: bool,
}

impl AudioPage {
    pub fn new() -> Self {
        Self {
            output_volume: 75.0,
            input_volume: 50.0,
            output_device: "Built-in Audio".to_string(),
            input_device: "Built-in Microphone".to_string(),
            output_muted: false,
            input_muted: false,
        }
    }

    pub fn set_output_volume(&mut self, vol: f32) {
        self.output_volume = vol;
    }

    pub fn set_input_volume(&mut self, vol: f32) {
        self.input_volume = vol;
    }

    pub fn view(&self) -> Element<Message> {
        let output_devices = vec![
            "Built-in Audio".to_string(),
            "HDMI Audio".to_string(),
            "USB Audio".to_string(),
            "Bluetooth Headphones".to_string(),
        ];

        let input_devices = vec![
            "Built-in Microphone".to_string(),
            "USB Microphone".to_string(),
            "Webcam Microphone".to_string(),
        ];

        column![
            // Output section
            text("Output").size(16),
            Space::with_height(Length::Fixed(8.0)),
            row![
                text("Output device"),
                Space::with_width(Length::Fill),
                pick_list(
                    output_devices,
                    Some(self.output_device.clone()),
                    Message::OutputDeviceChanged
                ),
            ]
            .align_items(iced::Alignment::Center)
            .padding(8),
            row![
                text("🔊"),
                Space::with_width(Length::Fixed(8.0)),
                slider(
                    0.0..=100.0,
                    self.output_volume,
                    Message::OutputVolumeChanged
                )
                .width(Length::Fill),
                Space::with_width(Length::Fixed(8.0)),
                text(format!("{}%", self.output_volume as u32)).width(Length::Fixed(50.0)),
            ]
            .align_items(iced::Alignment::Center)
            .padding(8),
            Space::with_height(Length::Fixed(24.0)),
            // Input section
            text("Input").size(16),
            Space::with_height(Length::Fixed(8.0)),
            row![
                text("Input device"),
                Space::with_width(Length::Fill),
                pick_list(
                    input_devices,
                    Some(self.input_device.clone()),
                    Message::InputDeviceChanged
                ),
            ]
            .align_items(iced::Alignment::Center)
            .padding(8),
            row![
                text("🎤"),
                Space::with_width(Length::Fixed(8.0)),
                slider(0.0..=100.0, self.input_volume, Message::InputVolumeChanged)
                    .width(Length::Fill),
                Space::with_width(Length::Fixed(8.0)),
                text(format!("{}%", self.input_volume as u32)).width(Length::Fixed(50.0)),
            ]
            .align_items(iced::Alignment::Center)
            .padding(8),
            Space::with_height(Length::Fixed(24.0)),
            // PipeWire info
            text("Audio System").size(16),
            Space::with_height(Length::Fixed(8.0)),
            row![
                text("Audio server"),
                Space::with_width(Length::Fill),
                text("PipeWire").style(iced::theme::Text::Color(iced::Color::from_rgb(
                    0.6, 0.8, 0.6
                ))),
            ]
            .padding(8),
            row![
                text("Sample rate"),
                Space::with_width(Length::Fill),
                text("48000 Hz"),
            ]
            .padding(8),
            row![
                text("Buffer size"),
                Space::with_width(Length::Fill),
                text("256 samples (5.3ms)"),
            ]
            .padding(8),
        ]
        .spacing(4)
        .into()
    }
}
