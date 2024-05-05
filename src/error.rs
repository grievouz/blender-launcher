use thiserror::Error;

#[derive(Error, Debug)]
pub enum InternalError {
    #[error("failed window setup")]
    WindowSetup,
    #[error("failed to load language")]
    LanguageLoad,
    #[error("failed to parse icon")]
    IconParsing,
    #[error("failed to initialize logging")]
    LoggingInitialization,
}
