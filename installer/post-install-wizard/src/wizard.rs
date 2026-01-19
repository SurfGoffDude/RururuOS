use iced::{
    widget::{button, column, container, horizontal_space, progress_bar, row, text, vertical_space},
    Application, Command, Element, Length, Theme,
};

use crate::pages::{
    welcome::WelcomePage,
    hardware::HardwarePage,
    workflow::WorkflowPage,
    apps::AppsPage,
    settings::SettingsPage,
    finish::FinishPage,
};

#[derive(Debug, Clone)]
pub enum Message {
    NextPage,
    PrevPage,
    GoToPage(usize),
    
    // Welcome
    LanguageSelected(String),
    
    // Hardware
    HardwareDetected(Box<rururu_hardware_detect::HardwareInfo>),
    ApplyRecommendation(usize),
    
    // Workflow
    WorkflowSelected(rururu_workflows::WorkflowType),
    
    // Apps
    ToggleApp(String),
    InstallApps,
    AppInstalled(String, bool),
    
    // Settings
    ToggleDarkMode(bool),
    ToggleAutoUpdates(bool),
    ToggleTelemetry(bool),
    
    // Finish
    Finish,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Page {
    Welcome,
    Hardware,
    Workflow,
    Apps,
    Settings,
    Finish,
}

impl Page {
    fn index(&self) -> usize {
        match self {
            Page::Welcome => 0,
            Page::Hardware => 1,
            Page::Workflow => 2,
            Page::Apps => 3,
            Page::Settings => 4,
            Page::Finish => 5,
        }
    }
    
    fn from_index(index: usize) -> Self {
        match index {
            0 => Page::Welcome,
            1 => Page::Hardware,
            2 => Page::Workflow,
            3 => Page::Apps,
            4 => Page::Settings,
            _ => Page::Finish,
        }
    }
    
    fn title(&self) -> &'static str {
        match self {
            Page::Welcome => "Welcome",
            Page::Hardware => "Hardware",
            Page::Workflow => "Workflow",
            Page::Apps => "Applications",
            Page::Settings => "Settings",
            Page::Finish => "Complete",
        }
    }
}

pub struct SetupWizard {
    current_page: Page,
    
    // Page states
    welcome: WelcomePage,
    hardware: HardwarePage,
    workflow: WorkflowPage,
    apps: AppsPage,
    settings: SettingsPage,
    finish: FinishPage,
}

impl Application for SetupWizard {
    type Executor = iced::executor::Default;
    type Message = Message;
    type Theme = Theme;
    type Flags = ();

    fn new(_flags: ()) -> (Self, Command<Message>) {
        (
            Self {
                current_page: Page::Welcome,
                welcome: WelcomePage::new(),
                hardware: HardwarePage::new(),
                workflow: WorkflowPage::new(),
                apps: AppsPage::new(),
                settings: SettingsPage::new(),
                finish: FinishPage::new(),
            },
            Command::none(),
        )
    }

    fn title(&self) -> String {
        format!("RururuOS Setup - {}", self.current_page.title())
    }

    fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            Message::NextPage => {
                let next = (self.current_page.index() + 1).min(5);
                self.current_page = Page::from_index(next);
                
                // Trigger hardware detection when entering hardware page
                if self.current_page == Page::Hardware && self.hardware.info.is_none() {
                    return Command::perform(
                        async { rururu_hardware_detect::detect_all() },
                        |info| Message::HardwareDetected(Box::new(info)),
                    );
                }
            }
            Message::PrevPage => {
                let prev = self.current_page.index().saturating_sub(1);
                self.current_page = Page::from_index(prev);
            }
            Message::GoToPage(index) => {
                self.current_page = Page::from_index(index);
            }
            
            Message::LanguageSelected(lang) => {
                self.welcome.language = lang;
            }
            
            Message::HardwareDetected(info) => {
                self.hardware.info = Some(*info);
            }
            Message::ApplyRecommendation(index) => {
                self.hardware.apply_recommendation(index);
            }
            
            Message::WorkflowSelected(workflow) => {
                self.workflow.selected = Some(workflow);
                self.apps.update_for_workflow(workflow);
            }
            
            Message::ToggleApp(app) => {
                self.apps.toggle_app(&app);
            }
            Message::InstallApps => {
                return self.apps.install_selected();
            }
            Message::AppInstalled(app, success) => {
                self.apps.mark_installed(&app, success);
            }
            
            Message::ToggleDarkMode(enabled) => {
                self.settings.dark_mode = enabled;
            }
            Message::ToggleAutoUpdates(enabled) => {
                self.settings.auto_updates = enabled;
            }
            Message::ToggleTelemetry(enabled) => {
                self.settings.telemetry = enabled;
            }
            
            Message::Finish => {
                self.finish.save_configuration(
                    &self.workflow,
                    &self.apps,
                    &self.settings,
                );
                std::process::exit(0);
            }
        }
        
        Command::none()
    }

    fn view(&self) -> Element<Message> {
        let progress = self.current_page.index() as f32 / 5.0;
        
        let content: Element<Message> = match self.current_page {
            Page::Welcome => self.welcome.view(),
            Page::Hardware => self.hardware.view(),
            Page::Workflow => self.workflow.view(),
            Page::Apps => self.apps.view(),
            Page::Settings => self.settings.view(),
            Page::Finish => self.finish.view(),
        };
        
        let nav_buttons = {
            let back_btn = if self.current_page.index() > 0 {
                button(text("← Back")).on_press(Message::PrevPage)
            } else {
                button(text("← Back"))
            };
            
            let next_btn = if self.current_page == Page::Finish {
                button(text("Finish ✓"))
                    .on_press(Message::Finish)
                    .style(iced::theme::Button::Primary)
            } else {
                button(text("Next →"))
                    .on_press(Message::NextPage)
                    .style(iced::theme::Button::Primary)
            };
            
            row![
                back_btn,
                horizontal_space(),
                next_btn,
            ]
            .spacing(10)
        };
        
        let page_indicators = row(
            (0..6).map(|i| {
                let is_current = i == self.current_page.index();
                let style = if is_current {
                    iced::theme::Button::Primary
                } else {
                    iced::theme::Button::Secondary
                };
                button(text(Page::from_index(i).title()).size(12))
                    .on_press(Message::GoToPage(i))
                    .style(style)
                    .into()
            }).collect()
        ).spacing(5);
        
        container(
            column![
                page_indicators,
                vertical_space().height(10),
                progress_bar(0.0..=1.0, progress).height(4),
                vertical_space().height(20),
                container(content)
                    .width(Length::Fill)
                    .height(Length::Fill),
                vertical_space().height(20),
                nav_buttons,
            ]
            .padding(30)
        )
        .width(Length::Fill)
        .height(Length::Fill)
        .into()
    }

    fn theme(&self) -> Theme {
        if self.settings.dark_mode {
            Theme::Dark
        } else {
            Theme::Light
        }
    }
}
