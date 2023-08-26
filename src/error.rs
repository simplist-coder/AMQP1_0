use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("Malformed Frame: {0}")]
    MalformedFrame(String),
}
