use crate::app::Message;
use iced::widget::{button, column, progress_bar, row, text, Space};
use iced::{Element, Length};

pub struct StoragePage {
    pub disks: Vec<DiskInfo>,
}

pub struct DiskInfo {
    pub name: String,
    pub mount_point: String,
    pub total: u64,
    pub used: u64,
    pub fs_type: String,
}

impl StoragePage {
    pub fn new() -> Self {
        Self {
            disks: vec![
                DiskInfo {
                    name: "NVMe SSD".to_string(),
                    mount_point: "/".to_string(),
                    total: 500 * 1024 * 1024 * 1024,
                    used: 180 * 1024 * 1024 * 1024,
                    fs_type: "ext4".to_string(),
                },
                DiskInfo {
                    name: "Data Drive".to_string(),
                    mount_point: "/home".to_string(),
                    total: 2000 * 1024 * 1024 * 1024,
                    used: 850 * 1024 * 1024 * 1024,
                    fs_type: "btrfs".to_string(),
                },
            ],
        }
    }

    pub fn refresh(&mut self) {
        // Would refresh disk info from system
    }

    pub fn view(&self) -> Element<Message> {
        let disk_items: Vec<Element<Message>> = self
            .disks
            .iter()
            .map(|disk| {
                let usage = disk.used as f32 / disk.total as f32;
                let used_gb = disk.used / (1024 * 1024 * 1024);
                let total_gb = disk.total / (1024 * 1024 * 1024);
                let free_gb = total_gb - used_gb;

                column![
                    row![
                        text("💾"),
                        Space::with_width(Length::Fixed(8.0)),
                        column![
                            text(&disk.name).size(14),
                            text(format!("{} ({})", disk.mount_point, disk.fs_type)).size(11),
                        ],
                        Space::with_width(Length::Fill),
                    ]
                    .align_items(iced::Alignment::Center),

                    Space::with_height(Length::Fixed(8.0)),

                    progress_bar(0.0..=1.0, usage)
                        .height(Length::Fixed(8.0)),

                    Space::with_height(Length::Fixed(4.0)),

                    row![
                        text(format!("{} GB used", used_gb)).size(11),
                        Space::with_width(Length::Fill),
                        text(format!("{} GB free of {} GB", free_gb, total_gb)).size(11),
                    ],
                ]
                .padding(12)
                .into()
            })
            .collect();

        column![
            row![
                text("Storage").size(16),
                Space::with_width(Length::Fill),
                button(text("Refresh"))
                    .style(iced::theme::Button::Secondary)
                    .on_press(Message::RefreshStorage),
            ]
            .align_items(iced::Alignment::Center),

            Space::with_height(Length::Fixed(16.0)),

            column(disk_items).spacing(16),

            Space::with_height(Length::Fixed(24.0)),

            // Storage breakdown
            text("Usage Breakdown").size(16),
            Space::with_height(Length::Fixed(8.0)),

            row![
                text("🎬 Videos"),
                Space::with_width(Length::Fill),
                text("120 GB"),
            ]
            .padding(8),

            row![
                text("🖼️ Images"),
                Space::with_width(Length::Fill),
                text("85 GB"),
            ]
            .padding(8),

            row![
                text("🎵 Audio"),
                Space::with_width(Length::Fill),
                text("45 GB"),
            ]
            .padding(8),

            row![
                text("🧊 3D Projects"),
                Space::with_width(Length::Fill),
                text("200 GB"),
            ]
            .padding(8),

            row![
                text("📄 Documents"),
                Space::with_width(Length::Fill),
                text("15 GB"),
            ]
            .padding(8),

            row![
                text("💻 Applications"),
                Space::with_width(Length::Fill),
                text("25 GB"),
            ]
            .padding(8),
        ]
        .spacing(4)
        .into()
    }
}
