use crate::file_list::{FileEntry, FileList};
use crate::preview::Preview;
use crate::sidebar::Sidebar;
use crate::toolbar::Toolbar;
use iced::widget::{column, container, row, scrollable, text};
use iced::{Application, Command, Element, Length, Theme};
use std::path::PathBuf;
use tracing::{debug, info};

#[derive(Debug, Clone)]
pub enum Message {
    // Navigation
    NavigateTo(PathBuf),
    NavigateBack,
    NavigateForward,
    NavigateUp,
    NavigateHome,
    
    // File operations
    FileSelected(PathBuf),
    FileDoubleClicked(PathBuf),
    OpenFile(PathBuf),
    DeleteSelected,
    RenameStart,
    RenameConfirm(String),
    CopySelected,
    CutSelected,
    Paste,
    NewFolder,
    
    // View
    ToggleHiddenFiles,
    SetViewMode(ViewMode),
    TogglePreview,
    
    // Search
    SearchChanged(String),
    SearchSubmit,
    
    // Sidebar
    BookmarkClicked(PathBuf),
    AddBookmark,
    RemoveBookmark(PathBuf),
    
    // Preview
    PreviewLoaded(PreviewData),
    PreviewError(String),
    
    // File system events
    DirectoryChanged,
    RefreshDirectory,
    
    // Async results
    FilesLoaded(Vec<FileEntry>),
    MetadataLoaded(PathBuf, serde_json::Value),
    ThumbnailLoaded(PathBuf, Vec<u8>),
    
    // Errors
    Error(String),
    
    // Tags
    ToggleTagPanel,
    TagInputChanged(String),
    TagColorSelected(crate::tags::TagColor),
    CreateTag,
    DeleteTag(String),
    AddTagToFile(String),
    RemoveTagFromFile(String),
    ToggleTagFilter(String),
    
    // Batch operations
    BatchToggleSelect(std::path::PathBuf),
    BatchSelectAll,
    BatchDeselectAll,
    BatchSetOperation(crate::batch::BatchOperationType),
    BatchRenamePatternChanged(String),
    BatchSelectTargetDir,
    BatchExecute,
    BatchCancel,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ViewMode {
    #[default]
    List,
    Grid,
    Columns,
}

#[derive(Debug, Clone)]
pub enum PreviewData {
    Image(Vec<u8>),
    Text(String),
    Metadata(serde_json::Value),
    None,
}

pub struct RururuFiles {
    current_path: PathBuf,
    history: Vec<PathBuf>,
    history_index: usize,
    
    files: Vec<FileEntry>,
    selected: Option<PathBuf>,
    
    show_hidden: bool,
    view_mode: ViewMode,
    show_preview: bool,
    
    search_query: String,
    
    bookmarks: Vec<PathBuf>,
    
    preview_data: PreviewData,
    
    clipboard: Option<(Vec<PathBuf>, bool)>, // (paths, is_cut)
    
    loading: bool,
    error: Option<String>,
}

impl Application for RururuFiles {
    type Executor = iced::executor::Default;
    type Message = Message;
    type Theme = Theme;
    type Flags = ();

    fn new(_flags: ()) -> (Self, Command<Message>) {
        let home = dirs::home_dir().unwrap_or_else(|| PathBuf::from("/"));
        
        let bookmarks = vec![
            dirs::home_dir().unwrap_or_default(),
            dirs::document_dir().unwrap_or_default(),
            dirs::download_dir().unwrap_or_default(),
            dirs::picture_dir().unwrap_or_default(),
            dirs::video_dir().unwrap_or_default(),
            dirs::audio_dir().unwrap_or_default(),
        ]
        .into_iter()
        .filter(|p| p.exists())
        .collect();

        let app = Self {
            current_path: home.clone(),
            history: vec![home.clone()],
            history_index: 0,
            files: Vec::new(),
            selected: None,
            show_hidden: false,
            view_mode: ViewMode::List,
            show_preview: true,
            search_query: String::new(),
            bookmarks,
            preview_data: PreviewData::None,
            clipboard: None,
            loading: true,
            error: None,
        };

        (app, Command::perform(load_directory(home), |result| {
            match result {
                Ok(files) => Message::FilesLoaded(files),
                Err(e) => Message::Error(e.to_string()),
            }
        }))
    }

    fn title(&self) -> String {
        format!("RururuOS Files - {}", self.current_path.display())
    }

    fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            Message::NavigateTo(path) => {
                if path.is_dir() {
                    info!("Navigating to: {:?}", path);
                    self.current_path = path.clone();
                    
                    // Update history
                    self.history.truncate(self.history_index + 1);
                    self.history.push(path.clone());
                    self.history_index = self.history.len() - 1;
                    
                    self.loading = true;
                    self.selected = None;
                    self.preview_data = PreviewData::None;
                    
                    return Command::perform(load_directory(path), |result| {
                        match result {
                            Ok(files) => Message::FilesLoaded(files),
                            Err(e) => Message::Error(e.to_string()),
                        }
                    });
                }
            }
            
            Message::NavigateBack => {
                if self.history_index > 0 {
                    self.history_index -= 1;
                    let path = self.history[self.history_index].clone();
                    self.current_path = path.clone();
                    self.loading = true;
                    
                    return Command::perform(load_directory(path), |result| {
                        match result {
                            Ok(files) => Message::FilesLoaded(files),
                            Err(e) => Message::Error(e.to_string()),
                        }
                    });
                }
            }
            
            Message::NavigateForward => {
                if self.history_index < self.history.len() - 1 {
                    self.history_index += 1;
                    let path = self.history[self.history_index].clone();
                    self.current_path = path.clone();
                    self.loading = true;
                    
                    return Command::perform(load_directory(path), |result| {
                        match result {
                            Ok(files) => Message::FilesLoaded(files),
                            Err(e) => Message::Error(e.to_string()),
                        }
                    });
                }
            }
            
            Message::NavigateUp => {
                if let Some(parent) = self.current_path.parent() {
                    return Command::perform(
                        async move { parent.to_path_buf() },
                        Message::NavigateTo,
                    );
                }
            }
            
            Message::NavigateHome => {
                if let Some(home) = dirs::home_dir() {
                    return Command::perform(async move { home }, Message::NavigateTo);
                }
            }
            
            Message::FileSelected(path) => {
                debug!("File selected: {:?}", path);
                self.selected = Some(path.clone());
                
                if self.show_preview {
                    return Command::perform(
                        load_preview(path),
                        |result| match result {
                            Ok(data) => Message::PreviewLoaded(data),
                            Err(e) => Message::PreviewError(e.to_string()),
                        },
                    );
                }
            }
            
            Message::FileDoubleClicked(path) => {
                if path.is_dir() {
                    return Command::perform(async move { path }, Message::NavigateTo);
                } else {
                    return Command::perform(async move { path }, Message::OpenFile);
                }
            }
            
            Message::OpenFile(path) => {
                debug!("Opening file: {:?}", path);
                if let Err(e) = open::that(&path) {
                    self.error = Some(format!("Failed to open file: {}", e));
                }
            }
            
            Message::DeleteSelected => {
                if let Some(ref path) = self.selected {
                    let path = path.clone();
                    return Command::perform(
                        async move {
                            trash::delete(&path)?;
                            Ok::<_, trash::Error>(())
                        },
                        |result| match result {
                            Ok(()) => Message::RefreshDirectory,
                            Err(e) => Message::Error(e.to_string()),
                        },
                    );
                }
            }
            
            Message::ToggleHiddenFiles => {
                self.show_hidden = !self.show_hidden;
                return Command::perform(
                    load_directory(self.current_path.clone()),
                    |result| match result {
                        Ok(files) => Message::FilesLoaded(files),
                        Err(e) => Message::Error(e.to_string()),
                    },
                );
            }
            
            Message::SetViewMode(mode) => {
                self.view_mode = mode;
            }
            
            Message::TogglePreview => {
                self.show_preview = !self.show_preview;
            }
            
            Message::SearchChanged(query) => {
                self.search_query = query;
            }
            
            Message::BookmarkClicked(path) => {
                return Command::perform(async move { path }, Message::NavigateTo);
            }
            
            Message::FilesLoaded(files) => {
                let mut files = files;
                if !self.show_hidden {
                    files.retain(|f| !f.name.starts_with('.'));
                }
                
                // Apply search filter
                if !self.search_query.is_empty() {
                    let query = self.search_query.to_lowercase();
                    files.retain(|f| f.name.to_lowercase().contains(&query));
                }
                
                // Sort: directories first, then by name
                files.sort_by(|a, b| {
                    match (a.is_dir, b.is_dir) {
                        (true, false) => std::cmp::Ordering::Less,
                        (false, true) => std::cmp::Ordering::Greater,
                        _ => a.name.to_lowercase().cmp(&b.name.to_lowercase()),
                    }
                });
                
                self.files = files;
                self.loading = false;
                self.error = None;
            }
            
            Message::PreviewLoaded(data) => {
                self.preview_data = data;
            }
            
            Message::PreviewError(e) => {
                debug!("Preview error: {}", e);
                self.preview_data = PreviewData::None;
            }
            
            Message::RefreshDirectory => {
                self.loading = true;
                return Command::perform(
                    load_directory(self.current_path.clone()),
                    |result| match result {
                        Ok(files) => Message::FilesLoaded(files),
                        Err(e) => Message::Error(e.to_string()),
                    },
                );
            }
            
            Message::Error(e) => {
                self.error = Some(e);
                self.loading = false;
            }
            
            _ => {}
        }

        Command::none()
    }

    fn view(&self) -> Element<Message> {
        let toolbar = Toolbar::view(self);
        let sidebar = Sidebar::view(&self.bookmarks, &self.current_path);
        let file_list = FileList::view(&self.files, &self.selected, self.view_mode);
        
        let main_content = if self.show_preview {
            row![
                file_list,
                Preview::view(&self.preview_data, &self.selected),
            ]
            .spacing(8)
        } else {
            row![file_list]
        };

        let content = row![
            sidebar,
            column![
                toolbar,
                main_content,
            ]
            .spacing(8),
        ]
        .spacing(8)
        .padding(8);

        let content = if let Some(ref error) = self.error {
            column![
                content,
                container(text(error).style(iced::theme::Text::Color(
                    iced::Color::from_rgb(0.9, 0.3, 0.3)
                )))
                .padding(8)
            ]
            .into()
        } else {
            content.into()
        };

        container(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .into()
    }

    fn theme(&self) -> Theme {
        Theme::Dark
    }
}

async fn load_directory(path: PathBuf) -> Result<Vec<FileEntry>, std::io::Error> {
    let mut entries = Vec::new();
    
    let mut read_dir = tokio::fs::read_dir(&path).await?;
    
    while let Some(entry) = read_dir.next_entry().await? {
        let metadata = entry.metadata().await?;
        let file_type = if metadata.is_dir() {
            "directory"
        } else {
            entry
                .path()
                .extension()
                .and_then(|e| e.to_str())
                .unwrap_or("file")
        };

        entries.push(FileEntry {
            name: entry.file_name().to_string_lossy().to_string(),
            path: entry.path(),
            is_dir: metadata.is_dir(),
            size: metadata.len(),
            modified: metadata.modified().ok(),
            file_type: file_type.to_string(),
        });
    }

    Ok(entries)
}

async fn load_preview(path: PathBuf) -> Result<PreviewData, Box<dyn std::error::Error + Send + Sync>> {
    let ext = path
        .extension()
        .and_then(|e| e.to_str())
        .unwrap_or("")
        .to_lowercase();

    match ext.as_str() {
        "png" | "jpg" | "jpeg" | "gif" | "webp" | "bmp" => {
            let data = tokio::fs::read(&path).await?;
            Ok(PreviewData::Image(data))
        }
        "txt" | "md" | "rs" | "py" | "js" | "ts" | "json" | "toml" | "yaml" | "yml" | "sh" => {
            let content = tokio::fs::read_to_string(&path).await?;
            let truncated = if content.len() > 10000 {
                format!("{}...\n\n[Truncated]", &content[..10000])
            } else {
                content
            };
            Ok(PreviewData::Text(truncated))
        }
        _ => Ok(PreviewData::None),
    }
}
