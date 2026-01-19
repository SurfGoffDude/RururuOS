use crate::app::Message;
use iced::widget::{button, column, container, row, scrollable, text, text_input, Space};
use iced::{Element, Length};
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use std::path::{Path, PathBuf};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TagDatabase {
    tags: HashMap<String, TagInfo>,
    file_tags: HashMap<PathBuf, HashSet<String>>,
    #[serde(skip)]
    db_path: PathBuf,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TagInfo {
    pub name: String,
    pub color: TagColor,
    pub description: Option<String>,
    pub file_count: usize,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum TagColor {
    Red,
    Orange,
    Yellow,
    Green,
    Blue,
    Purple,
    Pink,
    Gray,
}

impl TagColor {
    pub fn to_rgb(&self) -> [u8; 3] {
        match self {
            TagColor::Red => [247, 118, 142],
            TagColor::Orange => [255, 158, 100],
            TagColor::Yellow => [224, 175, 104],
            TagColor::Green => [158, 206, 106],
            TagColor::Blue => [122, 162, 247],
            TagColor::Purple => [187, 154, 247],
            TagColor::Pink => [255, 121, 198],
            TagColor::Gray => [128, 128, 128],
        }
    }

    pub fn all() -> &'static [TagColor] {
        &[
            TagColor::Red,
            TagColor::Orange,
            TagColor::Yellow,
            TagColor::Green,
            TagColor::Blue,
            TagColor::Purple,
            TagColor::Pink,
            TagColor::Gray,
        ]
    }
}

impl Default for TagDatabase {
    fn default() -> Self {
        Self::new()
    }
}

impl TagDatabase {
    pub fn new() -> Self {
        let db_path = dirs::data_dir()
            .unwrap_or_else(|| PathBuf::from("."))
            .join("rururu-files")
            .join("tags.json");

        Self {
            tags: HashMap::new(),
            file_tags: HashMap::new(),
            db_path,
        }
    }

    pub fn load() -> Self {
        let db_path = dirs::data_dir()
            .unwrap_or_else(|| PathBuf::from("."))
            .join("rururu-files")
            .join("tags.json");

        if db_path.exists() {
            if let Ok(content) = std::fs::read_to_string(&db_path) {
                if let Ok(mut db) = serde_json::from_str::<TagDatabase>(&content) {
                    db.db_path = db_path;
                    return db;
                }
            }
        }

        let mut db = Self::new();
        db.db_path = db_path;
        db
    }

    pub fn save(&self) -> std::io::Result<()> {
        if let Some(parent) = self.db_path.parent() {
            std::fs::create_dir_all(parent)?;
        }
        let content = serde_json::to_string_pretty(self)?;
        std::fs::write(&self.db_path, content)
    }

    pub fn create_tag(&mut self, name: &str, color: TagColor) {
        if !self.tags.contains_key(name) {
            self.tags.insert(
                name.to_string(),
                TagInfo {
                    name: name.to_string(),
                    color,
                    description: None,
                    file_count: 0,
                },
            );
        }
    }

    pub fn delete_tag(&mut self, name: &str) {
        self.tags.remove(name);
        for tags in self.file_tags.values_mut() {
            tags.remove(name);
        }
    }

    pub fn add_tag_to_file(&mut self, path: &Path, tag: &str) {
        if !self.tags.contains_key(tag) {
            self.create_tag(tag, TagColor::Blue);
        }

        let tags = self.file_tags.entry(path.to_path_buf()).or_default();
        if tags.insert(tag.to_string()) {
            if let Some(info) = self.tags.get_mut(tag) {
                info.file_count += 1;
            }
        }
    }

    pub fn remove_tag_from_file(&mut self, path: &Path, tag: &str) {
        if let Some(tags) = self.file_tags.get_mut(path) {
            if tags.remove(tag) {
                if let Some(info) = self.tags.get_mut(tag) {
                    info.file_count = info.file_count.saturating_sub(1);
                }
            }
        }
    }

    pub fn get_file_tags(&self, path: &Path) -> Vec<&TagInfo> {
        self.file_tags
            .get(path)
            .map(|tags| tags.iter().filter_map(|t| self.tags.get(t)).collect())
            .unwrap_or_default()
    }

    pub fn get_all_tags(&self) -> Vec<&TagInfo> {
        self.tags.values().collect()
    }

    pub fn get_files_with_tag(&self, tag: &str) -> Vec<&PathBuf> {
        self.file_tags
            .iter()
            .filter(|(_, tags)| tags.contains(tag))
            .map(|(path, _)| path)
            .collect()
    }

    pub fn search_by_tags(&self, required_tags: &[&str]) -> Vec<&PathBuf> {
        self.file_tags
            .iter()
            .filter(|(_, tags)| required_tags.iter().all(|t| tags.contains(*t)))
            .map(|(path, _)| path)
            .collect()
    }
}

#[derive(Debug, Clone)]
pub struct TagPanel {
    pub visible: bool,
    pub new_tag_input: String,
    pub selected_color: TagColor,
    pub filter_tags: HashSet<String>,
}

impl Default for TagPanel {
    fn default() -> Self {
        Self {
            visible: false,
            new_tag_input: String::new(),
            selected_color: TagColor::Blue,
            filter_tags: HashSet::new(),
        }
    }
}

impl TagPanel {
    pub fn view<'a>(
        &'a self,
        db: &'a TagDatabase,
        selected_file: Option<&'a Path>,
    ) -> Element<'a, Message> {
        if !self.visible {
            return Space::new(Length::Shrink, Length::Shrink).into();
        }

        let header = row![
            text("Tags").size(16),
            Space::with_width(Length::Fill),
            button(text("×"))
                .style(iced::theme::Button::Text)
                .on_press(Message::ToggleTagPanel),
        ]
        .align_items(iced::Alignment::Center);

        // New tag input
        let new_tag_row = row![
            text_input("New tag...", &self.new_tag_input)
                .on_input(Message::TagInputChanged)
                .width(Length::Fill),
            button(text("+"))
                .style(iced::theme::Button::Primary)
                .on_press(Message::CreateTag),
        ]
        .spacing(4);

        // Color picker
        let colors: Vec<Element<Message>> = TagColor::all()
            .iter()
            .map(|color| {
                let rgb = color.to_rgb();
                let is_selected = *color == self.selected_color;

                button(
                    container(Space::new(Length::Fixed(20.0), Length::Fixed(20.0)))
                        .style(iced::theme::Container::Box),
                )
                .style(if is_selected {
                    iced::theme::Button::Primary
                } else {
                    iced::theme::Button::Secondary
                })
                .on_press(Message::TagColorSelected(*color))
                .into()
            })
            .collect();

        let color_row = row(colors).spacing(4);

        // All tags list
        let all_tags: Vec<Element<Message>> = db
            .get_all_tags()
            .iter()
            .map(|tag| {
                let rgb = tag.color.to_rgb();
                let is_filter = self.filter_tags.contains(&tag.name);

                row![
                    container(Space::new(Length::Fixed(8.0), Length::Fixed(8.0)))
                        .style(iced::theme::Container::Box),
                    text(&tag.name).size(13),
                    Space::with_width(Length::Fill),
                    text(format!("({})", tag.file_count)).size(11),
                    button(text(if is_filter { "✓" } else { "○" }).size(12))
                        .style(iced::theme::Button::Text)
                        .on_press(Message::ToggleTagFilter(tag.name.clone())),
                ]
                .spacing(4)
                .align_items(iced::Alignment::Center)
                .into()
            })
            .collect();

        // File tags (if file selected)
        let file_tags_section: Element<Message> = if let Some(path) = selected_file {
            let file_tags = db.get_file_tags(path);
            let tags_list: Vec<Element<Message>> = file_tags
                .iter()
                .map(|tag| {
                    row![
                        text(&tag.name).size(12),
                        button(text("×").size(10))
                            .style(iced::theme::Button::Text)
                            .on_press(Message::RemoveTagFromFile(tag.name.clone())),
                    ]
                    .spacing(2)
                    .into()
                })
                .collect();

            column![
                text("File Tags").size(14),
                if tags_list.is_empty() {
                    column![text("No tags").size(11)]
                } else {
                    column(tags_list).spacing(2)
                },
            ]
            .spacing(4)
            .into()
        } else {
            text("Select a file to manage tags").size(11).into()
        };

        container(
            column![
                header,
                Space::with_height(Length::Fixed(8.0)),
                new_tag_row,
                color_row,
                Space::with_height(Length::Fixed(12.0)),
                text("Filter by Tags").size(14),
                scrollable(column(all_tags).spacing(4)).height(Length::Fixed(150.0)),
                Space::with_height(Length::Fixed(12.0)),
                file_tags_section,
            ]
            .spacing(8)
            .padding(12),
        )
        .width(Length::Fixed(220.0))
        .height(Length::Fill)
        .style(iced::theme::Container::Box)
        .into()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileMetadata {
    pub path: PathBuf,
    pub size: u64,
    pub created: Option<u64>,
    pub modified: Option<u64>,
    pub mime_type: Option<String>,
    pub dimensions: Option<(u32, u32)>,
    pub duration: Option<f64>,
    pub author: Option<String>,
    pub title: Option<String>,
    pub description: Option<String>,
    pub custom: HashMap<String, String>,
}

impl FileMetadata {
    pub fn from_path(path: &Path) -> std::io::Result<Self> {
        let metadata = std::fs::metadata(path)?;

        let created = metadata
            .created()
            .ok()
            .and_then(|t| t.duration_since(std::time::UNIX_EPOCH).ok())
            .map(|d| d.as_secs());

        let modified = metadata
            .modified()
            .ok()
            .and_then(|t| t.duration_since(std::time::UNIX_EPOCH).ok())
            .map(|d| d.as_secs());

        let mime_type = infer::get_from_path(path)
            .ok()
            .flatten()
            .map(|t| t.mime_type().to_string());

        Ok(Self {
            path: path.to_path_buf(),
            size: metadata.len(),
            created,
            modified,
            mime_type,
            dimensions: None,
            duration: None,
            author: None,
            title: None,
            description: None,
            custom: HashMap::new(),
        })
    }

    pub fn format_size(&self) -> String {
        const KB: u64 = 1024;
        const MB: u64 = KB * 1024;
        const GB: u64 = MB * 1024;

        if self.size >= GB {
            format!("{:.2} GB", self.size as f64 / GB as f64)
        } else if self.size >= MB {
            format!("{:.2} MB", self.size as f64 / MB as f64)
        } else if self.size >= KB {
            format!("{:.2} KB", self.size as f64 / KB as f64)
        } else {
            format!("{} B", self.size)
        }
    }

    pub fn format_date(timestamp: u64) -> String {
        use std::time::{Duration, UNIX_EPOCH};
        let datetime = UNIX_EPOCH + Duration::from_secs(timestamp);
        // Simple formatting
        format!("{:?}", datetime)
    }
}

pub fn view_metadata<'a>(metadata: &'a FileMetadata) -> Element<'a, Message> {
    let mut items = vec![("Size", metadata.format_size())];

    if let Some(created) = metadata.created {
        items.push(("Created", FileMetadata::format_date(created)));
    }

    if let Some(modified) = metadata.modified {
        items.push(("Modified", FileMetadata::format_date(modified)));
    }

    if let Some(ref mime) = metadata.mime_type {
        items.push(("Type", mime.clone()));
    }

    if let Some((w, h)) = metadata.dimensions {
        items.push(("Dimensions", format!("{}×{}", w, h)));
    }

    if let Some(duration) = metadata.duration {
        let mins = (duration / 60.0) as u32;
        let secs = (duration % 60.0) as u32;
        items.push(("Duration", format!("{}:{:02}", mins, secs)));
    }

    let rows: Vec<Element<Message>> = items
        .into_iter()
        .map(|(label, value)| {
            row![
                text(label).size(12).width(Length::Fixed(80.0)),
                text(value).size(12),
            ]
            .spacing(8)
            .into()
        })
        .collect();

    column![
        text("Metadata").size(14),
        Space::with_height(Length::Fixed(4.0)),
        column(rows).spacing(4),
    ]
    .spacing(4)
    .into()
}
