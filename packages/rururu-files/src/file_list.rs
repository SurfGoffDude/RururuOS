use crate::app::{Message, ViewMode};
use iced::widget::{button, column, container, row, scrollable, text, Space};
use iced::{Element, Length};
use std::path::PathBuf;
use std::time::SystemTime;

#[derive(Debug, Clone)]
pub struct FileEntry {
    pub name: String,
    pub path: PathBuf,
    pub is_dir: bool,
    pub size: u64,
    pub modified: Option<SystemTime>,
    pub file_type: String,
}

pub struct FileList;

impl FileList {
    pub fn view<'a>(
        files: &'a [FileEntry],
        selected: &'a Option<PathBuf>,
        view_mode: ViewMode,
    ) -> Element<'a, Message> {
        match view_mode {
            ViewMode::List => Self::list_view(files, selected),
            ViewMode::Grid => Self::grid_view(files, selected),
            ViewMode::Columns => Self::list_view(files, selected), // TODO: implement columns
        }
    }

    fn list_view<'a>(
        files: &'a [FileEntry],
        selected: &'a Option<PathBuf>,
    ) -> Element<'a, Message> {
        let header = row![
            text("Name").width(Length::FillPortion(4)),
            text("Size").width(Length::FillPortion(1)),
            text("Modified").width(Length::FillPortion(2)),
            text("Type").width(Length::FillPortion(1)),
        ]
        .spacing(8)
        .padding(8);

        let rows: Vec<Element<Message>> = files
            .iter()
            .map(|entry| {
                let is_selected = selected
                    .as_ref()
                    .map(|s| s == &entry.path)
                    .unwrap_or(false);

                let icon = if entry.is_dir { "📁" } else { Self::file_icon(&entry.file_type) };

                let size_str = if entry.is_dir {
                    "—".to_string()
                } else {
                    humansize::format_size(entry.size, humansize::BINARY)
                };

                let modified_str = entry
                    .modified
                    .and_then(|t| {
                        t.duration_since(SystemTime::UNIX_EPOCH)
                            .ok()
                            .map(|d| {
                                chrono::DateTime::from_timestamp(d.as_secs() as i64, 0)
                                    .map(|dt| dt.format("%Y-%m-%d %H:%M").to_string())
                                    .unwrap_or_default()
                            })
                    })
                    .unwrap_or_else(|| "—".to_string());

                let path = entry.path.clone();
                let path2 = entry.path.clone();

                let row_content = row![
                    text(format!("{} {}", icon, entry.name)).width(Length::FillPortion(4)),
                    text(size_str).width(Length::FillPortion(1)),
                    text(modified_str).width(Length::FillPortion(2)),
                    text(&entry.file_type).width(Length::FillPortion(1)),
                ]
                .spacing(8)
                .padding(4);

                let style = if is_selected {
                    iced::theme::Button::Primary
                } else {
                    iced::theme::Button::Text
                };

                button(row_content)
                    .style(style)
                    .width(Length::Fill)
                    .on_press(Message::FileSelected(path))
                    .into()
            })
            .collect();

        let content = column![header]
            .push(scrollable(column(rows).spacing(2)))
            .spacing(4);

        container(content)
            .width(Length::FillPortion(3))
            .height(Length::Fill)
            .into()
    }

    fn grid_view<'a>(
        files: &'a [FileEntry],
        selected: &'a Option<PathBuf>,
    ) -> Element<'a, Message> {
        let items: Vec<Element<Message>> = files
            .iter()
            .map(|entry| {
                let is_selected = selected
                    .as_ref()
                    .map(|s| s == &entry.path)
                    .unwrap_or(false);

                let icon = if entry.is_dir { "📁" } else { Self::file_icon(&entry.file_type) };
                let path = entry.path.clone();

                let name = if entry.name.len() > 12 {
                    format!("{}...", &entry.name[..12])
                } else {
                    entry.name.clone()
                };

                let item = column![
                    text(icon).size(32),
                    text(name).size(12),
                ]
                .align_items(iced::Alignment::Center)
                .spacing(4)
                .width(Length::Fixed(100.0))
                .height(Length::Fixed(80.0));

                let style = if is_selected {
                    iced::theme::Button::Primary
                } else {
                    iced::theme::Button::Text
                };

                button(item)
                    .style(style)
                    .on_press(Message::FileSelected(path))
                    .into()
            })
            .collect();

        // Create rows of 6 items each
        let mut rows: Vec<Element<Message>> = Vec::new();
        for chunk in items.chunks(6) {
            let row_items: Vec<Element<Message>> = chunk.to_vec();
            rows.push(row(row_items).spacing(8).into());
        }

        let content = scrollable(column(rows).spacing(8));

        container(content)
            .width(Length::FillPortion(3))
            .height(Length::Fill)
            .into()
    }

    fn file_icon(file_type: &str) -> &'static str {
        match file_type.to_lowercase().as_str() {
            // Images
            "png" | "jpg" | "jpeg" | "gif" | "webp" | "bmp" | "svg" | "tiff" => "🖼️",
            "psd" | "xcf" | "kra" => "🎨",
            "cr2" | "cr3" | "nef" | "arw" | "dng" | "raw" => "📷",
            "exr" | "hdr" => "✨",
            
            // Video
            "mp4" | "mkv" | "avi" | "mov" | "webm" | "flv" => "🎬",
            
            // Audio
            "mp3" | "flac" | "wav" | "ogg" | "m4a" | "aac" => "🎵",
            
            // 3D
            "gltf" | "glb" | "obj" | "fbx" | "blend" | "stl" => "🧊",
            
            // Documents
            "pdf" => "📄",
            "doc" | "docx" | "odt" => "📝",
            "xls" | "xlsx" | "ods" => "📊",
            "ppt" | "pptx" | "odp" => "📽️",
            
            // Code
            "rs" | "py" | "js" | "ts" | "c" | "cpp" | "h" | "go" | "java" => "💻",
            "html" | "css" | "scss" => "🌐",
            "json" | "yaml" | "yml" | "toml" | "xml" => "⚙️",
            "sh" | "bash" | "zsh" | "fish" => "🐚",
            
            // Archives
            "zip" | "tar" | "gz" | "bz2" | "xz" | "7z" | "rar" => "📦",
            
            // Text
            "txt" | "md" | "rst" => "📃",
            
            // Executables
            "exe" | "msi" | "deb" | "rpm" | "appimage" => "⚡",
            
            _ => "📄",
        }
    }
}
