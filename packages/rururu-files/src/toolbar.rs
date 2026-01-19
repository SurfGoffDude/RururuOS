use crate::app::{Message, RururuFiles, ViewMode};
use iced::widget::{button, container, row, text, text_input, Space};
use iced::{Element, Length};

pub struct Toolbar;

impl Toolbar {
    pub fn view(app: &RururuFiles) -> Element<Message> {
        let nav_buttons = row![
            button(text("◀"))
                .on_press(Message::NavigateBack)
                .style(iced::theme::Button::Secondary),
            button(text("▶"))
                .on_press(Message::NavigateForward)
                .style(iced::theme::Button::Secondary),
            button(text("▲"))
                .on_press(Message::NavigateUp)
                .style(iced::theme::Button::Secondary),
            button(text("🏠"))
                .on_press(Message::NavigateHome)
                .style(iced::theme::Button::Secondary),
            button(text("🔄"))
                .on_press(Message::RefreshDirectory)
                .style(iced::theme::Button::Secondary),
        ]
        .spacing(4);

        let path_bar = container(
            text(app.current_path.to_string_lossy())
                .size(14)
        )
        .padding(8)
        .style(iced::theme::Container::Box)
        .width(Length::Fill);

        let search = text_input("Search...", &app.search_query)
            .on_input(Message::SearchChanged)
            .on_submit(Message::SearchSubmit)
            .width(Length::Fixed(200.0));

        let view_buttons = row![
            button(text("☰"))
                .on_press(Message::SetViewMode(ViewMode::List))
                .style(if app.view_mode == ViewMode::List {
                    iced::theme::Button::Primary
                } else {
                    iced::theme::Button::Secondary
                }),
            button(text("▦"))
                .on_press(Message::SetViewMode(ViewMode::Grid))
                .style(if app.view_mode == ViewMode::Grid {
                    iced::theme::Button::Primary
                } else {
                    iced::theme::Button::Secondary
                }),
        ]
        .spacing(4);

        let options = row![
            button(if app.show_hidden { text("👁") } else { text("👁‍🗨") })
                .on_press(Message::ToggleHiddenFiles)
                .style(iced::theme::Button::Secondary),
            button(if app.show_preview { text("◧") } else { text("▢") })
                .on_press(Message::TogglePreview)
                .style(iced::theme::Button::Secondary),
        ]
        .spacing(4);

        let toolbar = row![
            nav_buttons,
            Space::with_width(Length::Fixed(16.0)),
            path_bar,
            Space::with_width(Length::Fixed(16.0)),
            search,
            Space::with_width(Length::Fixed(16.0)),
            view_buttons,
            Space::with_width(Length::Fixed(8.0)),
            options,
        ]
        .spacing(8)
        .align_items(iced::Alignment::Center);

        container(toolbar)
            .padding(8)
            .width(Length::Fill)
            .style(iced::theme::Container::Box)
            .into()
    }
}
