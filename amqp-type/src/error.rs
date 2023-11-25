use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("Encountered an IO Error.")]
    IoError(#[from] std::io::Error),
    #[error("Error while trying to deserialize value. Constructor {0:#04x} was invalid.")]
    DeserializationIllegalConstructorError(u8),
    #[error("Iterator was empty or too short.")]
    IteratorEmptyOrTooShortError,
    #[error("Byte value was not a valid char")]
    InvalidChar,
    #[error("Bytes cannot be transformed into valid utf8 string")]
    FromUtf8ConversionError(#[from] std::string::FromUtf8Error),
}
