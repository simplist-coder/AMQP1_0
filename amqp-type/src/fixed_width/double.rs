use std::hash::Hash;

use crate::common::read_bytes_8;
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

const DEFAULT_CONSTR: u8 = 0x82;

impl Encode for Double {
    fn encode(&self) -> Encoded {
        Encoded::new_fixed(0x82, self.0.to_be_bytes().to_vec())
    }
}

impl Decode for f64 {
    fn can_decode(iter: impl Iterator<Item=u8>) -> bool {
        match iter.peekable().peek() {
            Some(&DEFAULT_CONSTR) => true,
            _ => false,
        }
    }

    fn try_decode(mut iter: impl Iterator<Item=u8>) -> Result<Self, AppError>
        where
            Self: Sized,
    {
        match iter.next() {
            Some(DEFAULT_CONSTR) => Ok(parse_f64(iter)?),
            Some(c) => Err(AppError::DeserializationIllegalConstructorError(c)),
            None => Err(AppError::IteratorEmptyOrTooShortError),
        }
    }
}

fn parse_f64(iter: impl Iterator<Item=u8>) -> Result<f64, AppError> {
    let byte_vals = read_bytes_8(iter)?;
    Ok(f64::from_be_bytes(byte_vals))
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

    #[test]
    fn can_deocde_returns_true_if_constructor_is_valid() {
        let val_norm = vec![0x82];
        assert_eq!(f64::can_decode(val_norm.into_iter()), true);
    }

    #[test]
    fn can_decode_return_false_if_constructor_is_invalid() {
        let val = vec![0x75];
        assert_eq!(f64::can_decode(val.into_iter()), false);
    }

    #[test]
    fn try_decode_returns_correct_value() {
        let val = vec![0x82, 0x40, 0x20, 0x00, 0x00, 0x41, 0x70, 0x00, 0x10];
        assert_eq!(f64::try_decode(val.into_iter()).unwrap(), 8.0000019501895);
    }

    #[test]
    fn try_decode_returns_error_when_value_bytes_are_invalid() {
        let val = vec![0x66, 0x44];
        assert!(f64::try_decode(val.into_iter()).is_err());
    }

    #[test]
    fn try_decode_returns_error_when_bytes_are_missing() {
        let val = vec![0x82, 0x00, 0x01];
        assert!(f64::try_decode(val.into_iter()).is_err());
    }
}
