use error_stack::{Result, ResultExt};
use i18n_embed::{
    DesktopLanguageRequester,
    fluent::{fluent_language_loader, FluentLanguageLoader},
    LanguageLoader, unic_langid::LanguageIdentifier,
};
use once_cell::sync::Lazy;
use rust_embed::RustEmbed;

use crate::error::InternalError;

#[derive(RustEmbed)]
#[folder = "i18n/"]
struct Localizations;

pub static LANGUAGE_LOADER: Lazy<FluentLanguageLoader> =
    Lazy::new(|| fluent_language_loader!());


pub fn load_language(lang: Option<LanguageIdentifier>) -> Result<(), InternalError> {
    tracing::debug!("Loading language: {:?}", lang);

    LANGUAGE_LOADER.load_available_languages(&Localizations).change_context(InternalError::LanguageLoad)?;

    let mut requested_languages =
        DesktopLanguageRequester::requested_languages();

    if let Some(lang) = lang {
        tracing::debug!("Loading language: {:?}", lang);
        requested_languages.insert(0, lang);
    }

    i18n_embed::select(
        &*LANGUAGE_LOADER,
        &Localizations,
        &requested_languages,
    ).change_context(InternalError::LanguageLoad)?;

    tracing::debug!("Language loaded successfully");
    Ok(())
}


#[macro_export]
macro_rules! fl {
    ($message_id:literal) => {{
        i18n_embed_fl::fl!($crate::i18n::LANGUAGE_LOADER, $message_id)
    }};

    ($message_id:literal, $($args:expr),*) => {{
        i18n_embed_fl::fl!($crate::i18n::LANGUAGE_LOADER, $message_id, $($args), *)
    }};
}