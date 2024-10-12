use std::hash::Hash;
use std::pin::Pin;
use tokio_stream::{Stream, StreamExt};
use crate::common::read_bytes_4;
use crate::constants::constructors::FLOAT;
use crate::error::AppError;
use crate::serde::decode::Decode;
use crate::serde::encode::{Encode, Encoded};



#[derive(Debug)]
pub struct Float(f32);

impl Encode for Float {
    fn encode(&self) -> Encoded {
        Encoded::new_fixed(FLOAT, self.0.to_be_bytes().to_vec())
    }
}

impl Decode for f32 {
    async fn can_decode(iter: Pin<Box<impl Stream<Item=u8>>>) -> bool {
        match iter.peekable().peek().await {
            Some(&FLOAT) => true,
            _ => false,
        }
    }

    async fn try_decode(mut iter: Pin<Box<impl Stream<Item=u8>>>) -> Result<Self, crate::error::AppError>
        where
            Self: Sized,
    {
        match iter.next().await {
            Some(FLOAT) => Ok(parse_f32(&mut iter).await?),
            Some(c) => Err(AppError::DeserializationIllegalConstructorError(c)),
            None => Err(AppError::IteratorEmptyOrTooShortError),
        }
    }
}

async fn parse_f32(iter: &mut Pin<Box<impl Stream<Item=u8>>>) -> Result<f32, AppError> {
    let byte_vals = read_bytes_4(iter).await?;
    Ok(f32::from_be_bytes(byte_vals))
}

impl From<f32> for Float {
    fn from(value: f32) -> Self {
        Float(value)
    }
}

impl Hash for Float {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.0.to_bits().hash(state)
    }
}

impl PartialEq for Float {
    fn eq(&self, other: &Self) -> bool {
        self.0.to_bits() == other.0.to_bits()
    }

    fn ne(&self, other: &Self) -> bool {
        !self.eq(other)
    }
}

impl Eq for Float {}

#[cfg(test)]
mod test {
    use crate::common::tests::ByteVecExt;
    use super::*;

    #[test]
    fn construct_float() {
        let val: Float = 32.0.into();
        assert_eq!(val.encode().constructor(), 0x72);
    }

    #[test]
    fn test_encode_float() {
        let test_cases = [
            (Float(0.0), vec![0x72, 0, 0, 0, 0]),                   // Test with zero
            (Float(1.0), vec![0x72, 63, 128, 0, 0]),                // Test with a positive value
            (Float(-1.0), vec![0x72, 191, 128, 0, 0]),              // Test with a negative value
            (Float(f32::INFINITY), vec![0x72, 127, 128, 0, 0]),     // Test with positive infinity
            (Float(f32::NEG_INFINITY), vec![0x72, 255, 128, 0, 0]), // Test with negative infinity
            (Float(f32::NAN), vec![0x72, 127, 192, 0, 0]),          // Test with NaN
        ];

        for (input, expected) in test_cases {
            let encoded = input.encode();
            assert_eq!(encoded.to_bytes(), expected, "Failed encoding for Float value: {:?}", input);
        }
    }

    #[tokio::test]
    async fn can_deocde_returns_true_if_constructor_is_valid() {
        let val = vec![0x72];
        assert_eq!(f32::can_decode(val.into_pinned_stream()).await, true);
    }

    #[tokio::test]
    async fn can_decode_return_false_if_constructor_is_invalid() {
        let val = vec![0x75];
        assert_eq!(f32::can_decode(val.into_pinned_stream()).await, false);
    }

    #[tokio::test]
    async fn try_decode_returns_correct_value() {
        let val = vec![0x72, 0x41, 0x70, 0x00, 0x10];
        assert_eq!(f32::try_decode(val.into_pinned_stream()).await.unwrap(), 15.000015);
    }

    #[tokio::test]
    async fn try_decode_returns_error_when_value_bytes_are_invalid() {
        let val = vec![0x66, 0x44];
        assert!(f32::try_decode(val.into_pinned_stream()).await.is_err());
    }

    #[tokio::test]
    async fn try_decode_returns_error_when_bytes_are_missing() {
        let val = vec![0x72, 0x00, 0x01];
        assert!(f32::try_decode(val.into_pinned_stream()).await.is_err());
    }
}
