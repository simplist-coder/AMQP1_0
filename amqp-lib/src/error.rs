use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("Malformed Frame: {0}")]
    MalformedFrame(&'static str),
    #[error("Encountered an IO Error.")]
    IoError(#[from] std::io::Error),
}
