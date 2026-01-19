use crate::app::{Message, PreviewData};
use iced::widget::{column, container, image, scrollable, text, Space};
use iced::{Element, Length};
use std::path::PathBuf;

pub struct Preview;

impl Preview {
    pub fn view<'a>(
        data: &'a PreviewData,
        selected: &'a Option<PathBuf>,
    ) -> Element<'a, Message> {
        let content = match data {
            PreviewData::Image(bytes) => {
                let handle = image::Handle::from_memory(bytes.clone());
                column![
                    Self::header(selected),
                    image(handle)
                        .width(Length::Fill)
                        .height(Length::Fill),
                ]
                .spacing(8)
            }
            PreviewData::Text(content) => {
                column![
                    Self::header(selected),
                    scrollable(
                        text(content)
                            .font(iced::Font::MONOSPACE)
                            .size(12)
                    )
                    .height(Length::Fill),
                ]
                .spacing(8)
            }
            PreviewData::Metadata(json) => {
                let formatted = serde_json::to_string_pretty(json).unwrap_or_default();
                column![
                    Self::header(selected),
                    scrollable(
                        text(formatted)
                            .font(iced::Font::MONOSPACE)
                            .size(12)
                    )
                    .height(Length::Fill),
                ]
                .spacing(8)
            }
            PreviewData::None => {
                if let Some(path) = selected {
                    column![
                        Self::header(selected),
                        Space::with_height(Length::Fill),
                        text("No preview available").size(14),
                        Space::with_height(Length::Fill),
                    ]
                    .align_items(iced::Alignment::Center)
                } else {
                    column![
                        Space::with_height(Length::Fill),
                        text("Select a file to preview").size(14),
                        Space::with_height(Length::Fill),
                    ]
                    .align_items(iced::Alignment::Center)
                }
            }
        };

        container(content)
            .width(Length::FillPortion(2))
            .height(Length::Fill)
            .padding(8)
            .style(iced::theme::Container::Box)
            .into()
    }

    fn header<'a>(selected: &'a Option<PathBuf>) -> Element<'a, Message> {
        if let Some(path) = selected {
            let name = path
                .file_name()
                .and_then(|n| n.to_str())
                .unwrap_or("Unknown");

            column![
                text(name).size(16),
                text(path.to_string_lossy()).size(10),
            ]
            .spacing(4)
            .into()
        } else {
            Space::with_height(Length::Shrink).into()
        }
    }
}
