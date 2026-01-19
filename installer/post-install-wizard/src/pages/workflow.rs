use iced::{
    widget::{button, column, container, row, text, vertical_space},
    Element, Length,
};
use rururu_workflows::WorkflowType;
use crate::wizard::Message;

pub struct WorkflowPage {
    pub selected: Option<WorkflowType>,
}

impl WorkflowPage {
    pub fn new() -> Self {
        Self { selected: None }
    }
    
    pub fn view(&self) -> Element<Message> {
        let workflows = [
            (WorkflowType::VideoEditor, "🎬", "Video Editor", 
             "DaVinci Resolve, Kdenlive, Handbrake\nOptimized for video editing and encoding"),
            (WorkflowType::ThreeDArtist, "🎨", "3D Artist",
             "Blender, FreeCAD\nGPU-accelerated rendering, ACES color"),
            (WorkflowType::TwoDDesigner, "✏️", "2D Designer",
             "Krita, GIMP, Inkscape\nDigital painting and vector graphics"),
            (WorkflowType::AudioProducer, "🎵", "Audio Producer",
             "Ardour, Bitwig, Audacity\nLow-latency audio, JACK routing"),
            (WorkflowType::Photographer, "📷", "Photographer",
             "Darktable, RawTherapee, digiKam\nRAW processing, color management"),
            (WorkflowType::General, "💻", "General Creative",
             "Balanced configuration\nGood for mixed workflows"),
        ];
        
        let cards: Vec<Element<Message>> = workflows.iter().map(|(wf, icon, name, desc)| {
            let is_selected = self.selected == Some(*wf);
            let style = if is_selected {
                iced::theme::Button::Primary
            } else {
                iced::theme::Button::Secondary
            };
            
            button(
                column![
                    text(*icon).size(40),
                    text(*name).size(16),
                    text(*desc).size(12),
                ]
                .spacing(5)
                .align_items(iced::Alignment::Center)
                .width(Length::Fill)
            )
            .on_press(Message::WorkflowSelected(*wf))
            .style(style)
            .width(250)
            .height(150)
            .into()
        }).collect();
        
        let row1: Vec<Element<Message>> = cards.into_iter().take(3).collect();
        let row2: Vec<Element<Message>> = workflows.iter().skip(3).map(|(wf, icon, name, desc)| {
            let is_selected = self.selected == Some(*wf);
            let style = if is_selected {
                iced::theme::Button::Primary
            } else {
                iced::theme::Button::Secondary
            };
            
            button(
                column![
                    text(*icon).size(40),
                    text(*name).size(16),
                    text(*desc).size(12),
                ]
                .spacing(5)
                .align_items(iced::Alignment::Center)
                .width(Length::Fill)
            )
            .on_press(Message::WorkflowSelected(*wf))
            .style(style)
            .width(250)
            .height(150)
            .into()
        }).collect();
        
        container(
            column![
                text("Select Your Workflow").size(24),
                vertical_space().height(10),
                text("Choose the workflow that best matches your creative needs."),
                text("This will optimize system settings and suggest applications."),
                vertical_space().height(30),
                row(row1).spacing(20),
                vertical_space().height(20),
                row(row2).spacing(20),
                vertical_space().height(20),
                if let Some(wf) = self.selected {
                    text(format!("Selected: {}", wf.name())).size(16)
                } else {
                    text("Please select a workflow to continue").size(16)
                },
            ]
            .spacing(10)
            .align_items(iced::Alignment::Center)
        )
        .width(Length::Fill)
        .center_x()
        .into()
    }
}
