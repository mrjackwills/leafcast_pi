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
    #[error("WS Connect")]
    TungsteniteConnect(#[from] tokio_tungstenite::tungstenite::Error),
    #[error("Invalid WS Status Code")]
    WsStatus,
}
