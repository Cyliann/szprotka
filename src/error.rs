use thiserror;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Cancelled")]
    Cancelled,

    #[error(transparent)]
    IO(#[from] std::io::Error),

    #[error(transparent)]
    Request(#[from] reqwest::Error),

    #[error("Error obtaining the token")]
    Token,

    #[error("Error streaming events: {0}")]
    EventsourceClient(#[from] eventsource_client::Error),

    #[error("Error streaming events: {0}")]
    JoinError(#[from] tokio::task::JoinError),
}
