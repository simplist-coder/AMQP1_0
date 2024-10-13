use crate::constants::constructors::BYTE;
use crate::error::AppError;
use crate::serde::decode::Decode;
use crate::serde::encode::{Encode, Encoded};
use std::pin::Pin;
use tokio_stream::{Stream, StreamExt};

impl Encode for i8 {
    fn encode(&self) -> Encoded {
        Encoded::new_fixed(BYTE, self.to_be_bytes().to_vec())
    }
}

impl Decode for i8 {
    async fn try_decode(
        constructor: u8,
        stream: &mut Pin<Box<impl Stream<Item = u8>>>,
    ) -> Result<Self, AppError>
    where
        Self: Sized,
    {
        match constructor {
            BYTE => Ok(parse_i8(stream).await?),
            other => Err(AppError::DeserializationIllegalConstructorError(other)),
        }
    }
}

async fn parse_i8(iter: &mut Pin<Box<impl Stream<Item = u8>>>) -> Result<i8, AppError> {
    if let Some(val) = iter.next().await {
        Ok(val as i8)
    } else {
        Err(AppError::IteratorEmptyOrTooShortError)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::common::tests::ByteVecExt;
    use crate::constants::constructors::BYTE;

    #[test]
    fn construct_byte() {
        let val: i8 = 8;
        assert_eq!(val.encode().constructor(), 0x51);
    }

    #[test]
    fn test_encode_i8() {
        let test_cases = [
            (0_i8, vec![BYTE, 0]),       // Test with zero
            (1_i8, vec![BYTE, 1]),       // Test with a positive value
            (-1_i8, vec![BYTE, 0xff]),   // Test with a negative value
            (i8::MAX, vec![BYTE, 0x7f]), // Test with the maximum i8 value
            (i8::MIN, vec![BYTE, 0x80]), // Test with the minimum i8 value
        ];

        for (input, expected) in test_cases {
            let encoded = input.encode();
            assert_eq!(
                encoded.to_bytes(),
                expected,
                "Failed encoding for i8 value: {}",
                input
            );
        }
    }

    #[tokio::test]
    async fn try_decode_returns_correct_value() {
        let val = vec![0x10];
        assert_eq!(
            i8::try_decode(0x51, &mut val.into_pinned_stream())
                .await
                .unwrap(),
            16
        );
    }

    #[tokio::test]
    async fn decode_returns_error_when_value_bytes_are_invalid() {
        let val = vec![0x44];
        assert!(i8::try_decode(0x66, &mut val.into_pinned_stream())
            .await
            .is_err());
    }

    #[tokio::test]
    async fn decode_returns_error_when_bytes_are_missing() {
        let val = vec![];
        assert!(i8::try_decode(0x51, &mut val.into_pinned_stream())
            .await
            .is_err());
    }
}
