use std::hash::Hash;
use std::pin::Pin;
use tokio_stream::{Stream, StreamExt};
use crate::common::read_bytes_8;
use crate::constants::constructors::DOUBLE;
use crate::error::AppError;
use crate::serde::decode::Decode;
use crate::serde::encode::{Encode, Encoded};

/// Crate assumes nothing about the values being passed to it.
/// Any kind of f64 value is handled as is.
/// This means that the hash function considers only the bytes of a float.
/// Two f64 or f64 values are considered Equal if and only if, the entirety of their by sequences match.
/// This ensures that no information is lost.
/// regardless of whether they mean Nan or similar things.
/// As a result, Nan == Nan if and only if the two numbers have the exact same byte sequence.
#[derive(Debug)]
pub struct Double(f64);


impl Encode for Double {
    fn encode(&self) -> Encoded {
        Encoded::new_fixed(0x82, self.0.to_be_bytes().to_vec())
    }
}

impl Decode for Double {
    async fn can_decode(iter: Pin<Box<impl Stream<Item=u8>>>) -> bool {
        match iter.peekable().peek().await {
            Some(&DOUBLE) => true,
            _ => false,
        }
    }

    async fn try_decode(mut iter: Pin<Box<impl Stream<Item=u8>>>) -> Result<Self, AppError>
        where
            Self: Sized,
    {
        match iter.next().await {
            Some(DOUBLE) => Ok(parse_f64(&mut iter).await?),
            Some(c) => Err(AppError::DeserializationIllegalConstructorError(c)),
            None => Err(AppError::IteratorEmptyOrTooShortError),
        }
    }
}

async fn parse_f64(iter: &mut Pin<Box<impl Stream<Item=u8>>>) -> Result<Double, AppError> {
    let byte_vals = read_bytes_8(iter).await?;
    Ok(Double(f64::from_be_bytes(byte_vals)))
}


impl Hash for Double {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.0.to_bits().hash(state)
    }
}

impl Eq for Double {}

impl PartialEq for Double {
    fn eq(&self, other: &Self) -> bool {
        self.0.to_bits() == other.0.to_bits()
    }

    fn ne(&self, other: &Self) -> bool {
        !self.eq(other)
    }
}


impl From<f64> for Double {
    fn from(value: f64) -> Self {
        Double(value)
    }
}


#[cfg(test)]
mod test {
    use crate::common::tests::ByteVecExt;
    use super::*;

    #[test]
    fn construct_double() {
        let val: Double = 64.0.into();
        assert_eq!(val.encode().constructor(), 0x82);
    }

    #[test]
    fn test_encode_double() {
        let test_cases = [
            (Double(0.0), [0x82, 0, 0, 0, 0, 0, 0, 0, 0]),                 // Test with zero
            (Double(1.0), [0x82, 63, 240, 0, 0, 0, 0, 0, 0]),              // Test with a positive value
            (Double(-1.0), [0x82, 191, 240, 0, 0, 0, 0, 0, 0]),            // Test with a negative value
            (Double(f64::INFINITY), [0x82, 127, 240, 0, 0, 0, 0, 0, 0]),   // Test with positive infinity
            (Double(f64::NEG_INFINITY), [0x82, 255, 240, 0, 0, 0, 0, 0, 0]), // Test with negative infinity
            (Double(f64::NAN), [0x82, 127, 248, 0, 0, 0, 0, 0, 0]),        // Test with NaN
        ];

        for (input, expected) in test_cases {
            let encoded = input.encode();
            assert_eq!(encoded.to_bytes(), expected, "Failed encoding for Double value: {:?}", input);
        }
    }

    #[tokio::test]
    async fn can_deocde_returns_true_if_constructor_is_valid() {
        let val_norm = vec![0x82];
        assert_eq!(Double::can_decode(val_norm.into_pinned_stream()).await, true);
    }

    #[tokio::test]
    async fn can_decode_return_false_if_constructor_is_invalid() {
        let val = vec![0x75];
        assert_eq!(Double::can_decode(val.into_pinned_stream()).await, false);
    }

    #[tokio::test]
    async fn try_decode_returns_correct_value() {
        let val = vec![0x82, 0x40, 0x20, 0x00, 0x00, 0x41, 0x70, 0x00, 0x10];
        assert_eq!(Double::try_decode(val.into_pinned_stream()).await.unwrap(), 8.0000019501895.into());
    }

    #[tokio::test]
    async fn try_decode_returns_error_when_value_bytes_are_invalid() {
        let val = vec![0x66, 0x44];
        assert!(Double::try_decode(val.into_pinned_stream()).await.is_err());
    }

    #[tokio::test]
    async fn try_decode_returns_error_when_bytes_are_missing() {
        let val = vec![0x82, 0x00, 0x01];
        assert!(Double::try_decode(val.into_pinned_stream()).await.is_err());
    }
}
