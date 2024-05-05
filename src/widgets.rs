use std::borrow::Cow;
use iced::widget::{button, container, row, text, Button, Container, svg};
use iced::Theme;
use iced::{color, Border, Color, Element, Length, Shadow};
use iced::border::Radius;

use crate::font;

pub fn tabs_header<'a, Message: 'a>(
    buttons: impl IntoIterator<Item = Element<'a, Message>>,
) -> Container<'a, Message> {
    container(row(buttons))
        .padding(4)
        .style(tabs_header_style)
        .width(Length::Fill)
        .center_x()
}

fn tabs_header_style(_theme: &Theme) -> container::Style {
    container::Style {
        background: Some(color!(0xF4F4F5).into()),
        border: Border {
            color: color!(0xE5E7EB),
            width: 1.0,
            ..Default::default()
        },
        ..Default::default()
    }
}

pub fn tabs_button<'a, Message>(
    is_active: bool,
    content: impl text::IntoFragment<'a>,
) -> Button<'a, Message> {
    button(
        text(content)
            .size(15)
            .font(font::inter(font::Weight::Medium)),
    )
    .padding([6, 14])
    .style(tabs_button_style(is_active))
}

type ButtonStyleFn<'a> = dyn Fn(&Theme, button::Status) -> button::Style + 'a;

fn tabs_button_style<'a>(is_selected: bool) -> Box<ButtonStyleFn<'a>> {
    if is_selected {
        Box::new(|_theme, _status| button::Style {
            background: Some(color!(0xFFFFFF).into()),
            text_color: color!(0x09090B),
            border: Border::rounded(6),
            shadow: Shadow {
                color: Color {
                    r: 0.0,
                    g: 0.0,
                    b: 0.0,
                    a: 0.2,
                },
                offset: iced::Vector { x: 0.0, y: 1.0 },
                blur_radius: 5.0,
            },
        })
    } else {
        Box::new(|_theme, _status| button::Style {
            background: Some(Color::TRANSPARENT.into()),
            text_color: color!(0x71717A),
            ..Default::default()
        })
    }
}

pub fn icon_button<'a, Message>(
    bytes: impl Into<Cow<'static, [u8]>>,
) -> Button<'a, Message> {
    let content = svg(svg::Handle::from_memory(bytes))
        .height(20)
        .width(20);
    
    button(content)
        .padding(10)
        .style(icon_button_style)
}

fn icon_button_style(_theme: &Theme, status: button::Status) -> button::Style {
    if status == button::Status::Hovered || status == button::Status::Pressed {
        button::Style {
            background: Some(Color {
                r: 0.0,
                g: 0.0,
                b: 0.0,
                a: 0.1,
            }.into()),
            border: Border {
                color: Color::TRANSPARENT,
                width: 0.0,
                radius: Radius::from(360)
            },
            ..Default::default()
        }
    } else {
        button::Style {
            background: Some(Color::TRANSPARENT.into()),
            border: Border {
                color: Color::TRANSPARENT,
                width: 0.0,
                radius: Radius::from(360)
            },
            ..Default::default()
        }

    }
}