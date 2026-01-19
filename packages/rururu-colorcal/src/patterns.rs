use crate::app::Message;
use iced::widget::{column, container, row, text, Space};
use iced::{Color, Element, Length};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum TestPattern {
    #[default]
    ColorBars,
    Gradient,
    BlackLevel,
    WhiteLevel,
    Gamma,
    WhiteBalance,
    Resolution,
    DeadPixel,
}

impl TestPattern {
    pub fn name(&self) -> &'static str {
        match self {
            TestPattern::ColorBars => "Color Bars",
            TestPattern::Gradient => "Gradient",
            TestPattern::BlackLevel => "Black Level",
            TestPattern::WhiteLevel => "White Level",
            TestPattern::Gamma => "Gamma",
            TestPattern::WhiteBalance => "White Balance",
            TestPattern::Resolution => "Resolution",
            TestPattern::DeadPixel => "Dead Pixel",
        }
    }

    pub fn description(&self) -> &'static str {
        match self {
            TestPattern::ColorBars => {
                "Standard color bars for checking color reproduction and saturation."
            }
            TestPattern::Gradient => "Smooth gradient to check for banding and color transitions.",
            TestPattern::BlackLevel => "Black level test - adjust brightness until barely visible.",
            TestPattern::WhiteLevel => {
                "White level test - adjust contrast for visible distinctions."
            }
            TestPattern::Gamma => "Gamma calibration pattern - should appear uniform at distance.",
            TestPattern::WhiteBalance => {
                "White balance test - pure white should have no color cast."
            }
            TestPattern::Resolution => "Resolution test pattern for checking sharpness.",
            TestPattern::DeadPixel => "Dead pixel test - solid colors to find stuck pixels.",
        }
    }
}

pub fn view_pattern<'a>(pattern: &TestPattern) -> Element<'a, Message> {
    let pattern_element: Element<Message> = match pattern {
        TestPattern::ColorBars => view_color_bars(),
        TestPattern::Gradient => view_gradient(),
        TestPattern::BlackLevel => view_black_level(),
        TestPattern::WhiteLevel => view_white_level(),
        TestPattern::Gamma => view_gamma(),
        TestPattern::WhiteBalance => view_white_balance(),
        TestPattern::Resolution => view_resolution(),
        TestPattern::DeadPixel => view_dead_pixel(),
    };

    container(pattern_element)
        .width(Length::Fixed(600.0))
        .height(Length::Fixed(300.0))
        .style(iced::theme::Container::Box)
        .into()
}

fn view_color_bars<'a>() -> Element<'a, Message> {
    // SMPTE color bars simulation using containers
    let colors = [
        Color::from_rgb(0.75, 0.75, 0.75), // Gray
        Color::from_rgb(0.75, 0.75, 0.0),  // Yellow
        Color::from_rgb(0.0, 0.75, 0.75),  // Cyan
        Color::from_rgb(0.0, 0.75, 0.0),   // Green
        Color::from_rgb(0.75, 0.0, 0.75),  // Magenta
        Color::from_rgb(0.75, 0.0, 0.0),   // Red
        Color::from_rgb(0.0, 0.0, 0.75),   // Blue
    ];

    let bars: Vec<Element<Message>> = colors
        .iter()
        .map(|_color| {
            container(Space::new(Length::Fill, Length::Fill))
                .width(Length::FillPortion(1))
                .height(Length::Fill)
                .style(iced::theme::Container::Box)
                .into()
        })
        .collect();

    row(bars).height(Length::Fill).into()
}

fn view_gradient<'a>() -> Element<'a, Message> {
    // Grayscale gradient simulation
    let steps = 16;
    let bars: Vec<Element<Message>> = (0..steps)
        .map(|i| {
            let _intensity = i as f32 / (steps - 1) as f32;
            container(Space::new(Length::Fill, Length::Fill))
                .width(Length::FillPortion(1))
                .height(Length::Fill)
                .style(iced::theme::Container::Box)
                .into()
        })
        .collect();

    column![
        text("Grayscale Gradient").size(12),
        row(bars).height(Length::Fixed(100.0)),
        Space::with_height(Length::Fixed(8.0)),
        text("Check for smooth transitions without banding").size(10),
    ]
    .spacing(4)
    .into()
}

fn view_black_level<'a>() -> Element<'a, Message> {
    // Black level patches
    let levels: Vec<Element<Message>> = (0..8)
        .map(|i| {
            let _value = i as f32 / 100.0; // 0% to 7%
            column![
                container(Space::new(Length::Fixed(60.0), Length::Fixed(60.0)))
                    .style(iced::theme::Container::Box),
                text(format!("{}%", i)).size(10),
            ]
            .spacing(2)
            .align_items(iced::Alignment::Center)
            .into()
        })
        .collect();

    column![
        text("Black Level Test").size(14),
        text("Adjust brightness until patch 2-3 is barely visible").size(11),
        Space::with_height(Length::Fixed(8.0)),
        row(levels).spacing(8),
    ]
    .spacing(4)
    .into()
}

fn view_white_level<'a>() -> Element<'a, Message> {
    // White level patches
    let levels: Vec<Element<Message>> = (0..8)
        .map(|i| {
            let _value = 0.93 + (i as f32 / 100.0); // 93% to 100%
            column![
                container(Space::new(Length::Fixed(60.0), Length::Fixed(60.0)))
                    .style(iced::theme::Container::Box),
                text(format!("{}%", 93 + i)).size(10),
            ]
            .spacing(2)
            .align_items(iced::Alignment::Center)
            .into()
        })
        .collect();

    column![
        text("White Level Test").size(14),
        text("Adjust contrast until all patches are distinguishable").size(11),
        Space::with_height(Length::Fixed(8.0)),
        row(levels).spacing(8),
    ]
    .spacing(4)
    .into()
}

fn view_gamma<'a>() -> Element<'a, Message> {
    column![
        text("Gamma Test").size(14),
        text("The striped area should appear uniform gray at a distance").size(11),
        Space::with_height(Length::Fixed(8.0)),
        container(
            column![
                text("Target: γ = 2.2").size(12),
                Space::with_height(Length::Fixed(8.0)),
                container(Space::new(Length::Fixed(200.0), Length::Fixed(100.0)))
                    .style(iced::theme::Container::Box),
            ]
            .align_items(iced::Alignment::Center),
        )
        .width(Length::Fill)
        .center_x(),
    ]
    .spacing(4)
    .into()
}

fn view_white_balance<'a>() -> Element<'a, Message> {
    let temps = [
        ("5000K (Warm)", 5000),
        ("5500K", 5500),
        ("6500K (D65)", 6500),
        ("7500K", 7500),
        ("9300K (Cool)", 9300),
    ];

    let patches: Vec<Element<Message>> = temps
        .iter()
        .map(|(label, _temp)| {
            column![
                container(Space::new(Length::Fixed(80.0), Length::Fixed(80.0)))
                    .style(iced::theme::Container::Box),
                text(*label).size(10),
            ]
            .spacing(4)
            .align_items(iced::Alignment::Center)
            .into()
        })
        .collect();

    column![
        text("White Balance").size(14),
        text("Pure white should have no color tint at 6500K").size(11),
        Space::with_height(Length::Fixed(8.0)),
        row(patches).spacing(8),
    ]
    .spacing(4)
    .into()
}

fn view_resolution<'a>() -> Element<'a, Message> {
    column![
        text("Resolution Test").size(14),
        text("Lines should be crisp and distinguishable").size(11),
        Space::with_height(Length::Fixed(8.0)),
        container(text("|||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||").size(8),)
            .width(Length::Fill)
            .center_x(),
        container(text("=====================================").size(8),)
            .width(Length::Fill)
            .center_x(),
    ]
    .spacing(4)
    .into()
}

fn view_dead_pixel<'a>() -> Element<'a, Message> {
    let colors = [
        ("Red", Color::from_rgb(1.0, 0.0, 0.0)),
        ("Green", Color::from_rgb(0.0, 1.0, 0.0)),
        ("Blue", Color::from_rgb(0.0, 0.0, 1.0)),
        ("White", Color::WHITE),
        ("Black", Color::BLACK),
    ];

    let buttons: Vec<Element<Message>> = colors
        .iter()
        .map(|(label, _color)| {
            column![
                container(Space::new(Length::Fixed(60.0), Length::Fixed(60.0)))
                    .style(iced::theme::Container::Box),
                text(*label).size(10),
            ]
            .spacing(4)
            .align_items(iced::Alignment::Center)
            .into()
        })
        .collect();

    column![
        text("Dead Pixel Test").size(14),
        text("Use fullscreen mode and check each color for stuck pixels").size(11),
        Space::with_height(Length::Fixed(8.0)),
        row(buttons).spacing(8),
    ]
    .spacing(4)
    .into()
}
