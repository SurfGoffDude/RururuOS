use iced::{
    widget::{column, container, text, vertical_space},
    Element, Length,
};
use crate::wizard::Message;
use crate::pages::{workflow::WorkflowPage, apps::AppsPage, settings::SettingsPage};
use std::fs;

pub struct FinishPage {
    pub saved: bool,
}

impl FinishPage {
    pub fn new() -> Self {
        Self { saved: false }
    }
    
    pub fn save_configuration(
        &mut self,
        workflow: &WorkflowPage,
        apps: &AppsPage,
        settings: &SettingsPage,
    ) {
        // Save workflow config
        if let Some(wf) = workflow.selected {
            if let Ok(mut config) = rururu_workflows::WorkflowConfig::load() {
                config.set_active_workflow(wf);
                let _ = config.save();
            }
        }
        
        // Save appearance settings
        if let Some(config_dir) = dirs::config_dir() {
            let rururu_dir = config_dir.join("rururu");
            let _ = fs::create_dir_all(&rururu_dir);
            
            let settings_toml = format!(
                r#"[appearance]
dark_mode = {}

[updates]
automatic = {}

[privacy]
telemetry = {}
"#,
                settings.dark_mode,
                settings.auto_updates,
                settings.telemetry,
            );
            
            let _ = fs::write(rururu_dir.join("settings.toml"), settings_toml);
        }
        
        self.saved = true;
    }
    
    pub fn view(&self) -> Element<Message> {
        container(
            column![
                text("🎉").size(60),
                vertical_space().height(20),
                text("Setup Complete!").size(32),
                vertical_space().height(20),
                text("RururuOS is now configured for your creative workflow."),
                vertical_space().height(30),
                text("What's next:").size(18),
                text("• Your selected applications will be installed in the background"),
                text("• System settings have been optimized for your workflow"),
                text("• Color management is ready for calibration"),
                vertical_space().height(30),
                text("Tips:").size(18),
                text("• Press Super key to open the application launcher"),
                text("• Use rururu-settings to customize your system"),
                text("• Run rururu-colorcal to calibrate your display"),
                text("• Check rururu-workflow for workflow management"),
                vertical_space().height(30),
                text("Click 'Finish' to start using RururuOS!").size(16),
            ]
            .spacing(10)
            .align_items(iced::Alignment::Center)
        )
        .width(Length::Fill)
        .center_x()
        .into()
    }
}
