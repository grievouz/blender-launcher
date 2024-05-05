#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

mod app;
mod blender;
mod error;
mod font;
mod i18n;
mod theme;
mod utils;
mod widgets;

use crate::app::{app_icon, App, AppErrorDialog, Tab};
use error_stack::{Result, ResultExt};
use iced::{window, Theme as IcedTheme};
use std::env;
use iced::window::Position::Default;
use tracing_subscriber::prelude::*;

use crate::error::InternalError;
use crate::theme::Theme;
use crate::utils::Patchable;

pub fn main() -> Result<(), InternalError> {
    if let Err(error) = run_app() {
        let error_msg = format!("{:#?}", error);

        tracing::error!("Error: {}", error_msg);
        // let _ = run_app_error_dialog(error_msg);

        return Err(error);
    }

    Ok(())
}

pub fn run_app() -> Result<(), InternalError> {
    env::set_var("RUST_BACKTRACE", "1");

    let subscriber = tracing_subscriber::Registry::default()
        .with(tracing_subscriber::EnvFilter::from_default_env())
        .with(tracing_subscriber::fmt::layer());

    tracing::subscriber::set_global_default(subscriber)
        .change_context(InternalError::LoggingInitialization)?;

    i18n::load_language(None)?;

    let theme = Theme::new();

    iced::program(|_: &_| fl!("window-title"), App::update, App::view)
        .settings(iced::Settings {
            window: window::Settings {
                transparent: false,
                decorations: false,
                resizable: false,
                icon: Some(app_icon()?),
                size: iced::Size {
                    width: 440.0,
                    height: 640.0,
                },
                ..Default::default()
            },
            ..Default::default()
        })
        .centered()
        .antialiasing(true)
        .theme(|_state| IcedTheme::Light)
        .patch(font::load_fonts)
        .default_font(font::inter(font::Weight::Normal))
        .run_with(move || App {
            theme,
            ..Default::default()
        })
        .change_context(InternalError::WindowSetup)?;

    Ok(())
}

pub fn run_app_error_dialog(error_msg: String) -> Result<(), InternalError> {
   todo!()
}