use thiserror;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    // #[error("Generic {0}")]
    // Generic(String),
    #[error("Program terminated with {0}")]
    ProgramTerminated(String),

    #[error(transparent)]
    IO(#[from] std::io::Error),

    #[error(transparent)]
    Request(#[from] reqwest::Error),

    #[error("Error obtaining the token")]
    Token,
}
