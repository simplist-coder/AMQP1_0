use std::pin::Pin;
use tokio_stream::{Stream, StreamExt};
use crate::common::{read_bytes, read_bytes_4};
use crate::constants::constructors::{BINARY, BINARY_SHORT};
use crate::error::AppError;
use crate::serde::decode::Decode;
use crate::serde::encode::{Encode, Encoded};


#[derive(Hash, Eq, PartialEq)]
pub struct Binary(Vec<u8>);

impl Encode for Binary {
    fn encode(&self) -> Encoded {
        match self.0.len() {
            x if x <= 255 => Encoded::new_variable(BINARY_SHORT, self.0.to_owned()),
            _ => Encoded::new_variable(BINARY, self.0.to_owned()),
        }
    }
}

impl Decode for Binary {
    async fn can_decode(iter: Pin<Box<impl Stream<Item=u8>>>) -> bool {
        match iter.peekable().peek().await {
            Some(&BINARY_SHORT) => true,
            Some(&BINARY) => true,
            _ => false,
        }
    }

    async fn try_decode(mut iter: Pin<Box<impl Stream<Item=u8>>>) -> Result<Self, AppError> where Self: Sized {
        match iter.next().await {
            Some(BINARY_SHORT) => Ok(parse_small_binary(&mut iter).await?),
            Some(BINARY) => Ok(parse_large_binary(&mut iter).await?),
            Some(illegal) => Err(AppError::DeserializationIllegalConstructorError(illegal)),
            None => Err(AppError::IteratorEmptyOrTooShortError),
        }
    }
}

async fn parse_small_binary(iter: &mut Pin<Box<impl Stream<Item=u8>>>) -> Result<Binary, AppError> {
    match iter.next().await {
        Some(size) => Ok(Binary(read_bytes(iter, size as usize).await?)),
        None => Err(AppError::IteratorEmptyOrTooShortError),
    }
}

async fn parse_large_binary(iter: &mut Pin<Box<impl Stream<Item=u8>>>) -> Result<Binary, AppError> {
    let size = u32::from_be_bytes(read_bytes_4(iter).await?);
    Ok(Binary(read_bytes(iter, size as usize).await?))
}

impl From<Vec<u8>> for Binary {
    fn from(value: Vec<u8>) -> Self {
        Binary(value)
    }
}

#[cfg(test)]
mod test {
    use crate::common::tests::ByteVecExt;
    use super::*;

    #[test]
    fn construct_binary() {
        let val = Binary(Vec::new());
        assert_eq!(val.encode().constructor(), 0xa0);
    }

    #[test]
    fn test_encode_short_data() {
        let data = vec![0; 255]; // 255 bytes of data
        let binary = Binary(data.clone());
        let encoded = binary.encode();
        let mut expected = vec![BINARY_SHORT];
        let x = (data.len() as u8).to_be_bytes();
        expected.append(&mut x.to_vec());
        expected.append(&mut data.clone());

        assert_eq!(encoded.constructor(), BINARY_SHORT);
        assert_eq!(encoded.to_bytes(), expected);
    }

    #[test]
    fn test_encode_long_data() {
        let data = vec![0; 256]; // 256 bytes of data
        let binary = Binary(data.clone());
        let encoded = binary.encode();
        let mut expected = vec![BINARY];
        let x = (data.len() as u32).to_be_bytes();
        expected.append(&mut x.to_vec());
        expected.append(&mut data.clone());

        assert_eq!(encoded.constructor(), BINARY);
        assert_eq!(encoded.to_bytes(), expected);
    }

    #[tokio::test]
    async fn test_decode_small_binary() {
        let data = vec![BINARY_SHORT, 3, 0x01, 0x02, 0x03];
        let result = Binary::try_decode(data.into_pinned_stream()).await.unwrap();
        assert_eq!(result.0, vec![0x01, 0x02, 0x03]);
    }

    #[tokio::test]
    async fn test_decode_large_binary() {
        let size_bytes = (4u32).to_be_bytes();
        let data = vec![
            BINARY,
            size_bytes[0],
            size_bytes[1],
            size_bytes[2],
            size_bytes[3],
            0x01, 0x02, 0x03, 0x04
        ];
        let result = Binary::try_decode(data.into_pinned_stream()).await.unwrap();
        assert_eq!(result.0, vec![0x01, 0x02, 0x03, 0x04]);
    }

    #[tokio::test]
    async fn test_illegal_constructor() {
        let data = vec![0xFF, 3, 0x01, 0x02, 0x03];
        let result = Binary::try_decode(data.into_pinned_stream()).await;
        assert!(matches!(result, Err(AppError::DeserializationIllegalConstructorError(0xFF))));
    }

    #[tokio::test]
    async fn test_iterator_empty_or_too_short() {
        let data: Vec<u8> = vec![];
        let result = Binary::try_decode(data.into_pinned_stream()).await;
        assert!(matches!(result, Err(AppError::IteratorEmptyOrTooShortError)));
    }
}
