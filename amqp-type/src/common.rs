use crate::error::AppError;

pub fn verify_bytes_read_eq(actual: usize, expected: usize) -> Result<(), AppError> {
    if actual == expected {
        Ok(())
    } else {
        Err(AppError::IteratorEmptyOrTooShortError)
    }
}

/// reads the passed number of bytes from the passed stream.
/// ensures that exactly the expected number of bytes is read, and returns Err otherwise
pub fn read_bytes(iter: &mut impl Iterator<Item=u8>, size: usize) -> Result<Vec<u8>, AppError> {
    let mut res = Vec::with_capacity(size);
    let mut read = 0;
    while let Some(byte) = iter.next() {
        res.push(byte);
        read += 1;
        if read == size {
            break;
        }
    }
    verify_bytes_read_eq(read, size)?;
    Ok(res)
}

pub fn read_bytes_2(iter: &mut impl Iterator<Item=u8>) -> Result<[u8; 2], AppError> {
    Ok(read_bytes(iter, 2)?.try_into().unwrap())
}

pub fn read_bytes_4(iter: &mut impl Iterator<Item=u8>) -> Result<[u8; 4], AppError> {
    Ok(read_bytes(iter, 4)?.try_into().unwrap())
}

pub fn read_bytes_8(iter: &mut impl Iterator<Item=u8>) -> Result<[u8; 8], AppError> {
    Ok(read_bytes(iter, 8)?.try_into().unwrap())
}

pub fn read_bytes_16(iter: &mut impl Iterator<Item=u8>) -> Result<[u8; 16], AppError> {
    Ok(read_bytes(iter, 16)?.try_into().unwrap())
}