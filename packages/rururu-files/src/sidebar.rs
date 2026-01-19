use crate::app::Message;
use iced::widget::{button, column, container, scrollable, text, Space};
use iced::{Element, Length};
use std::path::PathBuf;

pub struct Sidebar;

impl Sidebar {
    pub fn view<'a>(bookmarks: &'a [PathBuf], current_path: &'a PathBuf) -> Element<'a, Message> {
        let mut items: Vec<Element<Message>> = Vec::new();

        // Places header
        items.push(text("Places").size(14).into());
        items.push(Space::with_height(Length::Fixed(8.0)).into());

        // Default locations
        let places = [
            ("🏠", "Home", dirs::home_dir()),
            ("📄", "Documents", dirs::document_dir()),
            ("⬇️", "Downloads", dirs::download_dir()),
            ("🖼️", "Pictures", dirs::picture_dir()),
            ("🎬", "Videos", dirs::video_dir()),
            ("🎵", "Music", dirs::audio_dir()),
            ("🖥️", "Desktop", dirs::desktop_dir()),
        ];

        for (icon, name, path_opt) in places {
            if let Some(path) = path_opt {
                if path.exists() {
                    let is_current = &path == current_path;
                    let path_clone = path.clone();

                    let style = if is_current {
                        iced::theme::Button::Primary
                    } else {
                        iced::theme::Button::Text
                    };

                    items.push(
                        button(text(format!("{} {}", icon, name)))
                            .style(style)
                            .width(Length::Fill)
                            .on_press(Message::BookmarkClicked(path_clone))
                            .into(),
                    );
                }
            }
        }

        // Separator
        items.push(Space::with_height(Length::Fixed(16.0)).into());

        // Devices header
        items.push(text("Devices").size(14).into());
        items.push(Space::with_height(Length::Fixed(8.0)).into());

        // Root
        items.push(
            button(text("💽 Computer"))
                .style(iced::theme::Button::Text)
                .width(Length::Fill)
                .on_press(Message::BookmarkClicked(PathBuf::from("/")))
                .into(),
        );

        // Mounted volumes (simplified - would need system integration)
        if PathBuf::from("/mnt").exists() {
            items.push(
                button(text("📁 /mnt"))
                    .style(iced::theme::Button::Text)
                    .width(Length::Fill)
                    .on_press(Message::BookmarkClicked(PathBuf::from("/mnt")))
                    .into(),
            );
        }

        if PathBuf::from("/media").exists() {
            items.push(
                button(text("💾 /media"))
                    .style(iced::theme::Button::Text)
                    .width(Length::Fill)
                    .on_press(Message::BookmarkClicked(PathBuf::from("/media")))
                    .into(),
            );
        }

        // Bookmarks section
        if !bookmarks.is_empty() {
            items.push(Space::with_height(Length::Fixed(16.0)).into());
            items.push(text("Bookmarks").size(14).into());
            items.push(Space::with_height(Length::Fixed(8.0)).into());

            for bookmark in bookmarks {
                if !Self::is_default_place(bookmark) {
                    let name = bookmark
                        .file_name()
                        .and_then(|n| n.to_str())
                        .unwrap_or("Unknown");

                    let is_current = bookmark == current_path;
                    let path_clone = bookmark.clone();

                    let style = if is_current {
                        iced::theme::Button::Primary
                    } else {
                        iced::theme::Button::Text
                    };

                    items.push(
                        button(text(format!("📌 {}", name)))
                            .style(style)
                            .width(Length::Fill)
                            .on_press(Message::BookmarkClicked(path_clone))
                            .into(),
                    );
                }
            }
        }

        let content = scrollable(column(items).spacing(2));

        container(content)
            .width(Length::Fixed(180.0))
            .height(Length::Fill)
            .padding(8)
            .style(iced::theme::Container::Box)
            .into()
    }

    fn is_default_place(path: &PathBuf) -> bool {
        let defaults = [
            dirs::home_dir(),
            dirs::document_dir(),
            dirs::download_dir(),
            dirs::picture_dir(),
            dirs::video_dir(),
            dirs::audio_dir(),
            dirs::desktop_dir(),
        ];

        defaults.iter().any(|d| d.as_ref() == Some(path))
    }
}
