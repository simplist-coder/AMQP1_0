use crate::constants::constructors::{BINARY, BINARY_SHORT};
use crate::serde::decode::Decode;
use crate::serde::encode::{Encode, Encoded};
use amqp_error::AppError;
use amqp_utils::{read_bytes, read_bytes_4};
use std::pin::Pin;
use tokio_stream::{Stream, StreamExt};

#[derive(Debug, Hash, Eq, PartialEq, Clone)]
pub struct Binary(Vec<u8>);

impl Encode for Binary {
    fn encode(self) -> Encoded {
        match self.0.len() {
            x if x <= 255 => Encoded::new_variable(BINARY_SHORT, self.0.clone()),
            _ => Encoded::new_variable(BINARY, self.0.clone()),
        }
    }
}

impl Decode for Binary {
    async fn try_decode(
        constructor: u8,
        stream: &mut Pin<Box<impl Stream<Item = u8>>>,
    ) -> Result<Self, AppError>
    where
        Self: Sized,
    {
        match constructor {
            BINARY_SHORT => Ok(parse_small_binary(stream).await?),
            BINARY => Ok(parse_large_binary(stream).await?),
            illegal => Err(AppError::DeserializationIllegalConstructorError(illegal)),
        }
    }
}

async fn parse_small_binary(
    iter: &mut Pin<Box<impl Stream<Item = u8>>>,
) -> Result<Binary, AppError> {
    match iter.next().await {
        Some(size) => Ok(Binary(read_bytes(iter, size as usize).await?)),
        None => Err(AppError::IteratorEmptyOrTooShortError),
    }
}

async fn parse_large_binary(
    iter: &mut Pin<Box<impl Stream<Item = u8>>>,
) -> Result<Binary, AppError> {
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
    use super::*;
    use amqp_utils::ByteVecExt;

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
        assert_eq!(encoded.into_bytes(), expected);
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
        assert_eq!(encoded.into_bytes(), expected);
    }

    #[tokio::test]
    async fn test_decode_small_binary() {
        let data = vec![3, 0x01, 0x02, 0x03];
        let result = Binary::try_decode(BINARY_SHORT, &mut data.into_pinned_stream())
            .await
            .unwrap();
        assert_eq!(result.0, vec![0x01, 0x02, 0x03]);
    }

    #[tokio::test]
    async fn test_decode_large_binary() {
        let size_bytes = (4u32).to_be_bytes();
        let data = vec![
            size_bytes[0],
            size_bytes[1],
            size_bytes[2],
            size_bytes[3],
            0x01,
            0x02,
            0x03,
            0x04,
        ];
        let result = Binary::try_decode(BINARY, &mut data.into_pinned_stream())
            .await
            .unwrap();
        assert_eq!(result.0, vec![0x01, 0x02, 0x03, 0x04]);
    }

    #[tokio::test]
    async fn test_illegal_constructor() {
        let data = vec![3, 0x01, 0x02, 0x03];
        let result = Binary::try_decode(0xFF, &mut data.into_pinned_stream()).await;
        assert!(matches!(
            result,
            Err(AppError::DeserializationIllegalConstructorError(0xFF))
        ));
    }

    #[tokio::test]
    async fn test_iterator_empty_or_too_short() {
        let data: Vec<u8> = vec![];
        let result = Binary::try_decode(BINARY_SHORT, &mut data.into_pinned_stream()).await;
        assert!(matches!(
            result,
            Err(AppError::IteratorEmptyOrTooShortError)
        ));
    }
}
