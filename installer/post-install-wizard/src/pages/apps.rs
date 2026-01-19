use iced::{
    widget::{button, checkbox, column, container, row, scrollable, text, vertical_space},
    Command, Element, Length,
};
use rururu_workflows::{WorkflowType, WorkflowProfile};
use std::collections::HashSet;
use crate::wizard::Message;

#[derive(Debug, Clone)]
pub struct AppEntry {
    pub name: String,
    pub description: String,
    pub package: String,
    pub flatpak_id: Option<String>,
    pub selected: bool,
    pub installed: bool,
    pub installing: bool,
}

pub struct AppsPage {
    pub apps: Vec<AppEntry>,
    pub selected_workflow: Option<WorkflowType>,
}

impl AppsPage {
    pub fn new() -> Self {
        Self {
            apps: Self::default_apps(),
            selected_workflow: None,
        }
    }
    
    fn default_apps() -> Vec<AppEntry> {
        vec![
            AppEntry {
                name: "Blender".to_string(),
                description: "3D creation suite".to_string(),
                package: "blender".to_string(),
                flatpak_id: Some("org.blender.Blender".to_string()),
                selected: false,
                installed: false,
                installing: false,
            },
            AppEntry {
                name: "GIMP".to_string(),
                description: "Image editor".to_string(),
                package: "gimp".to_string(),
                flatpak_id: Some("org.gimp.GIMP".to_string()),
                selected: false,
                installed: false,
                installing: false,
            },
            AppEntry {
                name: "Inkscape".to_string(),
                description: "Vector graphics".to_string(),
                package: "inkscape".to_string(),
                flatpak_id: Some("org.inkscape.Inkscape".to_string()),
                selected: false,
                installed: false,
                installing: false,
            },
            AppEntry {
                name: "Krita".to_string(),
                description: "Digital painting".to_string(),
                package: "krita".to_string(),
                flatpak_id: Some("org.kde.krita".to_string()),
                selected: false,
                installed: false,
                installing: false,
            },
            AppEntry {
                name: "Kdenlive".to_string(),
                description: "Video editor".to_string(),
                package: "kdenlive".to_string(),
                flatpak_id: Some("org.kde.kdenlive".to_string()),
                selected: false,
                installed: false,
                installing: false,
            },
            AppEntry {
                name: "Darktable".to_string(),
                description: "Photo workflow".to_string(),
                package: "darktable".to_string(),
                flatpak_id: Some("org.darktable.Darktable".to_string()),
                selected: false,
                installed: false,
                installing: false,
            },
            AppEntry {
                name: "Ardour".to_string(),
                description: "Digital audio workstation".to_string(),
                package: "ardour".to_string(),
                flatpak_id: None,
                selected: false,
                installed: false,
                installing: false,
            },
            AppEntry {
                name: "Audacity".to_string(),
                description: "Audio editor".to_string(),
                package: "audacity".to_string(),
                flatpak_id: Some("org.audacityteam.Audacity".to_string()),
                selected: false,
                installed: false,
                installing: false,
            },
            AppEntry {
                name: "OBS Studio".to_string(),
                description: "Streaming & recording".to_string(),
                package: "obs-studio".to_string(),
                flatpak_id: Some("com.obsproject.Studio".to_string()),
                selected: false,
                installed: false,
                installing: false,
            },
            AppEntry {
                name: "Handbrake".to_string(),
                description: "Video transcoder".to_string(),
                package: "handbrake".to_string(),
                flatpak_id: Some("fr.handbrake.ghb".to_string()),
                selected: false,
                installed: false,
                installing: false,
            },
        ]
    }
    
    pub fn update_for_workflow(&mut self, workflow: WorkflowType) {
        self.selected_workflow = Some(workflow);
        let profile = WorkflowProfile::get_profile(workflow);
        
        // Reset selections
        for app in &mut self.apps {
            app.selected = false;
        }
        
        // Select apps from workflow
        for wf_app in &profile.applications {
            if let Some(app) = self.apps.iter_mut().find(|a| a.package == wf_app.package) {
                app.selected = true;
            }
        }
    }
    
    pub fn toggle_app(&mut self, name: &str) {
        if let Some(app) = self.apps.iter_mut().find(|a| a.name == name) {
            app.selected = !app.selected;
        }
    }
    
    pub fn install_selected(&mut self) -> Command<Message> {
        let to_install: Vec<_> = self.apps.iter()
            .filter(|a| a.selected && !a.installed)
            .map(|a| a.name.clone())
            .collect();
        
        for name in &to_install {
            if let Some(app) = self.apps.iter_mut().find(|a| &a.name == name) {
                app.installing = true;
            }
        }
        
        Command::batch(to_install.into_iter().map(|name| {
            let app_name = name.clone();
            Command::perform(
                async move {
                    // Simulate installation
                    tokio::time::sleep(std::time::Duration::from_secs(2)).await;
                    (app_name, true)
                },
                |(name, success)| Message::AppInstalled(name, success),
            )
        }))
    }
    
    pub fn mark_installed(&mut self, name: &str, success: bool) {
        if let Some(app) = self.apps.iter_mut().find(|a| a.name == name) {
            app.installing = false;
            app.installed = success;
        }
    }
    
    pub fn view(&self) -> Element<Message> {
        let selected_count = self.apps.iter().filter(|a| a.selected).count();
        let installed_count = self.apps.iter().filter(|a| a.installed).count();
        
        let app_list = self.apps.iter().fold(column![].spacing(10), |col, app| {
            let status = if app.installed {
                "✓ Installed"
            } else if app.installing {
                "Installing..."
            } else {
                ""
            };
            
            col.push(
                row![
                    checkbox(
                        &app.name,
                        app.selected,
                        |_| Message::ToggleApp(app.name.clone()),
                    )
                    .width(150),
                    text(&app.description).width(200),
                    text(status).width(100),
                ]
                .spacing(20)
            )
        });
        
        let install_btn = if selected_count > installed_count {
            button(text(format!("Install {} Apps", selected_count - installed_count)))
                .on_press(Message::InstallApps)
                .style(iced::theme::Button::Primary)
        } else {
            button(text("All Selected Apps Installed"))
        };
        
        container(
            column![
                text("Install Applications").size(24),
                vertical_space().height(10),
                if let Some(wf) = self.selected_workflow {
                    text(format!("Recommended for: {}", wf.name()))
                } else {
                    text("Select applications to install")
                },
                vertical_space().height(20),
                scrollable(app_list).height(350),
                vertical_space().height(20),
                row![
                    text(format!("{} selected, {} installed", selected_count, installed_count)),
                    container(install_btn).width(Length::Fill).align_x(iced::alignment::Horizontal::Right),
                ],
            ]
            .spacing(10)
        )
        .width(Length::Fill)
        .into()
    }
}
