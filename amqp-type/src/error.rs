use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("Encountered an IO Error.")]
    IoError(#[from] std::io::Error),
    #[error("Error while trying to deserialize value. Constructor {0:#04x} was invalid.")]
    DeserializationIllegalConstructorError(u8),
    #[error("Iterator was empty or too short.")]
    IteratorEmptyOrTooShortError,
}
