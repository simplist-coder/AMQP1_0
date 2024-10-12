use crate::common::read_bytes_4;
use crate::constants::constructors::CHAR;
use crate::error::AppError;
use crate::fixed_width::char;
use crate::serde::decode::Decode;
use crate::serde::encode::{Encode, Encoded};
use std::pin::Pin;
use tokio_stream::Stream;


impl Encode for char {
    fn encode(&self) -> Encoded {
        Encoded::new_fixed(CHAR, self.to_string().into_bytes())
    }
}

impl Decode for char {

    async fn try_decode(constructor: u8, mut iter: Pin<Box<impl Stream<Item=u8>>>) -> Result<Self, AppError> where Self: Sized {
        match constructor {
            CHAR => Ok(parse_char(&mut iter).await?),
            c => Err(AppError::DeserializationIllegalConstructorError(c)),
        }
    }
}

async fn parse_char(iter: &mut Pin<Box<impl Stream<Item=u8>>>) -> Result<char, AppError> {
    let byte_vals = read_bytes_4(iter).await?;
    match char::from_u32(u32::from_be_bytes(byte_vals)) {
        None => Err(AppError::InvalidChar),
        Some(c) => Ok(c)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::common::tests::ByteVecExt;
    use crate::constants::constructors::CHAR;

    #[test]
    fn construct_char() {
        let val = 'a';
        assert_eq!(val.encode().constructor(), 0x73);
    }

    #[test]
    fn test_encode_char() {
        let test_cases = [
            ('a', vec![CHAR, 0x61]),            // Test with a basic ASCII character
            ('ñ', vec![CHAR, 0xc3, 0xb1]),      // Test with a non-ASCII character
            ('😊', vec![CHAR, 0xf0, 0x9f, 0x98, 0x8a]), // Test with an emoji (multi-byte character)
        ];

        for (input, expected) in test_cases {
            let encoded = input.encode();
            assert_eq!(encoded.to_bytes(), expected, "Failed encoding for char value: '{}'", input);
        }
    }

    #[tokio::test]
    async fn test_successful_deserialization() {
        let value = ('A' as u32).to_be_bytes().to_vec();

        match char::try_decode(CHAR, value.into_pinned_stream()).await {
            Ok(decoded_char) => assert_eq!('A', decoded_char),
            Err(e) => panic!("Unexpected error: {:?}", e),
        }
    }

    #[tokio::test]
    async fn test_illegal_constructor_deserialization() {
        let illegal_constructor = 0xFF;
        let bytes = vec![];

        match char::try_decode(illegal_constructor, bytes.into_pinned_stream()).await {
            Ok(_) => panic!("Expected an error, but deserialization succeeded"),
            Err(AppError::DeserializationIllegalConstructorError(c)) => assert_eq!(illegal_constructor, c),
            Err(e) => panic!("Unexpected error type: {:?}", e),
        }
    }

    #[tokio::test]
    async fn test_empty_iterator_deserialization() {
        let bytes = vec![]; // Empty vector

        match char::try_decode(CHAR, bytes.into_pinned_stream()).await {
            Ok(_) => panic!("Expected an error, but deserialization succeeded"),
            Err(AppError::IteratorEmptyOrTooShortError) => (), // Expected outcome
            Err(e) => panic!("Unexpected error type: {:?}", e),
        }
    }

    #[tokio::test]
    async fn test_invalid_char_deserialization() {
        let bytes = vec![CHAR, 0xFF, 0xFF, 0xFF, 0xFF]; // Invalid Unicode sequence

        match char::try_decode(CHAR, bytes.into_pinned_stream()).await {
            Ok(_) => panic!("Expected an error, but deserialization succeeded"),
            Err(AppError::InvalidChar) => (), // Expected outcome
            Err(e) => panic!("Unexpected error type: {:?}", e),
        }
    }
}
