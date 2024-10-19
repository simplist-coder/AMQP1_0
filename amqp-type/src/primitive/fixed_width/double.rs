use crate::constants::constructors::DOUBLE;
use crate::serde::decode::Decode;
use crate::serde::encode::{Encode, Encoded};
use amqp_error::AppError;
use amqp_utils::sync_util::read_bytes_8;
use std::hash::Hash;
use std::vec::IntoIter;

/// Crate assumes nothing about the values being passed to it.
/// Any kind of f64 value is handled as is.
/// This means that the hash function considers only the bytes of a float.
/// Two f64 or f64 values are considered Equal if and only if, the entirety of their by sequences match.
/// This ensures that no information is lost.
/// regardless of whether they mean Nan or similar things.
/// As a result, Nan == Nan if and only if the two numbers have the exact same byte sequence.
#[derive(Debug, Copy, Clone)]
pub struct Double(f64);

impl Encode for Double {
    fn encode(self) -> Encoded {
        Encoded::new_fixed(DOUBLE, self.0.to_be_bytes().to_vec())
    }
}

impl Decode for Double {
    fn try_decode(constructor: u8, stream: &mut IntoIter<u8>) -> Result<Self, AppError>
    where
        Self: Sized,
    {
        match constructor {
            DOUBLE => Ok(parse_f64(stream)?),
            c => Err(AppError::DeserializationIllegalConstructorError(c)),
        }
    }
}

fn parse_f64(iter: &mut IntoIter<u8>) -> Result<Double, AppError> {
    let byte_vals = read_bytes_8(iter)?;
    Ok(Double(f64::from_be_bytes(byte_vals)))
}

impl Hash for Double {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.0.to_bits().hash(state);
    }
}

impl Eq for Double {}

impl PartialEq for Double {
    fn eq(&self, other: &Self) -> bool {
        self.0.to_bits() == other.0.to_bits()
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
            (Double(0.0), [0x82, 0, 0, 0, 0, 0, 0, 0, 0]), // Test with zero
            (Double(1.0), [0x82, 63, 240, 0, 0, 0, 0, 0, 0]), // Test with a positive value
            (Double(-1.0), [0x82, 191, 240, 0, 0, 0, 0, 0, 0]), // Test with a negative value
            (Double(f64::INFINITY), [0x82, 127, 240, 0, 0, 0, 0, 0, 0]), // Test with positive infinity
            (
                Double(f64::NEG_INFINITY),
                [0x82, 255, 240, 0, 0, 0, 0, 0, 0],
            ), // Test with negative infinity
            (Double(f64::NAN), [0x82, 127, 248, 0, 0, 0, 0, 0, 0]),      // Test with NaN
        ];

        for (input, expected) in test_cases {
            let encoded = input.encode();
            assert_eq!(
                encoded.into_bytes(),
                expected,
                "Failed encoding for Double value: {input:?}"
            );
        }
    }

    #[test]
    fn try_decode_returns_correct_value() {
        let val = vec![0x40, 0x20, 0x00, 0x00, 0x41, 0x70, 0x00, 0x10];
        assert_eq!(
            Double::try_decode(DOUBLE, &mut val.into_iter()).unwrap(),
            8.000_001_950_189_5.into()
        );
    }

    #[test]
    fn try_decode_returns_error_when_value_bytes_are_invalid() {
        let val = vec![0x44];
        assert!(Double::try_decode(0x66, &mut val.into_iter()).is_err());
    }

    #[test]
    fn try_decode_returns_error_when_bytes_are_missing() {
        let val = vec![0x00, 0x01];
        assert!(Double::try_decode(DOUBLE, &mut val.into_iter()).is_err());
    }
}
