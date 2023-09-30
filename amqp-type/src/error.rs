use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("Encountered an IO Error.")]
    IoError(#[from] std::io::Error),
    #[error("Error while trying to deserialize value of type `{0}`. Reason: {1}")]
    DeserializationError(String, String)
    
}
