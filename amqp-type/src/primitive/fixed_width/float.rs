use crate::constants::FLOAT;
use crate::serde::decode::Decode;
use crate::serde::encode::{Encode, Encoded};
use crate::error::AppError;
use crate::utils::sync_util::read_bytes_4;
use std::hash::Hash;
use std::vec::IntoIter;
use crate::error::amqp_error::AmqpError;

#[derive(Debug, Copy, Clone)]
pub struct Float(f32);

impl Encode for Float {
    fn encode(self) -> Encoded {
        Encoded::new_fixed(FLOAT, self.0.to_be_bytes().to_vec())
    }
}

impl Decode for Float {
    fn try_decode(constructor: u8, stream: &mut IntoIter<u8>) -> Result<Self, AppError>
    where
        Self: Sized,
    {
        match constructor {
            FLOAT => Ok(parse_f32(stream)?),
            _ => Err(AmqpError::DecodeError)?,
        }
    }
}

fn parse_f32(iter: &mut IntoIter<u8>) -> Result<Float, AppError> {
    let byte_vals = read_bytes_4(iter)?;
    Ok(Float(f32::from_be_bytes(byte_vals)))
}

impl From<f32> for Float {
    fn from(value: f32) -> Self {
        Float(value)
    }
}

impl Hash for Float {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.0.to_bits().hash(state);
    }
}

impl PartialEq for Float {
    fn eq(&self, other: &Self) -> bool {
        self.0.to_bits() == other.0.to_bits()
    }
}

impl Eq for Float {}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn construct_float() {
        let val: Float = 32.0.into();
        assert_eq!(val.encode().constructor(), 0x72);
    }

    #[test]
    fn test_encode_float() {
        let test_cases = [
            (Float(0.0), vec![0x72, 0, 0, 0, 0]),      // Test with zero
            (Float(1.0), vec![0x72, 63, 128, 0, 0]),   // Test with a positive value
            (Float(-1.0), vec![0x72, 191, 128, 0, 0]), // Test with a negative value
            (Float(f32::INFINITY), vec![0x72, 127, 128, 0, 0]), // Test with positive infinity
            (Float(f32::NEG_INFINITY), vec![0x72, 255, 128, 0, 0]), // Test with negative infinity
            (Float(f32::NAN), vec![0x72, 127, 192, 0, 0]), // Test with NaN
        ];

        for (input, expected) in test_cases {
            let encoded = input.encode();
            assert_eq!(
                encoded.into_bytes(),
                expected,
                "Failed encoding for Float value: {input:?}"
            );
        }
    }

    #[test]
    fn try_decode_returns_correct_value() {
        let val = vec![0x41, 0x70, 0x00, 0x10];
        assert_eq!(
            Float::try_decode(0x72, &mut val.into_iter()).unwrap(),
            15.000_015.into()
        );
    }

    #[test]
    fn try_decode_returns_error_when_value_bytes_are_invalid() {
        let val = vec![0x44];
        assert!(Float::try_decode(0x66, &mut val.into_iter()).is_err());
    }

    #[test]
    fn try_decode_returns_error_when_bytes_are_missing() {
        let val = vec![0x72, 0x00, 0x01];
        assert!(Float::try_decode(0x72, &mut val.into_iter()).is_err());
    }
}
