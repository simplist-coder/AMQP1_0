use crate::error::AppError;

pub fn verify_bytes_read_eq(actual: usize, expected: usize) -> Result<(), AppError> {
    if actual == expected {
        Ok(())
    } else {
        Err(AppError::IteratorEmptyOrTooShortError)
    }
}


pub fn read_bytes_8(iter: impl Iterator<Item=u8> + Sized) -> Result<[u8; 8], AppError> {
    let mut byte_vals = [0; 8];
    let mut index = 0;
    for b in iter.take(8) {
        byte_vals[index] = b;
        index += 1;
    }
    verify_bytes_read_eq(index, 8)?;
    Ok(byte_vals)
}

pub fn read_bytes_4(iter: impl Iterator<Item=u8> + Sized + Sized) -> Result<[u8; 4], AppError> {
    let mut byte_vals = [0; 4];
    let mut index = 0;
    for b in iter.take(4) {
        byte_vals[index] = b;
        index += 1;
    }
    verify_bytes_read_eq(index, 4)?;
    Ok(byte_vals)
}

pub fn read_bytes_2(iter: impl Iterator<Item=u8> + Sized) -> Result<[u8; 2], AppError> {
    let mut val_bytes = [0; 2];
    let mut index = 0;
    for b in iter.take(2) {
        val_bytes[index] = b;
        index += 1;
    }
    verify_bytes_read_eq(index, 2)?;
    Ok(val_bytes)
}