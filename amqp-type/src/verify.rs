use crate::error::AppError;

pub fn verify_bytes_read_eq(actual: usize, expected: usize) -> Result<(), AppError> {
    if actual == expected {
        Ok(())
    } else {
        Err(AppError::IteratorEmptyOrTooShortError)
    }
}
