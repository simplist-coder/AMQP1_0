use std::pin::Pin;
use tokio_stream::{Stream, StreamExt};
use crate::{
    error::AppError,
    serde::encode::{Encode, Encoded},
};
use crate::common::read_bytes_4;
use crate::constants::constructors::{INTEGER, SMALL_INTEGER};
use crate::serde::decode::Decode;



impl Encode for i32 {
    fn encode(&self) -> Encoded {
        match self {
            x if x >= &-128 && x <= &127 => Encoded::new_fixed(SMALL_INTEGER, x.to_be_bytes().to_vec()),
            _ => Encoded::new_fixed(INTEGER, self.to_be_bytes().to_vec()),
        }
    }
}

impl Decode for i32 {
    async fn can_decode(iter: Pin<Box<impl Stream<Item=u8>>>) -> bool {
        match iter.peekable().peek().await {
            Some(&INTEGER) => true,
            Some(&SMALL_INTEGER) => true,
            _ => false,
        }
    }

    async fn try_decode(mut iter: Pin<Box<impl Stream<Item=u8>>>) -> Result<Self, crate::error::AppError>
        where
            Self: Sized,
    {
        match iter.next().await {
            Some(INTEGER) => Ok(parse_i32(&mut iter).await?),
            Some(SMALL_INTEGER) => Ok(parse_small_i32(&mut iter).await?),
            Some(c) => Err(AppError::DeserializationIllegalConstructorError(c)),
            None => Err(AppError::IteratorEmptyOrTooShortError),
        }
    }
}

async fn parse_i32(iter: &mut Pin<Box<impl Stream<Item=u8>>>) -> Result<i32, AppError> {
    let val_bytes = read_bytes_4(iter).await?;
    Ok(i32::from_be_bytes(val_bytes))
}

async fn parse_small_i32(iter: &mut Pin<Box<impl Stream<Item=u8>>>) -> Result<i32, AppError> {
    if let Some(val) = iter.next().await {
        Ok(val as i32)
    } else {
        Err(AppError::IteratorEmptyOrTooShortError)
    }
}

#[cfg(test)]
mod test {
    use crate::common::tests::ByteVecExt;
    use super::*;

    #[test]
    fn test_encode_i32() {
        let test_cases = [
            (127_i32, vec![0x54, 0, 0, 0, 127]),         // Test with upper boundary of small int
            (-128_i32, vec![0x54, 0xff, 0xff, 0xff, 0x80]), // Test with lower boundary of small int
            (128_i32, vec![0x71, 0, 0, 0, 128]),         // Test just outside upper boundary
            (-129_i32, vec![0x71, 0xff, 0xff, 0xff, 0x7f]), // Test just outside lower boundary
            (i32::MAX, vec![0x71, 0x7f, 0xff, 0xff, 0xff]), // Test with the maximum i32 value
            (i32::MIN, vec![0x71, 0x80, 0, 0, 0]),       // Test with the minimum i32 value
        ];

        for (input, expected) in test_cases {
            let encoded = input.encode();
            assert_eq!(encoded.to_bytes(), expected, "Failed encoding for i32 value: {}", input);
        }
    }

    #[tokio::test]
    async fn can_deocde_returns_true_if_constructor_is_valid() {
        let val = vec![0x71, 0x41];
        let small_val = vec![0x54, 0x41];
        assert_eq!(i32::can_decode(val.into_pinned_stream()).await, true);
        assert_eq!(i32::can_decode(small_val.into_pinned_stream()).await, true);
    }

    #[tokio::test]
    async fn can_decode_return_false_if_constructor_is_invalid() {
        let val = vec![0x70];
        assert_eq!(i32::can_decode(val.into_pinned_stream()).await, false);
    }

    #[tokio::test]
    async fn try_decode_returns_correct_value() {
        let val = vec![0x71, 0x00, 0x00, 0x00, 0x10];
        assert_eq!(i32::try_decode(val.into_pinned_stream()).await.unwrap(), 16)
    }

    #[tokio::test]
    async fn decode_returns_error_when_value_bytes_are_invalid() {
        let val = vec![0x56, 0x44];
        assert!(i32::try_decode(val.into_pinned_stream()).await.is_err());
    }

    #[tokio::test]
    async fn decode_returns_error_when_bytes_are_missing() {
        let val = vec![0x71, 0x01];
        assert!(i32::try_decode(val.into_pinned_stream()).await.is_err());
    }

    #[tokio::test]
    async fn try_decode_can_decode_smallulong_values() {
        let val = vec![0x54, 0xff];
        assert_eq!(i32::try_decode(val.into_pinned_stream()).await.unwrap(), 255);
    }

    #[tokio::test]
    async fn try_decode_returns_error_when_parsing_small_ulong_and_bytes_are_missing() {
        let val = vec![0x54];
        assert!(i32::try_decode(val.into_pinned_stream()).await.is_err());
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
}
