use thiserror;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    // #[error("Generic {0}")]
    // Generic(String),
    #[error("Program terminated with {0}")]
    ProgramTerminated(String),
}
