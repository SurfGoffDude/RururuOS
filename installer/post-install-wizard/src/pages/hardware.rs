use iced::{
    widget::{button, column, container, row, scrollable, text, vertical_space},
    Element, Length,
};
use rururu_hardware_detect::{HardwareInfo, Priority};
use crate::wizard::Message;

pub struct HardwarePage {
    pub info: Option<HardwareInfo>,
    pub applied_recommendations: Vec<usize>,
}

impl HardwarePage {
    pub fn new() -> Self {
        Self {
            info: None,
            applied_recommendations: Vec::new(),
        }
    }
    
    pub fn apply_recommendation(&mut self, index: usize) {
        if let Some(ref info) = self.info {
            if let Some(rec) = info.recommendations.get(index) {
                if let Some(ref action) = rec.action {
                    let _ = std::process::Command::new("sh")
                        .args(["-c", action])
                        .spawn();
                    self.applied_recommendations.push(index);
                }
            }
        }
    }
    
    pub fn view(&self) -> Element<Message> {
        match &self.info {
            None => {
                container(
                    column![
                        text("Detecting hardware...").size(24),
                        vertical_space().height(20),
                        text("Please wait while we analyze your system."),
                    ]
                    .spacing(10)
                )
                .width(Length::Fill)
                .center_x()
                .into()
            }
            Some(info) => {
                let cpu_section = column![
                    text("CPU").size(18),
                    text(format!("{}", info.cpu.model)),
                    text(format!("{} cores / {} threads", info.cpu.cores, info.cpu.threads)),
                ]
                .spacing(5);
                
                let gpu_section = column![
                    text("GPU").size(18),
                ]
                .push(
                    info.gpu.iter().fold(column![].spacing(2), |col, gpu| {
                        let vram = gpu.vram_mb.map(|v| format!(" ({} MB)", v)).unwrap_or_default();
                        col.push(text(format!("• {:?}: {}{}", gpu.vendor, gpu.name, vram)))
                    })
                )
                .spacing(5);
                
                let mem_section = column![
                    text("Memory").size(18),
                    text(format!("{} GB {:?}", info.memory.total_gb, info.memory.memory_type)),
                ]
                .spacing(5);
                
                let storage_section = column![
                    text("Storage").size(18),
                ]
                .push(
                    info.storage.iter().take(3).fold(column![].spacing(2), |col, disk| {
                        col.push(text(format!("• {} ({:?}): {} GB", disk.name, disk.storage_type, disk.size_gb)))
                    })
                )
                .spacing(5);
                
                let recommendations = if !info.recommendations.is_empty() {
                    let recs = info.recommendations.iter().enumerate().fold(
                        column![text("Recommendations").size(18)].spacing(10),
                        |col, (i, rec)| {
                            let priority_icon = match rec.priority {
                                Priority::Critical => "🔴",
                                Priority::High => "🟠",
                                Priority::Medium => "🟡",
                                Priority::Low => "🟢",
                            };
                            
                            let applied = self.applied_recommendations.contains(&i);
                            
                            let action_btn = if rec.action.is_some() && !applied {
                                button(text("Apply")).on_press(Message::ApplyRecommendation(i))
                            } else if applied {
                                button(text("Applied ✓"))
                            } else {
                                button(text("Info"))
                            };
                            
                            col.push(
                                row![
                                    text(format!("{} {}", priority_icon, rec.title)).width(200),
                                    text(&rec.description).width(Length::Fill),
                                    action_btn,
                                ]
                                .spacing(10)
                            )
                        }
                    );
                    recs
                } else {
                    column![
                        text("Recommendations").size(18),
                        text("No recommendations - your system looks good!"),
                    ]
                };
                
                scrollable(
                    column![
                        text("Hardware Detection").size(24),
                        vertical_space().height(20),
                        row![
                            cpu_section.width(Length::FillPortion(1)),
                            gpu_section.width(Length::FillPortion(1)),
                        ]
                        .spacing(30),
                        vertical_space().height(20),
                        row![
                            mem_section.width(Length::FillPortion(1)),
                            storage_section.width(Length::FillPortion(1)),
                        ]
                        .spacing(30),
                        vertical_space().height(30),
                        recommendations,
                    ]
                    .spacing(10)
                    .padding(10)
                )
                .into()
            }
        }
    }
}
