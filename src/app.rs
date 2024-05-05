use iced::{Alignment, Command, Element, Length, Subscription, window};
use iced::alignment::Vertical;
use iced::widget::{Column, container, horizontal_space, MouseArea, row, text};
use strum::IntoEnumIterator;
use strum_macros::EnumIter;
use error_stack::{Result, ResultExt};
use iced::window::{icon, Icon};

use crate::{fl, font};
use crate::blender::Versions;
use crate::error::InternalError;
use crate::theme::Theme;
use crate::widgets::{icon_button, tabs_button, tabs_header};

#[derive(Default, Debug, Clone, Copy, EnumIter, PartialEq, Eq)]
pub enum Tab {
    #[default]
    Stable,
    Experimental,
    Patch,
}

impl From<Tab> for String {
    fn from(value: Tab) -> Self {
        match value {
            Tab::Stable => fl!("tabs-stable"),
            Tab::Experimental => fl!("tabs-experimental"),
            Tab::Patch => fl!("tabs-patch"),
        }
    }
}

#[derive(Debug, Clone)]
pub enum Message {
    SwitchTab(Tab),
    SettingsOpen,
    WindowDrag,
    WindowClose,
}

pub struct App {
    pub current_tab: Tab,
    pub theme: Theme,
    pub versions: Versions
}

impl App {
    pub fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            Message::SwitchTab(tab) => {
                self.current_tab = tab;
                Command::none()
            }
            Message::SettingsOpen => Command::none(),
            Message::WindowDrag => window::drag(window::Id::MAIN),
            Message::WindowClose => window::close(window::Id::MAIN),
        }
    }

    pub fn subscription(&self) -> Subscription<Message> {
        Subscription::from_recipe(self.versions);
    }

    pub fn view(&self) -> Element<Message> {
        let title = text(fl!("window-title"))
            .font(font::inter(font::Weight::Bold))
            .size(26);

        let settings_button =
            icon_button(include_bytes!("resources/icons/settings.svg"))
                .on_press(Message::SettingsOpen);

        let close_button =
            icon_button(include_bytes!("resources/icons/close.svg"))
                .on_press(Message::WindowClose);

        let title_bar = MouseArea::new(
            container(
                row(vec![
                    title
                        .width(Length::Fill)
                        .vertical_alignment(Vertical::Center)
                        .into(),
                    settings_button.into(),
                    horizontal_space().width(10).into(),
                    close_button.into(),
                ])
                    .align_items(Alignment::Center),
            )
                .padding([16, 20])
                .width(Length::Fill),
        )
            .on_press(Message::WindowDrag);

        let tabs_list = tabs_header(
            Tab::iter()
                .map(|tab| {
                    tabs_button(self.current_tab == tab, String::from(tab))
                        .on_press(Message::SwitchTab(tab))
                        .into()
                })
                .collect::<Vec<Element<Message>>>(),
        );

        let tab = match self.current_tab {
            Tab::Stable => text(String::from(Tab::Stable)),
            Tab::Experimental => text(String::from(Tab::Experimental)),
            Tab::Patch => text(String::from(Tab::Patch)),
        };

        container(Column::with_children(vec![
            title_bar.into(),
            tabs_list.into(),
            tab.into(),
        ]))
            .width(Length::Fill)
            .height(Length::Fill)
            .into()
    }
}

pub fn app_icon() -> Result<Icon, InternalError> {
    let image = image::load_from_memory(include_bytes!(
        "resources/icons/bl_16x16.png"
    ))
        .change_context(InternalError::IconParsing)?
        .into_rgba8();
    let (width, height) = image.dimensions();
    let rgba = image.into_raw();

    icon::from_rgba(rgba, width, height)
        .change_context(InternalError::IconParsing)
}

pub struct AppErrorDialog {
    pub error_msg: String
}

impl AppErrorDialog {
    pub fn update(&mut self, _message: Message) -> Command<Message> {
        todo!()
    }

    pub fn view(&self) -> Element<Message> {
        todo!()
    }
}