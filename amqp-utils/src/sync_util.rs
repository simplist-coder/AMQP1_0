use crate::validation::verify_bytes_read_eq;
use amqp_error::AppError;
use std::vec::IntoIter;

pub fn read_bytes(
    iter: &mut IntoIter<u8>,
    size: usize, // TODO: change to u32
) -> Result<Vec<u8>, AppError> {
    let mut res = Vec::with_capacity(size);
    let mut read = 0;
    for byte in iter.by_ref() {
        res.push(byte);
        read += 1;
        if read == size {
            break;
        }
    }
    verify_bytes_read_eq(read, size)?;
    Ok(res)
}

pub fn read_bytes_2(iter: &mut IntoIter<u8>) -> Result<[u8; 2], AppError> {
    Ok(read_bytes(iter, 2)?.try_into().unwrap())
}

pub fn read_bytes_4(iter: &mut IntoIter<u8>) -> Result<[u8; 4], AppError> {
    Ok(read_bytes(iter, 4)?.try_into().unwrap())
}

pub fn read_bytes_8(iter: &mut IntoIter<u8>) -> Result<[u8; 8], AppError> {
    Ok(read_bytes(iter, 8)?.try_into().unwrap())
}

pub fn read_bytes_16(iter: &mut IntoIter<u8>) -> Result<[u8; 16], AppError> {
    Ok(read_bytes(iter, 16)?.try_into().unwrap())
}
