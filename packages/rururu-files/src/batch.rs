use crate::app::Message;
use iced::widget::{button, checkbox, column, container, pick_list, progress_bar, row, text, text_input, Space};
use iced::{Element, Length};
use std::path::{Path, PathBuf};
use std::collections::HashSet;

#[derive(Debug, Clone)]
pub struct BatchOperation {
    pub selected_files: HashSet<PathBuf>,
    pub operation: Option<BatchOperationType>,
    pub progress: f32,
    pub is_running: bool,
    pub results: Vec<BatchResult>,
    pub rename_pattern: String,
    pub target_directory: Option<PathBuf>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BatchOperationType {
    Copy,
    Move,
    Delete,
    Rename,
    AddTag,
    RemoveTag,
    ConvertFormat,
    Compress,
}

impl std::fmt::Display for BatchOperationType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BatchOperationType::Copy => write!(f, "Copy"),
            BatchOperationType::Move => write!(f, "Move"),
            BatchOperationType::Delete => write!(f, "Delete"),
            BatchOperationType::Rename => write!(f, "Rename"),
            BatchOperationType::AddTag => write!(f, "Add Tag"),
            BatchOperationType::RemoveTag => write!(f, "Remove Tag"),
            BatchOperationType::ConvertFormat => write!(f, "Convert Format"),
            BatchOperationType::Compress => write!(f, "Compress"),
        }
    }
}

#[derive(Debug, Clone)]
pub struct BatchResult {
    pub path: PathBuf,
    pub success: bool,
    pub message: String,
}

impl Default for BatchOperation {
    fn default() -> Self {
        Self {
            selected_files: HashSet::new(),
            operation: None,
            progress: 0.0,
            is_running: false,
            results: Vec::new(),
            rename_pattern: String::from("{name}_{n}"),
            target_directory: None,
        }
    }
}

impl BatchOperation {
    pub fn select_file(&mut self, path: PathBuf) {
        if self.selected_files.contains(&path) {
            self.selected_files.remove(&path);
        } else {
            self.selected_files.insert(path);
        }
    }

    pub fn select_all(&mut self, files: &[PathBuf]) {
        for file in files {
            self.selected_files.insert(file.clone());
        }
    }

    pub fn deselect_all(&mut self) {
        self.selected_files.clear();
    }

    pub fn is_selected(&self, path: &Path) -> bool {
        self.selected_files.contains(path)
    }

    pub fn selection_count(&self) -> usize {
        self.selected_files.len()
    }

    pub fn set_operation(&mut self, op: BatchOperationType) {
        self.operation = Some(op);
    }

    pub async fn execute(&mut self) -> Vec<BatchResult> {
        self.is_running = true;
        self.results.clear();
        self.progress = 0.0;

        let total = self.selected_files.len();
        let files: Vec<PathBuf> = self.selected_files.iter().cloned().collect();

        for (i, file) in files.iter().enumerate() {
            let result = match &self.operation {
                Some(BatchOperationType::Copy) => self.copy_file(file).await,
                Some(BatchOperationType::Move) => self.move_file(file).await,
                Some(BatchOperationType::Delete) => self.delete_file(file).await,
                Some(BatchOperationType::Rename) => self.rename_file(file, i).await,
                Some(BatchOperationType::Compress) => self.compress_file(file).await,
                _ => BatchResult {
                    path: file.clone(),
                    success: false,
                    message: "Operation not implemented".to_string(),
                },
            };

            self.results.push(result);
            self.progress = (i + 1) as f32 / total as f32;
        }

        self.is_running = false;
        self.results.clone()
    }

    async fn copy_file(&self, source: &Path) -> BatchResult {
        let target_dir = self.target_directory.as_ref();
        
        match target_dir {
            Some(dir) => {
                let dest = dir.join(source.file_name().unwrap_or_default());
                match tokio::fs::copy(source, &dest).await {
                    Ok(_) => BatchResult {
                        path: source.to_path_buf(),
                        success: true,
                        message: format!("Copied to {:?}", dest),
                    },
                    Err(e) => BatchResult {
                        path: source.to_path_buf(),
                        success: false,
                        message: e.to_string(),
                    },
                }
            }
            None => BatchResult {
                path: source.to_path_buf(),
                success: false,
                message: "No target directory specified".to_string(),
            },
        }
    }

    async fn move_file(&self, source: &Path) -> BatchResult {
        let target_dir = self.target_directory.as_ref();
        
        match target_dir {
            Some(dir) => {
                let dest = dir.join(source.file_name().unwrap_or_default());
                match tokio::fs::rename(source, &dest).await {
                    Ok(_) => BatchResult {
                        path: source.to_path_buf(),
                        success: true,
                        message: format!("Moved to {:?}", dest),
                    },
                    Err(e) => BatchResult {
                        path: source.to_path_buf(),
                        success: false,
                        message: e.to_string(),
                    },
                }
            }
            None => BatchResult {
                path: source.to_path_buf(),
                success: false,
                message: "No target directory specified".to_string(),
            },
        }
    }

    async fn delete_file(&self, path: &Path) -> BatchResult {
        // Move to trash instead of permanent delete
        match trash::delete(path) {
            Ok(_) => BatchResult {
                path: path.to_path_buf(),
                success: true,
                message: "Moved to trash".to_string(),
            },
            Err(e) => BatchResult {
                path: path.to_path_buf(),
                success: false,
                message: e.to_string(),
            },
        }
    }

    async fn rename_file(&self, path: &Path, index: usize) -> BatchResult {
        let parent = path.parent().unwrap_or(Path::new("."));
        let stem = path.file_stem().and_then(|s| s.to_str()).unwrap_or("file");
        let ext = path.extension().and_then(|s| s.to_str()).unwrap_or("");

        let new_name = self
            .rename_pattern
            .replace("{name}", stem)
            .replace("{n}", &format!("{:04}", index + 1))
            .replace("{ext}", ext);

        let new_path = if ext.is_empty() {
            parent.join(&new_name)
        } else {
            parent.join(format!("{}.{}", new_name, ext))
        };

        match tokio::fs::rename(path, &new_path).await {
            Ok(_) => BatchResult {
                path: path.to_path_buf(),
                success: true,
                message: format!("Renamed to {:?}", new_path.file_name().unwrap_or_default()),
            },
            Err(e) => BatchResult {
                path: path.to_path_buf(),
                success: false,
                message: e.to_string(),
            },
        }
    }

    async fn compress_file(&self, path: &Path) -> BatchResult {
        // Create zip archive for single file
        let zip_path = path.with_extension("zip");
        
        // This is a placeholder - real implementation would use zip crate
        BatchResult {
            path: path.to_path_buf(),
            success: false,
            message: "Compression not yet implemented".to_string(),
        }
    }
}

pub fn view_batch_toolbar<'a>(batch: &'a BatchOperation) -> Element<'a, Message> {
    if batch.selected_files.is_empty() {
        return Space::new(Length::Shrink, Length::Shrink).into();
    }

    let operations = vec![
        BatchOperationType::Copy,
        BatchOperationType::Move,
        BatchOperationType::Delete,
        BatchOperationType::Rename,
        BatchOperationType::Compress,
    ];

    row![
        text(format!("{} selected", batch.selection_count())).size(13),
        Space::with_width(Length::Fixed(16.0)),
        
        button(text("Copy"))
            .style(iced::theme::Button::Secondary)
            .on_press(Message::BatchSetOperation(BatchOperationType::Copy)),
        
        button(text("Move"))
            .style(iced::theme::Button::Secondary)
            .on_press(Message::BatchSetOperation(BatchOperationType::Move)),
        
        button(text("Delete"))
            .style(iced::theme::Button::Destructive)
            .on_press(Message::BatchSetOperation(BatchOperationType::Delete)),
        
        button(text("Rename"))
            .style(iced::theme::Button::Secondary)
            .on_press(Message::BatchSetOperation(BatchOperationType::Rename)),
        
        Space::with_width(Length::Fill),
        
        button(text("Deselect All"))
            .style(iced::theme::Button::Text)
            .on_press(Message::BatchDeselectAll),
    ]
    .spacing(8)
    .align_items(iced::Alignment::Center)
    .padding(8)
    .into()
}

pub fn view_batch_dialog<'a>(batch: &'a BatchOperation) -> Element<'a, Message> {
    let op = match &batch.operation {
        Some(op) => op,
        None => return Space::new(Length::Shrink, Length::Shrink).into(),
    };

    let title = format!("{} {} files", op, batch.selection_count());

    let options: Element<Message> = match op {
        BatchOperationType::Rename => {
            column![
                text("Rename pattern:").size(12),
                text_input("{name}_{n}", &batch.rename_pattern)
                    .on_input(Message::BatchRenamePatternChanged),
                text("Variables: {name}, {n}, {ext}").size(11),
            ]
            .spacing(4)
            .into()
        }
        BatchOperationType::Copy | BatchOperationType::Move => {
            column![
                text("Target directory:").size(12),
                row![
                    text(
                        batch
                            .target_directory
                            .as_ref()
                            .map(|p| p.display().to_string())
                            .unwrap_or_else(|| "Not selected".to_string())
                    )
                    .size(12),
                    button(text("Browse"))
                        .style(iced::theme::Button::Secondary)
                        .on_press(Message::BatchSelectTargetDir),
                ]
                .spacing(8),
            ]
            .spacing(4)
            .into()
        }
        BatchOperationType::Delete => {
            text("Files will be moved to trash.")
                .size(12)
                .into()
        }
        _ => Space::new(Length::Shrink, Length::Shrink).into(),
    };

    let progress_section: Element<Message> = if batch.is_running {
        column![
            progress_bar(0.0..=1.0, batch.progress),
            text(format!("{:.0}%", batch.progress * 100.0)).size(11),
        ]
        .spacing(4)
        .into()
    } else if !batch.results.is_empty() {
        let success_count = batch.results.iter().filter(|r| r.success).count();
        let fail_count = batch.results.len() - success_count;

        column![
            text(format!("Completed: {} success, {} failed", success_count, fail_count)).size(12),
        ]
        .into()
    } else {
        Space::new(Length::Shrink, Length::Shrink).into()
    };

    container(
        column![
            row![
                text(&title).size(16),
                Space::with_width(Length::Fill),
                button(text("×"))
                    .style(iced::theme::Button::Text)
                    .on_press(Message::BatchCancel),
            ],
            Space::with_height(Length::Fixed(12.0)),
            options,
            Space::with_height(Length::Fixed(12.0)),
            progress_section,
            Space::with_height(Length::Fixed(12.0)),
            row![
                Space::with_width(Length::Fill),
                button(text("Cancel"))
                    .style(iced::theme::Button::Secondary)
                    .on_press(Message::BatchCancel),
                button(text("Execute"))
                    .style(iced::theme::Button::Primary)
                    .on_press(Message::BatchExecute),
            ]
            .spacing(8),
        ]
        .spacing(8)
        .padding(16),
    )
    .width(Length::Fixed(400.0))
    .style(iced::theme::Container::Box)
    .into()
}
