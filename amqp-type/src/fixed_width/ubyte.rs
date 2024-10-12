use std::pin::Pin;
use tokio_stream::{Stream, StreamExt};
use crate::constants::constructors::UNSIGNED_BYTE;
use crate::error::AppError;
use crate::serde::decode::Decode;
use crate::serde::encode::{Encode, Encoded};



impl Encode for u8 {
    fn encode(&self) -> Encoded {
        Encoded::new_fixed(UNSIGNED_BYTE, self.to_be_bytes().to_vec())
    }
}

impl Decode for u8 {
    async fn can_decode(data: Pin<Box<impl Stream<Item=u8>>>) -> bool {
        match data.peekable().peek().await {
            Some(&UNSIGNED_BYTE) => true,
            _ => false,
        }
    }

    async fn try_decode(mut iter: Pin<Box<impl Stream<Item=u8>>>) -> Result<Self, crate::error::AppError>
        where
            Self: Sized,
    {
        let con = iter.next().await;
        let val = iter.next().await;
        match (con, val) {
            (Some(UNSIGNED_BYTE), Some(x)) => Ok(x),
            (Some(c), _) => Err(AppError::DeserializationIllegalConstructorError(c)),
            (_, _) => Err(AppError::IteratorEmptyOrTooShortError),
        }
    }
}

#[cfg(test)]
mod test {
    use crate::common::tests::ByteVecExt;
    use super::*;

    #[test]
    fn construct_ubyte() {
        let val: u8 = 8;
        assert_eq!(val.encode().constructor(), 0x50);
    }

    #[test]
    fn test_encode_u8() {
        let test_cases = [
            (0_u8, vec![0x50, 0]),                     // Test with zero
            (1_u8, vec![0x50, 1]),                     // Test with a small positive value
            (u8::MAX, vec![0x50, 0xff]),               // Test with the maximum u8 value
            (100_u8, vec![0x50, 100]),                 // Test with a typical number
        ];

        for (input, expected) in test_cases {
            let encoded = input.encode();
            assert_eq!(encoded.to_bytes(), expected, "Failed encoding for u8 value: {}", input);
        }
    }

    #[tokio::test]
    async fn can_deocde_returns_true_if_constructor_is_valid() {
        let val = vec![0x50, 0x41];
        assert_eq!(u8::can_decode(val.into_pinned_stream()).await, true);
    }

    #[tokio::test]
    async fn can_decode_return_false_if_constructor_is_invalid() {
        let val = vec![0x51];
        assert_eq!(u8::can_decode(val.into_pinned_stream()).await, false);
    }

    #[tokio::test]
    async fn try_decode_returns_correct_value() {
        let val = vec![0x50, 0x10];
        assert_eq!(u8::try_decode(val.into_pinned_stream()).await.unwrap(), 16)
    }
}
