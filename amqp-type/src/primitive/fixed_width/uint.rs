use crate::common::read_bytes_4;
use crate::constants::constructors::{
    SMALL_UNSIGNED_INTEGER, UNSIGNED_INTEGER, UNSIGNED_INTEGER_ZERO,
};
use crate::error::AppError;
use crate::serde::decode::Decode;
use crate::serde::encode::{Encode, Encoded};
use std::pin::Pin;
use tokio_stream::{Stream, StreamExt};

impl Encode for u32 {
    fn encode(self) -> Encoded {
        match self {
            0 => Encoded::new_empty(UNSIGNED_INTEGER_ZERO),
            x if x > 0 && x <= 255 => {
                Encoded::new_fixed(SMALL_UNSIGNED_INTEGER, (x as u8).to_be_bytes().to_vec())
            }
            _ => Encoded::new_fixed(UNSIGNED_INTEGER, self.to_be_bytes().to_vec()),
        }
    }
}

impl Decode for u32 {
    async fn try_decode(
        constructor: u8,
        stream: &mut Pin<Box<impl Stream<Item = u8>>>,
    ) -> Result<Self, AppError>
    where
        Self: Sized,
    {
        match constructor {
            UNSIGNED_INTEGER => Ok(parse_uint(stream).await?),
            SMALL_UNSIGNED_INTEGER => Ok(parse_small_uint(stream).await?),
            UNSIGNED_INTEGER_ZERO => Ok(0u32),
            c => Err(AppError::DeserializationIllegalConstructorError(c)),
        }
    }
}

async fn parse_uint(iter: &mut Pin<Box<impl Stream<Item = u8>>>) -> Result<u32, AppError> {
    let val_bytes = read_bytes_4(iter).await?;
    Ok(u32::from_be_bytes(val_bytes))
}

async fn parse_small_uint(iter: &mut Pin<Box<impl Stream<Item = u8>>>) -> Result<u32, AppError> {
    if let Some(val) = iter.next().await {
        Ok(u32::from(val))
    } else {
        Err(AppError::IteratorEmptyOrTooShortError)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::common::tests::ByteVecExt;

    #[test]
    fn construct_uint() {
        let val: u32 = 500;
        assert_eq!(val.encode().constructor(), 0x70);
    }

    #[test]
    fn test_encode_u32() {
        let test_cases = [
            (0_u32, vec![0x43]),                            // Test with zero
            (1_u32, vec![0x52, 1]),                         // Test with a small positive value
            (255_u32, vec![0x52, 255]), // Test with upper boundary of small uint
            (256_u32, vec![0x70, 0, 0, 1, 0]), // Test just outside upper boundary
            (u32::MAX, vec![0x70, 0xff, 0xff, 0xff, 0xff]), // Test with the maximum u32 value
        ];

        for (input, expected) in test_cases {
            let encoded = input.encode();
            assert_eq!(
                encoded.into_bytes(),
                expected,
                "Failed encoding for u32 value: {input}"
            );
        }
    }

    #[test]
    fn amqp_type_encodes_uint_value_0_as_zero_length() {
        let val: u32 = 0;
        assert_eq!(val.encode().constructor(), 0x43);
    }

    #[test]
    fn amqp_type_encodes_uint_values_smaller_than_256_as_smalluint() {
        let val: u32 = 255;
        assert_eq!(val.encode().constructor(), 0x52);
    }

    #[tokio::test]
    async fn try_decode_returns_correct_value() {
        let val = vec![0x00, 0x00, 0x00, 0x10];
        assert_eq!(
            u32::try_decode(0x70, &mut val.into_pinned_stream())
                .await
                .unwrap(),
            16
        );
    }

    #[tokio::test]
    async fn decode_returns_error_when_value_bytes_are_invalid() {
        let val = vec![0x44];
        assert!(u32::try_decode(0x66, &mut val.into_pinned_stream())
            .await
            .is_err());
    }

    #[tokio::test]
    async fn decode_returns_error_when_bytes_are_missing() {
        let val = vec![0x00, 0x00, 0x01];
        assert!(u32::try_decode(0x70, &mut val.into_pinned_stream())
            .await
            .is_err());
    }

    #[tokio::test]
    async fn try_decode_can_decode_zero_length_value_zero() {
        let val = vec![];
        assert_eq!(
            u32::try_decode(0x43, &mut val.into_pinned_stream())
                .await
                .unwrap(),
            0
        );
    }

    #[tokio::test]
    async fn try_decode_can_decode_smalluint_values() {
        let val = vec![0xff];
        assert_eq!(
            u32::try_decode(0x52, &mut val.into_pinned_stream())
                .await
                .unwrap(),
            255
        );
    }

    #[tokio::test]
    async fn try_decode_returns_error_when_parsing_small_unint_and_bytes_are_missing() {
        let val = vec![];
        assert!(u32::try_decode(0x52, &mut val.into_pinned_stream())
            .await
            .is_err());
    }
}
