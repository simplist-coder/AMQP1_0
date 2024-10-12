use crate::constants::constructors::BOOLEAN;
#[cfg(feature = "zero-length-encoding")]
use crate::constants::constructors::BOOLEAN_FALSE;
#[cfg(feature = "zero-length-encoding")]
use crate::constants::constructors::BOOLEAN_TRUE;
use crate::error::AppError;
use crate::serde::decode::Decode;
use crate::serde::encode::{Encode, Encoded};
use std::pin::Pin;
use tokio_stream::{Stream, StreamExt};

#[cfg(not(feature = "zero-length-encoding"))]
impl Encode for bool {
    fn encode(&self) -> Encoded {
        match self {
            true => Encoded::new_fixed(BOOLEAN, vec![0x01]),
            false => Encoded::new_fixed(BOOLEAN, vec![0x00]),
        }
    }
}

#[cfg(feature = "zero-length-encoding")]
impl Encode for bool {
    fn encode(&self) -> Encoded {
        match self {
            true => DEFAULT_TRUE.into(),
            false => DEFAULT_FALSE.into(),
        }
    }
}

#[cfg(not(feature = "zero-length-encoding"))]
impl Decode for bool {
    async fn can_decode(data: Pin<Box<impl Stream<Item=u8>>>) -> bool
    where
        Self: Sized,
    {
        match data.peekable().peek().await {
            Some(&BOOLEAN) => true,
            _ => false,
        }
    }

    async fn try_decode(iter: Pin<Box<impl Stream<Item=u8>>>) -> Result<Self, AppError>
    where
        Self: Sized,
    {
        let mut iter = Box::pin(iter);
        let con = iter.next().await;
        let val = iter.next().await;
        match (con, val) {
            (Some(c), Some(v)) if c == BOOLEAN && v == 0x00 => Ok(false),
            (Some(c), Some(v)) if c == BOOLEAN && v == 0x01 => Ok(true),
            (Some(c), _) => Err(AppError::DeserializationIllegalConstructorError(c)),
            (None, _) => Err(AppError::IteratorEmptyOrTooShortError),
        }
    }
}

#[cfg(feature = "zero-length-encoding")]
impl Decode for bool {
    fn can_decode(data: Iterator<Item=u8>) -> bool {
        let mut iter = data.into_iter().peekable();
        match iter.peek() {
            Some(BOOLEAN_TRUE) => true,
            Some(BOOLEAN_FALSE) => true,
            _ => false,
        }
    }

    fn try_decode(data: Iterator<Item=u8>) -> Result<Self, AppError>
    where
        Self: Sized,
    {
        if let Some(val) = iter.next() {
            return match val {
                BOOLEAN_TRUE => Ok(true),
                BOOLEAN_FALSE => Ok(false),
                _ => Err(AppError::DeserializationIllegalConstructorError(val)),
            };
        }
        Err(AppError::IteratorEmptyOrTooShortError)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::common::tests::ByteVecExt;

    #[test]
    #[cfg(not(feature = "zero-length-encoding"))]
    fn construct_bool() {
        assert_eq!(true.encode().constructor(), 0x56);
    }

    #[test]
    #[cfg(not(feature = "zero-length-encoding"))]
    fn bool_gets_encoded_correctly() {
        assert_eq!(true.encode().to_bytes(), vec![0x56, 0x01]);
        assert_eq!(false.encode().to_bytes(), vec![0x56, 0x00]);
    }

    #[tokio::test]
    #[cfg(not(feature = "zero-length-encoding"))]
    async fn can_decode_returns_true_if_constructor_is_valid() {
        let val_true = vec![0x56, 0x01];
        let val_false = vec![0x56, 0x00];
        assert_eq!(bool::can_decode(val_true.into_pinned_stream()).await, true);
        assert_eq!(bool::can_decode(val_false.into_pinned_stream()).await, true);
    }

    #[tokio::test]
    #[cfg(not(feature = "zero-length-encoding"))]
    async fn can_decode_returns_false_if_constructor_invalid() {
        let val_true = vec![0x88, 0x01];
        let val_false = vec![0x97, 0x00];
        assert_eq!(bool::can_decode(val_true.into_pinned_stream()).await, false);
        assert_eq!(bool::can_decode(val_false.into_pinned_stream()).await, false);
    }

    #[tokio::test]
    #[cfg(not(feature = "zero-length-encoding"))]
    async fn decode_returns_error_when_value_bytes_are_invalid() {
        let val_true = vec![0x56, 0x34];
        let val_false = vec![0x56, 0x44];
        assert!(bool::try_decode(val_true.into_pinned_stream()).await.is_err());
        assert!(bool::try_decode(val_false.into_pinned_stream()).await.is_err());
    }

    #[tokio::test]
    #[cfg(not(feature = "zero-length-encoding"))]
    async fn try_decode_returns_correct_value_if_bytes_are_valid() {
        let val_true = vec![0x56, 0x01];
        let val_false = vec![0x56, 0x00];
        assert_eq!(bool::try_decode(val_true.into_pinned_stream()).await.unwrap(), true);
        assert_eq!(bool::try_decode(val_false.into_pinned_stream()).await.unwrap(), false);
    }

    #[test]
    #[cfg(feature = "zero-length-encoding")]
    fn amqp_type_constructs_bool_false_as_zero_length() {
        assert_eq!(false.encode().constructor(), 0x42);
    }

    #[test]
    #[cfg(feature = "zero-length-encoding")]
    fn amqp_type_constructs_bool_true_as_zero_length() {
        assert_eq!(true.encode().constructor(), 0x41)
    }
}
