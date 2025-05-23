use thiserror::Error;

#[derive(Debug, Error)]
pub enum AppError {
    #[error("'{0}' - file not found'")]
    FileNotFound(String),
    #[error("IO Error")]
    IOError(#[from] std::io::Error),
    #[error("missing env: '{0}'")]
    MissingEnv(String),
    #[error("Reqwest Error")]
    Reqwest(#[from] reqwest::Error),
    #[error("Unable to set up tracing")]
    Tracing,
    #[error("Ws Connect: '{0}'")]
    TungsteniteConnect(String),
    #[error("Invalid WS Status Code")]
    WsStatus,
}
