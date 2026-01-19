use crate::calibration::{CalibrationState, CalibrationStep};
use crate::icc::IccProfile;
use crate::patterns::TestPattern;
use iced::widget::{button, column, container, pick_list, row, slider, text, Space};
use iced::{Application, Command, Element, Length, Theme};

#[derive(Debug, Clone)]
pub enum Message {
    // Navigation
    SelectTab(Tab),
    
    // Display selection
    SelectDisplay(String),
    RefreshDisplays,
    
    // Calibration
    StartCalibration,
    NextStep,
    PreviousStep,
    CancelCalibration,
    SaveProfile,
    
    // Adjustments
    BrightnessChanged(f32),
    ContrastChanged(f32),
    GammaChanged(f32),
    WhitePointChanged(u32),
    
    // Test patterns
    SelectPattern(TestPattern),
    ToggleFullscreen,
    
    // Profile management
    SelectProfile(String),
    ApplyProfile,
    DeleteProfile,
    ImportProfile,
    ExportProfile,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Tab {
    #[default]
    Calibrate,
    Profiles,
    TestPatterns,
    Settings,
}

impl Tab {
    pub fn title(&self) -> &'static str {
        match self {
            Tab::Calibrate => "Calibrate",
            Tab::Profiles => "Profiles",
            Tab::TestPatterns => "Test Patterns",
            Tab::Settings => "Settings",
        }
    }
}

pub struct ColorCalApp {
    current_tab: Tab,
    displays: Vec<DisplayInfo>,
    selected_display: Option<String>,
    calibration: CalibrationState,
    profiles: Vec<IccProfile>,
    selected_profile: Option<String>,
    current_pattern: TestPattern,
    fullscreen_pattern: bool,
    
    // Current adjustments
    brightness: f32,
    contrast: f32,
    gamma: f32,
    white_point: u32,
}

#[derive(Debug, Clone)]
pub struct DisplayInfo {
    pub name: String,
    pub model: String,
    pub resolution: (u32, u32),
    pub refresh_rate: u32,
    pub hdr_capable: bool,
    pub current_profile: Option<String>,
}

impl Application for ColorCalApp {
    type Executor = iced::executor::Default;
    type Message = Message;
    type Theme = Theme;
    type Flags = ();

    fn new(_flags: ()) -> (Self, Command<Message>) {
        let displays = detect_displays();
        let profiles = load_profiles();

        (
            Self {
                current_tab: Tab::default(),
                displays: displays.clone(),
                selected_display: displays.first().map(|d| d.name.clone()),
                calibration: CalibrationState::default(),
                profiles,
                selected_profile: None,
                current_pattern: TestPattern::default(),
                fullscreen_pattern: false,
                brightness: 50.0,
                contrast: 50.0,
                gamma: 2.2,
                white_point: 6500,
            },
            Command::none(),
        )
    }

    fn title(&self) -> String {
        "RururuOS Color Calibration".to_string()
    }

    fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            Message::SelectTab(tab) => {
                self.current_tab = tab;
            }
            Message::SelectDisplay(name) => {
                self.selected_display = Some(name);
            }
            Message::RefreshDisplays => {
                self.displays = detect_displays();
            }
            Message::StartCalibration => {
                self.calibration.start();
            }
            Message::NextStep => {
                self.calibration.next_step();
            }
            Message::PreviousStep => {
                self.calibration.previous_step();
            }
            Message::CancelCalibration => {
                self.calibration.cancel();
            }
            Message::SaveProfile => {
                if let Some(display) = &self.selected_display {
                    let profile = IccProfile::create(
                        display,
                        self.brightness,
                        self.contrast,
                        self.gamma,
                        self.white_point,
                    );
                    self.profiles.push(profile);
                    self.calibration.finish();
                }
            }
            Message::BrightnessChanged(val) => {
                self.brightness = val;
            }
            Message::ContrastChanged(val) => {
                self.contrast = val;
            }
            Message::GammaChanged(val) => {
                self.gamma = val;
            }
            Message::WhitePointChanged(val) => {
                self.white_point = val;
            }
            Message::SelectPattern(pattern) => {
                self.current_pattern = pattern;
            }
            Message::ToggleFullscreen => {
                self.fullscreen_pattern = !self.fullscreen_pattern;
            }
            Message::SelectProfile(name) => {
                self.selected_profile = Some(name);
            }
            Message::ApplyProfile => {
                if let Some(name) = &self.selected_profile {
                    if let Some(profile) = self.profiles.iter().find(|p| &p.name == name) {
                        apply_profile(profile);
                    }
                }
            }
            Message::DeleteProfile => {
                if let Some(name) = &self.selected_profile {
                    self.profiles.retain(|p| &p.name != name);
                    self.selected_profile = None;
                }
            }
            Message::ImportProfile | Message::ExportProfile => {
                // File dialog would be opened here
            }
        }
        Command::none()
    }

    fn view(&self) -> Element<Message> {
        let tabs = row![
            tab_button("Calibrate", Tab::Calibrate, self.current_tab),
            tab_button("Profiles", Tab::Profiles, self.current_tab),
            tab_button("Test Patterns", Tab::TestPatterns, self.current_tab),
            tab_button("Settings", Tab::Settings, self.current_tab),
        ]
        .spacing(4);

        let content: Element<Message> = match self.current_tab {
            Tab::Calibrate => self.view_calibrate(),
            Tab::Profiles => self.view_profiles(),
            Tab::TestPatterns => self.view_test_patterns(),
            Tab::Settings => self.view_settings(),
        };

        container(
            column![
                tabs,
                Space::with_height(Length::Fixed(16.0)),
                content,
            ]
            .padding(16),
        )
        .width(Length::Fill)
        .height(Length::Fill)
        .into()
    }

    fn theme(&self) -> Theme {
        Theme::Dark
    }
}

impl ColorCalApp {
    fn view_calibrate(&self) -> Element<Message> {
        // Display selector
        let display_names: Vec<String> = self.displays.iter().map(|d| d.name.clone()).collect();
        
        let display_selector = row![
            text("Display:").size(14),
            Space::with_width(Length::Fixed(8.0)),
            pick_list(
                display_names,
                self.selected_display.clone(),
                Message::SelectDisplay
            ),
            Space::with_width(Length::Fixed(8.0)),
            button(text("Refresh"))
                .style(iced::theme::Button::Secondary)
                .on_press(Message::RefreshDisplays),
        ]
        .align_items(iced::Alignment::Center);

        // Display info
        let display_info: Element<Message> = if let Some(display) = self
            .selected_display
            .as_ref()
            .and_then(|name| self.displays.iter().find(|d| &d.name == name))
        {
            column![
                row![
                    text("Model:").size(12),
                    Space::with_width(Length::Fixed(8.0)),
                    text(&display.model).size(12),
                ],
                row![
                    text("Resolution:").size(12),
                    Space::with_width(Length::Fixed(8.0)),
                    text(format!("{}×{} @ {}Hz", display.resolution.0, display.resolution.1, display.refresh_rate)).size(12),
                ],
                row![
                    text("HDR:").size(12),
                    Space::with_width(Length::Fixed(8.0)),
                    text(if display.hdr_capable { "Supported" } else { "Not supported" }).size(12),
                ],
                row![
                    text("Current Profile:").size(12),
                    Space::with_width(Length::Fixed(8.0)),
                    text(display.current_profile.as_deref().unwrap_or("None")).size(12),
                ],
            ]
            .spacing(4)
            .into()
        } else {
            text("No display selected").size(12).into()
        };

        // Calibration controls
        let calibration_content: Element<Message> = if self.calibration.is_active() {
            self.view_calibration_step()
        } else {
            column![
                text("Quick Calibration").size(16),
                Space::with_height(Length::Fixed(16.0)),
                
                row![
                    text("Brightness").width(Length::Fixed(100.0)),
                    slider(0.0..=100.0, self.brightness, Message::BrightnessChanged),
                    text(format!("{:.0}%", self.brightness)).width(Length::Fixed(50.0)),
                ]
                .spacing(8)
                .align_items(iced::Alignment::Center),

                row![
                    text("Contrast").width(Length::Fixed(100.0)),
                    slider(0.0..=100.0, self.contrast, Message::ContrastChanged),
                    text(format!("{:.0}%", self.contrast)).width(Length::Fixed(50.0)),
                ]
                .spacing(8)
                .align_items(iced::Alignment::Center),

                row![
                    text("Gamma").width(Length::Fixed(100.0)),
                    slider(1.0..=3.0, self.gamma, Message::GammaChanged).step(0.1),
                    text(format!("{:.1}", self.gamma)).width(Length::Fixed(50.0)),
                ]
                .spacing(8)
                .align_items(iced::Alignment::Center),

                row![
                    text("White Point").width(Length::Fixed(100.0)),
                    slider(5000.0..=9000.0, self.white_point as f32, |v| Message::WhitePointChanged(v as u32)).step(100.0),
                    text(format!("{}K", self.white_point)).width(Length::Fixed(50.0)),
                ]
                .spacing(8)
                .align_items(iced::Alignment::Center),

                Space::with_height(Length::Fixed(24.0)),

                row![
                    button(text("Start Guided Calibration"))
                        .style(iced::theme::Button::Primary)
                        .on_press(Message::StartCalibration),
                    Space::with_width(Length::Fixed(8.0)),
                    button(text("Save Profile"))
                        .style(iced::theme::Button::Secondary)
                        .on_press(Message::SaveProfile),
                ],
            ]
            .spacing(12)
            .into()
        };

        column![
            display_selector,
            Space::with_height(Length::Fixed(16.0)),
            display_info,
            Space::with_height(Length::Fixed(24.0)),
            calibration_content,
        ]
        .spacing(8)
        .into()
    }

    fn view_calibration_step(&self) -> Element<Message> {
        let step = self.calibration.current_step();
        let step_num = self.calibration.step_number();
        let total_steps = self.calibration.total_steps();

        let (title, description, content): (&str, &str, Element<Message>) = match step {
            CalibrationStep::Warmup => (
                "Display Warmup",
                "Let your display warm up for accurate calibration. Wait at least 15 minutes after turning on.",
                text("Please ensure your display has been on for at least 15 minutes.").into(),
            ),
            CalibrationStep::Brightness => (
                "Brightness Adjustment",
                "Adjust brightness so the darkest patch is barely visible.",
                column![
                    crate::patterns::view_pattern(&TestPattern::BlackLevel),
                    Space::with_height(Length::Fixed(16.0)),
                    slider(0.0..=100.0, self.brightness, Message::BrightnessChanged),
                ]
                .into(),
            ),
            CalibrationStep::Contrast => (
                "Contrast Adjustment",
                "Adjust contrast so all white patches are distinguishable.",
                column![
                    crate::patterns::view_pattern(&TestPattern::WhiteLevel),
                    Space::with_height(Length::Fixed(16.0)),
                    slider(0.0..=100.0, self.contrast, Message::ContrastChanged),
                ]
                .into(),
            ),
            CalibrationStep::Gamma => (
                "Gamma Adjustment",
                "Adjust gamma until the pattern appears uniform at a distance.",
                column![
                    crate::patterns::view_pattern(&TestPattern::Gamma),
                    Space::with_height(Length::Fixed(16.0)),
                    slider(1.0..=3.0, self.gamma, Message::GammaChanged).step(0.1),
                ]
                .into(),
            ),
            CalibrationStep::WhitePoint => (
                "White Point",
                "Select your target white point (6500K is standard for most work).",
                column![
                    crate::patterns::view_pattern(&TestPattern::WhiteBalance),
                    Space::with_height(Length::Fixed(16.0)),
                    slider(5000.0..=9000.0, self.white_point as f32, |v| Message::WhitePointChanged(v as u32)).step(100.0),
                    text(format!("{}K", self.white_point)),
                ]
                .into(),
            ),
            CalibrationStep::Verify => (
                "Verification",
                "Review test patterns to verify calibration quality.",
                column![
                    crate::patterns::view_pattern(&TestPattern::ColorBars),
                    Space::with_height(Length::Fixed(8.0)),
                    crate::patterns::view_pattern(&TestPattern::Gradient),
                ]
                .into(),
            ),
            CalibrationStep::Complete => (
                "Calibration Complete",
                "Your display has been calibrated. Save the profile to apply it.",
                column![
                    text("Calibration settings:").size(14),
                    text(format!("Brightness: {:.0}%", self.brightness)).size(12),
                    text(format!("Contrast: {:.0}%", self.contrast)).size(12),
                    text(format!("Gamma: {:.1}", self.gamma)).size(12),
                    text(format!("White Point: {}K", self.white_point)).size(12),
                ]
                .spacing(4)
                .into(),
            ),
        };

        column![
            text(format!("Step {} of {}: {}", step_num, total_steps, title)).size(18),
            text(description).size(12),
            Space::with_height(Length::Fixed(16.0)),
            content,
            Space::with_height(Length::Fixed(24.0)),
            row![
                if step_num > 1 {
                    button(text("Previous"))
                        .style(iced::theme::Button::Secondary)
                        .on_press(Message::PreviousStep)
                } else {
                    button(text("Previous"))
                        .style(iced::theme::Button::Secondary)
                },
                Space::with_width(Length::Fixed(8.0)),
                button(text("Cancel"))
                    .style(iced::theme::Button::Text)
                    .on_press(Message::CancelCalibration),
                Space::with_width(Length::Fill),
                if matches!(step, CalibrationStep::Complete) {
                    button(text("Save Profile"))
                        .style(iced::theme::Button::Primary)
                        .on_press(Message::SaveProfile)
                } else {
                    button(text("Next"))
                        .style(iced::theme::Button::Primary)
                        .on_press(Message::NextStep)
                },
            ],
        ]
        .spacing(8)
        .into()
    }

    fn view_profiles(&self) -> Element<Message> {
        let profile_list: Vec<Element<Message>> = self
            .profiles
            .iter()
            .map(|profile| {
                let is_selected = self.selected_profile.as_ref() == Some(&profile.name);
                
                button(
                    row![
                        column![
                            text(&profile.name).size(14),
                            text(&profile.description).size(11),
                        ],
                        Space::with_width(Length::Fill),
                        text(&profile.created).size(11),
                    ]
                    .align_items(iced::Alignment::Center)
                    .padding(8),
                )
                .style(if is_selected {
                    iced::theme::Button::Primary
                } else {
                    iced::theme::Button::Secondary
                })
                .width(Length::Fill)
                .on_press(Message::SelectProfile(profile.name.clone()))
                .into()
            })
            .collect();

        let actions = row![
            button(text("Apply"))
                .style(iced::theme::Button::Primary)
                .on_press(Message::ApplyProfile),
            button(text("Delete"))
                .style(iced::theme::Button::Destructive)
                .on_press(Message::DeleteProfile),
            Space::with_width(Length::Fill),
            button(text("Import"))
                .style(iced::theme::Button::Secondary)
                .on_press(Message::ImportProfile),
            button(text("Export"))
                .style(iced::theme::Button::Secondary)
                .on_press(Message::ExportProfile),
        ]
        .spacing(8);

        column![
            text("ICC Profiles").size(18),
            Space::with_height(Length::Fixed(16.0)),
            column(profile_list).spacing(4),
            Space::with_height(Length::Fixed(16.0)),
            actions,
        ]
        .spacing(8)
        .into()
    }

    fn view_test_patterns(&self) -> Element<Message> {
        let patterns = vec![
            TestPattern::ColorBars,
            TestPattern::Gradient,
            TestPattern::BlackLevel,
            TestPattern::WhiteLevel,
            TestPattern::Gamma,
            TestPattern::WhiteBalance,
            TestPattern::Resolution,
            TestPattern::DeadPixel,
        ];

        let pattern_buttons: Vec<Element<Message>> = patterns
            .iter()
            .map(|pattern| {
                let is_selected = *pattern == self.current_pattern;
                button(text(pattern.name()))
                    .style(if is_selected {
                        iced::theme::Button::Primary
                    } else {
                        iced::theme::Button::Secondary
                    })
                    .on_press(Message::SelectPattern(pattern.clone()))
                    .into()
            })
            .collect();

        column![
            text("Test Patterns").size(18),
            Space::with_height(Length::Fixed(16.0)),
            row(pattern_buttons).spacing(8),
            Space::with_height(Length::Fixed(16.0)),
            text(self.current_pattern.description()).size(12),
            Space::with_height(Length::Fixed(16.0)),
            crate::patterns::view_pattern(&self.current_pattern),
            Space::with_height(Length::Fixed(16.0)),
            button(text("Fullscreen"))
                .style(iced::theme::Button::Primary)
                .on_press(Message::ToggleFullscreen),
        ]
        .spacing(8)
        .into()
    }

    fn view_settings(&self) -> Element<Message> {
        column![
            text("Calibration Settings").size(18),
            Space::with_height(Length::Fixed(16.0)),

            row![
                text("Default White Point").width(Length::Fixed(200.0)),
                text("6500K (D65)"),
            ]
            .spacing(8),

            row![
                text("Default Gamma").width(Length::Fixed(200.0)),
                text("2.2 (sRGB)"),
            ]
            .spacing(8),

            row![
                text("Profile Location").width(Length::Fixed(200.0)),
                text("~/.local/share/icc/"),
            ]
            .spacing(8),

            Space::with_height(Length::Fixed(24.0)),

            text("Color Spaces").size(18),
            Space::with_height(Length::Fixed(8.0)),

            row![
                text("Working Space").width(Length::Fixed(200.0)),
                text("sRGB"),
            ]
            .spacing(8),

            row![
                text("CMYK Profile").width(Length::Fixed(200.0)),
                text("None"),
            ]
            .spacing(8),
        ]
        .spacing(8)
        .into()
    }
}

fn tab_button(label: &str, tab: Tab, current: Tab) -> Element<Message> {
    let style = if tab == current {
        iced::theme::Button::Primary
    } else {
        iced::theme::Button::Secondary
    };

    button(text(label))
        .style(style)
        .padding(8)
        .on_press(Message::SelectTab(tab))
        .into()
}

fn detect_displays() -> Vec<DisplayInfo> {
    // In real implementation, would use wlr-randr or similar
    vec![
        DisplayInfo {
            name: "DP-1".to_string(),
            model: "Dell U2720Q".to_string(),
            resolution: (3840, 2160),
            refresh_rate: 60,
            hdr_capable: true,
            current_profile: None,
        },
        DisplayInfo {
            name: "HDMI-1".to_string(),
            model: "BenQ SW271".to_string(),
            resolution: (3840, 2160),
            refresh_rate: 60,
            hdr_capable: true,
            current_profile: Some("BenQ_SW271_D65.icc".to_string()),
        },
    ]
}

fn load_profiles() -> Vec<IccProfile> {
    // Would load from ~/.local/share/icc/
    vec![
        IccProfile {
            name: "sRGB".to_string(),
            description: "Standard sRGB color space".to_string(),
            path: "/usr/share/color/icc/sRGB.icc".to_string(),
            created: "Built-in".to_string(),
        },
        IccProfile {
            name: "Display P3".to_string(),
            description: "Wide gamut display profile".to_string(),
            path: "/usr/share/color/icc/DisplayP3.icc".to_string(),
            created: "Built-in".to_string(),
        },
    ]
}

fn apply_profile(profile: &IccProfile) {
    // Would use colord or similar to apply profile
    tracing::info!("Applying profile: {}", profile.name);
}
