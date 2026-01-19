use crate::pages::{
    about::AboutPage, appearance::AppearancePage, audio::AudioPage, displays::DisplaysPage,
    keyboard::KeyboardPage, network::NetworkPage, power::PowerPage, storage::StoragePage,
};
use iced::widget::{button, column, container, row, scrollable, text, Space};
use iced::{Application, Command, Element, Length, Theme};

#[derive(Debug, Clone)]
pub enum Message {
    SelectPage(Page),
    // Appearance
    ThemeChanged(String),
    AccentColorChanged([u8; 3]),
    FontChanged(String),
    IconThemeChanged(String),
    // Display
    ResolutionChanged(String),
    RefreshRateChanged(u32),
    ScaleChanged(f32),
    NightLightToggled(bool),
    // Audio
    OutputVolumeChanged(f32),
    InputVolumeChanged(f32),
    OutputDeviceChanged(String),
    InputDeviceChanged(String),
    // Keyboard
    LayoutAdded(String),
    LayoutRemoved(String),
    ShortcutChanged(String, String),
    // Network
    WifiToggled(bool),
    WifiConnect(String),
    // Power
    PowerProfileChanged(String),
    AutoSuspendChanged(u32),
    // Storage
    RefreshStorage,
    // About
    CopySystemInfo,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Page {
    #[default]
    Appearance,
    Displays,
    Audio,
    Keyboard,
    Network,
    Power,
    Storage,
    About,
}

impl Page {
    pub fn title(&self) -> &'static str {
        match self {
            Page::Appearance => "Appearance",
            Page::Displays => "Displays",
            Page::Audio => "Audio",
            Page::Keyboard => "Keyboard",
            Page::Network => "Network",
            Page::Power => "Power",
            Page::Storage => "Storage",
            Page::About => "About",
        }
    }

    pub fn icon(&self) -> &'static str {
        match self {
            Page::Appearance => "🎨",
            Page::Displays => "🖥️",
            Page::Audio => "🔊",
            Page::Keyboard => "⌨️",
            Page::Network => "🌐",
            Page::Power => "🔋",
            Page::Storage => "💾",
            Page::About => "ℹ️",
        }
    }

    pub fn all() -> &'static [Page] {
        &[
            Page::Appearance,
            Page::Displays,
            Page::Audio,
            Page::Keyboard,
            Page::Network,
            Page::Power,
            Page::Storage,
            Page::About,
        ]
    }
}

pub struct SettingsApp {
    current_page: Page,
    appearance: AppearancePage,
    displays: DisplaysPage,
    audio: AudioPage,
    keyboard: KeyboardPage,
    network: NetworkPage,
    power: PowerPage,
    storage: StoragePage,
    about: AboutPage,
}

impl Application for SettingsApp {
    type Executor = iced::executor::Default;
    type Message = Message;
    type Theme = Theme;
    type Flags = ();

    fn new(_flags: ()) -> (Self, Command<Message>) {
        (
            Self {
                current_page: Page::default(),
                appearance: AppearancePage::new(),
                displays: DisplaysPage::new(),
                audio: AudioPage::new(),
                keyboard: KeyboardPage::new(),
                network: NetworkPage::new(),
                power: PowerPage::new(),
                storage: StoragePage::new(),
                about: AboutPage::new(),
            },
            Command::none(),
        )
    }

    fn title(&self) -> String {
        format!("RururuOS Settings - {}", self.current_page.title())
    }

    fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            Message::SelectPage(page) => {
                self.current_page = page;
            }
            Message::ThemeChanged(theme) => {
                self.appearance.set_theme(&theme);
            }
            Message::AccentColorChanged(color) => {
                self.appearance.set_accent_color(color);
            }
            Message::FontChanged(font) => {
                self.appearance.set_font(&font);
            }
            Message::IconThemeChanged(theme) => {
                self.appearance.set_icon_theme(&theme);
            }
            Message::OutputVolumeChanged(vol) => {
                self.audio.set_output_volume(vol);
            }
            Message::InputVolumeChanged(vol) => {
                self.audio.set_input_volume(vol);
            }
            Message::NightLightToggled(enabled) => {
                self.displays.set_night_light(enabled);
            }
            Message::ScaleChanged(scale) => {
                self.displays.set_scale(scale);
            }
            Message::PowerProfileChanged(profile) => {
                self.power.set_profile(&profile);
            }
            Message::RefreshStorage => {
                self.storage.refresh();
            }
            _ => {}
        }
        Command::none()
    }

    fn view(&self) -> Element<Message> {
        let sidebar = self.sidebar();
        let content = self.content();

        row![sidebar, content].spacing(0).into()
    }

    fn theme(&self) -> Theme {
        Theme::Dark
    }
}

impl SettingsApp {
    fn sidebar(&self) -> Element<Message> {
        let items: Vec<Element<Message>> = Page::all()
            .iter()
            .map(|page| {
                let is_selected = *page == self.current_page;
                let style = if is_selected {
                    iced::theme::Button::Primary
                } else {
                    iced::theme::Button::Text
                };

                button(
                    row![
                        text(page.icon()).size(18),
                        Space::with_width(Length::Fixed(8.0)),
                        text(page.title()),
                    ]
                    .align_items(iced::Alignment::Center),
                )
                .style(style)
                .width(Length::Fill)
                .padding(12)
                .on_press(Message::SelectPage(*page))
                .into()
            })
            .collect();

        container(
            column![
                text("Settings").size(24),
                Space::with_height(Length::Fixed(16.0)),
                column(items).spacing(4),
            ]
            .padding(16),
        )
        .width(Length::Fixed(200.0))
        .height(Length::Fill)
        .style(iced::theme::Container::Box)
        .into()
    }

    fn content(&self) -> Element<Message> {
        let page_content: Element<Message> = match self.current_page {
            Page::Appearance => self.appearance.view(),
            Page::Displays => self.displays.view(),
            Page::Audio => self.audio.view(),
            Page::Keyboard => self.keyboard.view(),
            Page::Network => self.network.view(),
            Page::Power => self.power.view(),
            Page::Storage => self.storage.view(),
            Page::About => self.about.view(),
        };

        container(
            column![
                text(self.current_page.title()).size(28),
                Space::with_height(Length::Fixed(16.0)),
                scrollable(page_content).height(Length::Fill),
            ]
            .padding(24),
        )
        .width(Length::Fill)
        .height(Length::Fill)
        .into()
    }
}
