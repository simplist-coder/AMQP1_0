use crate::constants::constructors::BOOLEAN;
use crate::constants::constructors::BOOLEAN_FALSE;
use crate::constants::constructors::BOOLEAN_TRUE;
use crate::error::AppError;
use crate::serde::decode::Decode;
use crate::serde::encode::{Encode, Encoded};
use std::pin::Pin;
use tokio_stream::{Stream, StreamExt};

#[cfg(not(feature = "zero-length-encoding"))]
impl Encode for bool {
    fn encode(self) -> Encoded {
        match self {
            true => Encoded::new_fixed(BOOLEAN, vec![0x01]),
            false => Encoded::new_fixed(BOOLEAN, vec![0x00]),
        }
    }
}

#[cfg(feature = "zero-length-encoding")]
impl Encode for bool {
    fn encode(self) -> Encoded {
        match self {
            true => BOOLEAN_TRUE.into(),
            false => BOOLEAN_FALSE.into(),
        }
    }
}

impl Decode for bool {
    async fn try_decode(
        constructor: u8,
        iter: &mut Pin<Box<impl Stream<Item = u8>>>,
    ) -> Result<Self, AppError>
    where
        Self: Sized,
    {
        match constructor {
            BOOLEAN_TRUE => Ok(true),
            BOOLEAN_FALSE => Ok(false),
            BOOLEAN => {
                let val = iter.next().await;
                match (constructor, val) {
                    (BOOLEAN, Some(v)) if v == 0x00 => Ok(false),
                    (BOOLEAN, Some(v)) if v == 0x01 => Ok(true),
                    (c, _) => Err(AppError::DeserializationIllegalConstructorError(c)),
                }
            }
            c => Err(AppError::DeserializationIllegalConstructorError(c)),
        }
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

    #[tokio::test]
    async fn decode_returns_error_when_value_bytes_are_invalid() {
        let val_true = vec![0x34];
        let val_false = vec![0x44];
        assert!(bool::try_decode(0x56, &mut val_true.into_pinned_stream())
            .await
            .is_err());
        assert!(bool::try_decode(0x56, &mut val_false.into_pinned_stream())
            .await
            .is_err());
    }

    #[tokio::test]
    async fn try_decode_returns_correct_value_if_bytes_are_valid() {
        let val_true = vec![0x01];
        let val_false = vec![0x00];
        let val_true_zero_length = vec![];
        let val_false_zero_length = vec![];
        assert!(bool::try_decode(0x56, &mut val_true.into_pinned_stream())
            .await
            .unwrap());
        assert!(!bool::try_decode(0x56, &mut val_false.into_pinned_stream())
            .await
            .unwrap());
        assert!(
            bool::try_decode(BOOLEAN_TRUE, &mut val_true_zero_length.into_pinned_stream())
                .await
                .unwrap()
        );
        assert!(!bool::try_decode(
            BOOLEAN_FALSE,
            &mut val_false_zero_length.into_pinned_stream()
        )
        .await
        .unwrap());
    }

    #[tokio::test]
    async fn try_decode_zero_length_encoded_bool_does_not_advance_the_stream() {
        let vals = vec![1, 2, 3];
        let mut stream = vals.into_pinned_stream();
        assert!(bool::try_decode(BOOLEAN_TRUE, &mut stream).await.unwrap());
        assert!(!bool::try_decode(BOOLEAN_FALSE, &mut stream).await.unwrap());
        assert_eq!(stream.next().await, Some(1));
        assert_eq!(stream.next().await, Some(2));
        assert_eq!(stream.next().await, Some(3));
        assert_eq!(stream.next().await, None);
        assert_eq!(stream.next().await, None);
    }
}
