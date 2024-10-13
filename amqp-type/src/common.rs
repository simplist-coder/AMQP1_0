use crate::error::AppError;
use std::pin::Pin;
use tokio_stream::Stream;
use tokio_stream::StreamExt;

pub fn verify_bytes_read_eq(actual: usize, expected: usize) -> Result<(), AppError> {
    if actual == expected {
        Ok(())
    } else {
        Err(AppError::IteratorEmptyOrTooShortError)
    }
}

/// reads the passed number of bytes from the passed stream.
/// ensures that exactly the expected number of bytes is read, and returns Err otherwise
pub(crate) async fn read_bytes(
    iter: &mut Pin<Box<impl Stream<Item = u8>>>,
    size: usize,
) -> Result<Vec<u8>, AppError> {
    let mut res = Vec::with_capacity(size);
    let mut read = 0;
    while let Some(byte) = iter.next().await {
        res.push(byte);
        read += 1;
        if read == size {
            break;
        }
    }
    verify_bytes_read_eq(read, size)?;
    Ok(res)
}

pub(crate) async fn read_bytes_2(
    iter: &mut Pin<Box<impl Stream<Item = u8>>>,
) -> Result<[u8; 2], AppError> {
    Ok(read_bytes(iter, 2).await?.try_into().unwrap())
}

pub(crate) async fn read_bytes_4(
    iter: &mut Pin<Box<impl Stream<Item = u8>>>,
) -> Result<[u8; 4], AppError> {
    Ok(read_bytes(iter, 4).await?.try_into().unwrap())
}

pub(crate) async fn read_bytes_8(
    iter: &mut Pin<Box<impl Stream<Item = u8>>>,
) -> Result<[u8; 8], AppError> {
    Ok(read_bytes(iter, 8).await?.try_into().unwrap())
}

pub(crate) async fn read_bytes_16(
    iter: &mut Pin<Box<impl Stream<Item = u8>>>,
) -> Result<[u8; 16], AppError> {
    Ok(read_bytes(iter, 16).await?.try_into().unwrap())
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;

    pub(crate) trait ByteVecExt {
        fn into_pinned_stream(self) -> Pin<Box<impl Stream<Item = u8>>>;
    }

    impl ByteVecExt for Vec<u8> {
        fn into_pinned_stream(self) -> Pin<Box<impl Stream<Item = u8>>> {
            Box::pin(tokio_stream::iter(self))
        }
    }
}
