use iced::{
    widget::{column, container, pick_list, text, vertical_space, Image},
    Element, Length,
};
use crate::wizard::Message;

pub struct WelcomePage {
    pub language: String,
}

impl WelcomePage {
    pub fn new() -> Self {
        Self {
            language: "English".to_string(),
        }
    }
    
    pub fn view(&self) -> Element<Message> {
        let languages = vec![
            "English".to_string(),
            "Русский".to_string(),
            "Deutsch".to_string(),
            "Español".to_string(),
            "Français".to_string(),
            "日本語".to_string(),
            "中文".to_string(),
        ];
        
        container(
            column![
                text("Welcome to RururuOS").size(40),
                vertical_space().height(20),
                text("A creative-focused Linux distribution").size(20),
                vertical_space().height(40),
                text("This wizard will help you set up your system for creative work."),
                text("You can configure your workflow, install applications, and optimize settings."),
                vertical_space().height(40),
                text("Select your language:"),
                pick_list(
                    languages,
                    Some(self.language.clone()),
                    Message::LanguageSelected,
                ),
                vertical_space().height(40),
                text("Features:").size(18),
                text("• Hardware auto-detection and optimization"),
                text("• Pre-configured creative workflows"),
                text("• One-click application installation"),
                text("• Color management and calibration"),
                text("• Audio production ready (low-latency)"),
            ]
            .spacing(10)
        )
        .width(Length::Fill)
        .center_x()
        .into()
    }
}
