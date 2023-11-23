use std::hash::Hash;

use crate::common::read_bytes_4;
use crate::error::AppError;
use crate::serde::decode::Decode;
use crate::serde::encode::{Encode, Encoded};

const DEFAULT_CONSTR: u8 = 0x72;

pub struct Float(f32);

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

impl Encode for Float {
    fn encode(&self) -> Encoded {
        Encoded::new_fixed(DEFAULT_CONSTR, self.0.to_be_bytes().to_vec())
    }
}

impl From<f32> for Float {
    fn from(value: f32) -> Self {
        Float(value)
    }
}

impl Decode for f32 {
    fn can_decode(iter: impl Iterator<Item = u8>) -> bool {
        match iter.peekable().peek() {
            Some(&DEFAULT_CONSTR) => true,
            _ => false,
        }
    }

    fn try_decode(mut iter: impl Iterator<Item = u8>) -> Result<Self, crate::error::AppError>
    where
        Self: Sized,
    {
        match iter.next() {
            Some(DEFAULT_CONSTR) => Ok(parse_f32(iter)?),
            Some(c) => Err(AppError::DeserializationIllegalConstructorError(c)),
            None => Err(AppError::IteratorEmptyOrTooShortError),
        }
    }
}

fn parse_f32(iter: impl Iterator<Item = u8>) -> Result<f32, AppError> {
    let byte_vals = read_bytes_4(iter)?;
    Ok(f32::from_be_bytes(byte_vals))
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn construct_float() {
        let val: Float = 32.0.into();
        assert_eq!(val.encode().constructor(), 0x72);
    }

    #[test]
    fn can_deocde_returns_true_if_constructor_is_valid() {
        let val_norm = vec![0x72];
        assert_eq!(f32::can_decode(val_norm.into_iter()), true);
    }

    #[test]
    fn can_decode_return_false_if_constructor_is_invalid() {
        let val = vec![0x75];
        assert_eq!(f32::can_decode(val.into_iter()), false);
    }

    #[test]
    fn try_decode_returns_correct_value() {
        let val = vec![0x72, 0x41, 0x70, 0x00, 0x10];
        assert_eq!(f32::try_decode(val.into_iter()).unwrap(), 15.000015);
    }

    #[test]
    fn try_decode_returns_error_when_value_bytes_are_invalid() {
        let val = vec![0x66, 0x44];
        assert!(f32::try_decode(val.into_iter()).is_err());
    }

    #[test]
    fn try_decode_returns_error_when_bytes_are_missing() {
        let val = vec![0x72, 0x00, 0x01];
        assert!(f32::try_decode(val.into_iter()).is_err());
    }
}
