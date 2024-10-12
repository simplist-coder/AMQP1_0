use std::pin::Pin;
use tokio_stream::{Stream, StreamExt};
use crate::common::read_bytes_8;
use crate::constants::constructors::{SMALL_UNSIGNED_LONG, UNSIGNED_LONG, UNSIGNED_LONG_ZERO};
use crate::error::AppError;
use crate::serde::decode::Decode;
use crate::serde::encode::{Encode, Encoded};

impl Encode for u64 {
    fn encode(&self) -> Encoded {
        match self {
            0 => Encoded::new_empty(UNSIGNED_LONG_ZERO),
            x if x > &&0 && x <= &255 => {
                Encoded::new_fixed(SMALL_UNSIGNED_LONG, x.to_be_bytes().to_vec())
            }
            _ => Encoded::new_fixed(UNSIGNED_LONG, self.to_be_bytes().to_vec()),
        }
    }
}

impl Decode for u64 {

    async fn try_decode(constructor: u8, mut iter: Pin<Box<impl Stream<Item=u8>>>) -> Result<Self, crate::error::AppError>
        where
            Self: Sized,
    {
        match constructor {
            UNSIGNED_LONG => Ok(parse_ulong(&mut iter).await?),
            SMALL_UNSIGNED_LONG => Ok(parse_small_ulong(&mut iter).await?),
            UNSIGNED_LONG_ZERO => Ok(0),
            c => Err(AppError::DeserializationIllegalConstructorError(c)),
        }
    }
}

async fn parse_ulong(iter: &mut Pin<Box<impl Stream<Item=u8>>>) -> Result<u64, AppError> {
    let byte_vals = read_bytes_8(iter).await?;
    Ok(u64::from_be_bytes(byte_vals))
}

async fn parse_small_ulong(iter: &mut Pin<Box<impl Stream<Item=u8>>>) -> Result<u64, AppError> {
    if let Some(val) = iter.next().await {
        Ok(val as u64)
    } else {
        Err(AppError::IteratorEmptyOrTooShortError)
    }
}

#[cfg(test)]
mod test {
    use crate::common::tests::ByteVecExt;
    use super::*;

    #[test]
    fn construct_ulong() {
        let val: u64 = 500;
        assert_eq!(val.encode().constructor(), 0x80);
    }

    #[test]
    fn test_encode_u64() {
        let test_cases = [
            (0_u64, vec![0x44]),                             // Test with zero
            (1_u64, vec![0x53, 0, 0, 0, 0, 0, 0, 0, 1]),    // Test with a small positive value
            (255_u64, vec![0x53, 0, 0, 0, 0, 0, 0, 0, 255]), // Test with upper boundary of small ulong
            (256_u64, vec![0x80, 0, 0, 0, 0, 0, 0, 1, 0]),   // Test just outside upper boundary
            (u64::MAX, vec![0x80, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff]), // Test with the maximum u64 value
        ];

        for (input, expected) in test_cases {
            let encoded = input.encode();
            assert_eq!(encoded.to_bytes(), expected, "Failed encoding for u64 value: {}", input);
        }
    }

    #[test]
    fn amqp_type_encodes_ulong_smaller_than_256_as_smallulong() {
        let val: u64 = 255;
        assert_eq!(val.encode().constructor(), 0x53);
    }

    #[test]
    fn amqp_type_encodes_ulong_value_0_as_zero_length() {
        let val: u64 = 0;
        assert_eq!(val.encode().constructor(), 0x44);
    }

    #[tokio::test]
    async fn try_decode_returns_correct_value() {
        let val = vec![0x01, 0x01, 0x11, 0x10, 0x10, 0x00, 0x00, 0x10];
        assert_eq!(u64::try_decode(0x80, val.into_pinned_stream()).await.unwrap(), 72357829700222992);
    }

    #[tokio::test]
    async fn decode_returns_error_when_value_bytes_are_invalid() {
        let val = vec![0x44];
        assert!(u64::try_decode(0x66, val.into_pinned_stream()).await.is_err());
    }

    #[tokio::test]
    async fn decode_returns_error_when_bytes_are_missing() {
        let val = vec![0x00, 0x00, 0x01];
        assert!(u64::try_decode(0x70, val.into_pinned_stream()).await.is_err());
    }

    #[tokio::test]
    async fn try_decode_can_decode_zero_length_value_zero() {
        let val = vec![];
        assert_eq!(u64::try_decode(0x44, val.into_pinned_stream()).await.unwrap(), 0);
    }

    #[tokio::test]
    async fn try_decode_can_decode_smallulong_values() {
        let val = vec![0xff];
        assert_eq!(u64::try_decode(0x53, val.into_pinned_stream()).await.unwrap(), 255);
    }

    #[tokio::test]
    async fn try_decode_returns_error_when_parsing_small_ulong_and_bytes_are_missing() {
        let val = vec![];
        assert!(u64::try_decode(0x53, val.into_pinned_stream()).await.is_err());
    }
}
