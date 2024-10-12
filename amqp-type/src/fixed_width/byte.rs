use std::pin::Pin;
use tokio_stream::{Stream, StreamExt};
use crate::constants::constructors::BYTE;
use crate::error::AppError;
use crate::serde::decode::Decode;
use crate::serde::encode::{Encode, Encoded};


impl Encode for i8 {
    fn encode(&self) -> Encoded {
        Encoded::new_fixed(BYTE, self.to_be_bytes().to_vec())
    }
}

impl Decode for i8 {
    async fn can_decode(iter: Pin<Box<impl Stream<Item=u8>>>) -> bool {
        match iter.peekable().peek().await {
            Some(&BYTE) => true,
            _ => false,
        }
    }

    async fn try_decode(mut iter: Pin<Box<impl Stream<Item=u8>>>) -> Result<Self, crate::error::AppError>
        where
            Self: Sized,
    {
        match iter.next().await {
            Some(BYTE) => Ok(parse_i8(iter).await?),
            Some(c) => Err(AppError::DeserializationIllegalConstructorError(c)),
            None => Err(AppError::IteratorEmptyOrTooShortError),
        }
    }
}

async fn parse_i8(mut iter: Pin<Box<impl Stream<Item=u8>>>) -> Result<i8, AppError> {
    if let Some(val) = iter.next().await {
        Ok(val as i8)
    } else {
        Err(AppError::IteratorEmptyOrTooShortError)
    }
}

#[cfg(test)]
mod test {
    use crate::common::tests::ByteVecExt;
    use crate::constants::constructors::BYTE;
    use super::*;

    #[test]
    fn construct_byte() {
        let val: i8 = 8;
        assert_eq!(val.encode().constructor(), 0x51);
    }

    #[test]
    fn test_encode_i8() {
        let test_cases = [
            (0_i8, vec![BYTE, 0]),          // Test with zero
            (1_i8, vec![BYTE, 1]),          // Test with a positive value
            (-1_i8, vec![BYTE, 0xff]),      // Test with a negative value
            (i8::MAX, vec![BYTE, 0x7f]),    // Test with the maximum i8 value
            (i8::MIN, vec![BYTE, 0x80]),    // Test with the minimum i8 value
        ];

        for (input, expected) in test_cases {
            let encoded = input.encode();
            assert_eq!(encoded.to_bytes(), expected, "Failed encoding for i8 value: {}", input);
        }
    }

    #[tokio::test]
    async fn can_decode_returns_true_if_constructor_is_valid() {
        let val = vec![0x51];
        assert_eq!(i8::can_decode(val.into_pinned_stream()).await, true);
    }

    #[tokio::test]
    async fn can_decode_return_false_if_constructor_is_invalid() {
        let val = vec![0x71];
        assert_eq!(i8::can_decode(val.into_pinned_stream()).await, false);
    }

    #[tokio::test]
    async fn try_decode_returns_correct_value() {
        let val = vec![0x51, 0x10];
        assert_eq!(i8::try_decode(val.into_pinned_stream()).await.unwrap(), 16);
    }

    #[tokio::test]
    async fn decode_returns_error_when_value_bytes_are_invalid() {
        let val = vec![0x66, 0x44];
        assert!(i8::try_decode(val.into_pinned_stream()).await.is_err());
    }

    #[tokio::test]
    async fn decode_returns_error_when_bytes_are_missing() {
        let val = vec![0x51];
        assert!(i8::try_decode(val.into_pinned_stream()).await.is_err());
    }
}
