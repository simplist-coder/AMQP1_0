use std::pin::Pin;
use tokio_stream::{Stream, StreamExt};
use crate::common::{read_bytes, read_bytes_4};
use crate::constants::constructors::{STRING, STRING_SHORT};
use crate::error::AppError;
use crate::serde::decode::Decode;
use crate::serde::encode::{Encode, Encoded};



impl Encode for String {
    fn encode(&self) -> Encoded {
        match self.len() as i32 {
            x if x >= 0 && x <= 255 => Encoded::new_variable(STRING_SHORT, self.as_bytes().to_vec()),
            _ => Encoded::new_variable(STRING, self.as_bytes().to_vec()),
        }
    }
}

impl Decode for String {
    async fn can_decode(stream: Pin<Box<impl Stream<Item=u8>>>) -> bool {
        match stream.peekable().peek().await {
            Some(&STRING_SHORT) => true,
            Some(&STRING) => true,
            _ => false,
        }
    }

    async fn try_decode(mut stream: Pin<Box<impl Stream<Item=u8>>>) -> Result<Self, AppError> where Self: Sized {
        match stream.next().await {
            Some(STRING_SHORT) => Ok(parse_small_string(&mut stream).await?),
            Some(STRING) => Ok(parse_large_string(&mut stream).await?),
            Some(illegal) => Err(AppError::DeserializationIllegalConstructorError(illegal)),
            None => Err(AppError::IteratorEmptyOrTooShortError),
        }
    }
}

async fn parse_small_string(iter: &mut Pin<Box<impl Stream<Item=u8>>>) -> Result<String, AppError> {
    match iter.next().await {
        Some(size) => Ok(String::from_utf8(read_bytes(iter, size as usize).await?)?),
        None => Err(AppError::IteratorEmptyOrTooShortError),
    }
}

async fn parse_large_string(iter: &mut Pin<Box<impl Stream<Item=u8>>>) -> Result<String, AppError> {
    let size = u32::from_be_bytes(read_bytes_4(iter).await?);
    Ok(String::from_utf8(read_bytes(iter, size as usize).await?)?)
}

#[cfg(test)]
mod test {
    use crate::common::tests::ByteVecExt;
    use super::*;

    #[test]
    fn test_encode_empty_string() {
        let empty_string = String::new();
        let encoded = empty_string.encode().to_bytes();

        let mut expected = vec![STRING_SHORT];
        let len = empty_string.into_bytes().len() as u8;
        expected.append(&mut len.to_be_bytes().to_vec());

        assert_eq!(encoded, expected);
    }

    #[test]
    fn test_encode_small_string() {
        let small_string = "Test".to_string();
        let encoded = small_string.encode().to_bytes();

        let mut expected = vec![STRING_SHORT];
        let mut bytes = small_string.into_bytes();
        expected.append(&mut (bytes.len() as u8).to_be_bytes().to_vec());
        expected.append(&mut bytes);

        assert_eq!(encoded, expected);
    }

    #[test]
    fn test_encode_boundary_string() {
        let boundary_string = "a".repeat(255);
        let encoded = boundary_string.encode().to_bytes();

        let mut expected = vec![STRING_SHORT];
        let mut bytes = boundary_string.into_bytes();
        expected.append(&mut (bytes.len() as u8).to_be_bytes().to_vec());
        expected.append(&mut bytes);

        assert_eq!(encoded, expected);
    }

    #[test]
    fn test_encode_large_string() {
        let large_string = "a".repeat(256);
        let encoded = large_string.encode().to_bytes();

        let mut expected = vec![STRING];
        let mut bytes = large_string.into_bytes();
        expected.append(&mut (bytes.len() as u32).to_be_bytes().to_vec());
        expected.append(&mut bytes);

        assert_eq!(encoded, expected);
    }

    #[tokio::test]
    async fn test_decode_small_string() {
        let data = vec![STRING_SHORT, 5, b'H', b'e', b'l', b'l', b'o'];
        let result = String::try_decode(data.into_pinned_stream()).await.unwrap();
        assert_eq!(result, "Hello".to_string());
    }

    #[tokio::test]
    async fn test_decode_large_string() {
        let size_bytes = 11u32.to_be_bytes();
        let mut data = vec![
            STRING,
            size_bytes[0],
            size_bytes[1],
            size_bytes[2],
            size_bytes[3],
        ];
        data.extend_from_slice(b"Hello World");
        let result = String::try_decode(data.into_pinned_stream()).await.unwrap();
        assert_eq!(result, "Hello World".to_string());
    }

    #[tokio::test]
    async fn test_illegal_constructor() {
        let data = vec![0xFF, 5, b'E', b'r', b'r', b'o', b'r'];
        let result = String::try_decode(data.into_pinned_stream()).await;
        assert!(matches!(result, Err(AppError::DeserializationIllegalConstructorError(0xFF))));
    }

    #[tokio::test]
    async fn test_iterator_empty_or_too_short() {
        let data = vec![];
        let result = String::try_decode(data.into_pinned_stream()).await;
        assert!(matches!(result, Err(AppError::IteratorEmptyOrTooShortError)));
    }

    #[tokio::test]
    async fn test_utf8_compliance() {
        let data = vec![STRING_SHORT, 2, 0xC3, 0xA9]; // 'é' in UTF-8
        let result = String::try_decode(data.into_pinned_stream()).await.unwrap();
        assert_eq!(result, "é".to_string());
    }

}
