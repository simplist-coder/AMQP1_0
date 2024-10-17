use crate::common::read_bytes_4;
use crate::constants::constructors::{INTEGER, SMALL_INTEGER};
use crate::serde::decode::Decode;
use crate::{
    error::AppError,
    serde::encode::{Encode, Encoded},
};
use std::pin::Pin;
use tokio_stream::{Stream, StreamExt};

impl Encode for i32 {
    fn encode(&self) -> Encoded {
        match self {
            x if x >= &-128 && x <= &127 => {
                Encoded::new_fixed(SMALL_INTEGER, (x.clone() as i8).to_be_bytes().to_vec())
            }
            _ => Encoded::new_fixed(INTEGER, self.to_be_bytes().to_vec()),
        }
    }
}

impl Decode for i32 {
    async fn try_decode(
        constructor: u8,
        stream: &mut Pin<Box<impl Stream<Item = u8>>>,
    ) -> Result<Self, AppError>
    where
        Self: Sized,
    {
        match constructor {
            INTEGER => Ok(parse_i32(stream).await?),
            SMALL_INTEGER => Ok(parse_small_i32(stream).await?),
            c => Err(AppError::DeserializationIllegalConstructorError(c)),
        }
    }
}

async fn parse_i32(iter: &mut Pin<Box<impl Stream<Item = u8>>>) -> Result<i32, AppError> {
    let val_bytes = read_bytes_4(iter).await?;
    Ok(i32::from_be_bytes(val_bytes))
}

async fn parse_small_i32(iter: &mut Pin<Box<impl Stream<Item = u8>>>) -> Result<i32, AppError> {
    if let Some(val) = iter.next().await {
        Ok(i8::from_be_bytes([val]) as i32)
    } else {
        Err(AppError::IteratorEmptyOrTooShortError)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::amqp_type::Primitive;
    use crate::common::tests::ByteVecExt;

    #[test]
    fn test_encode_i32() {
        let test_cases = [
            (127_i32, vec![0x54, 127]),   // Test with upper boundary of small int
            (-128_i32, vec![0x54, 0x80]), // Test with lower boundary of small int
            (128_i32, vec![0x71, 0, 0, 0, 128]), // Test just outside upper boundary
            (-129_i32, vec![0x71, 0xff, 0xff, 0xff, 0x7f]), // Test just outside lower boundary
            (i32::MAX, vec![0x71, 0x7f, 0xff, 0xff, 0xff]), // Test with the maximum i32 value
            (i32::MIN, vec![0x71, 0x80, 0, 0, 0]), // Test with the minimum i32 value
        ];

        for (input, expected) in test_cases {
            let encoded = input.encode();
            assert_eq!(
                encoded.to_bytes(),
                expected,
                "Failed encoding for i32 value: {}",
                input
            );
        }
    }

    #[tokio::test]
    async fn try_decode_returns_correct_value() {
        let val = vec![0x00, 0x00, 0x00, 0x10];
        assert_eq!(
            i32::try_decode(0x71, &mut val.into_pinned_stream())
                .await
                .unwrap(),
            16
        )
    }

    #[tokio::test]
    async fn decode_returns_error_when_value_bytes_are_invalid() {
        let val = vec![0x44];
        assert!(i32::try_decode(0x56, &mut val.into_pinned_stream())
            .await
            .is_err());
    }

    #[tokio::test]
    async fn decode_returns_error_when_bytes_are_missing() {
        let val = vec![0x01];
        assert!(i32::try_decode(0x71, &mut val.into_pinned_stream())
            .await
            .is_err());
    }

    #[tokio::test]
    async fn try_decode_can_decode_smallulong_values() {
        let val = vec![100];
        assert_eq!(
            i32::try_decode(SMALL_INTEGER, &mut val.into_pinned_stream())
                .await
                .unwrap(),
            100
        );
    }

    #[tokio::test]
    async fn try_decode_returns_error_when_parsing_small_ulong_and_bytes_are_missing() {
        let val = vec![];
        assert!(i32::try_decode(0x54, &mut val.into_pinned_stream())
            .await
            .is_err());
    }

    #[tokio::test]
    async fn construct_int() {
        let val = 500;
        assert_eq!(val.encode().constructor(), 0x71);
    }

    #[tokio::test]
    async fn amqp_encodes_ints_between_neg_128_and_127_as_smallint() {
        let lower = -128;
        let higher = 127;
        assert_eq!(lower.encode().constructor(), 0x54);
        assert_eq!(higher.encode().constructor(), 0x54);
    }

    #[tokio::test]
    async fn test_encode_decode_negative_small_int() {
        let original = Primitive::Int(-100);
        let encoded = original.encode().to_bytes();
        let stream = &mut encoded.into_pinned_stream();
        stream.next().await;
        let decoded = i32::try_decode(SMALL_INTEGER, stream).await.unwrap();
        assert_eq!(decoded, -100);
    }

    #[tokio::test]
    async fn test_encode_decode_negative_int() {
        let original = Primitive::Int(-1000);
        let encoded = original.encode().to_bytes();
        let stream = &mut encoded.into_pinned_stream();
        stream.next().await;
        let decoded = i32::try_decode(INTEGER, stream).await.unwrap();
        assert_eq!(decoded, -1000);
    }
}
