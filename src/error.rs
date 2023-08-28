use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
pub enum AppError {
    #[error("Malformed Frame: {0}")]
    MalformedFrame(&'static str),
}
