use crate::error::AppError;
use crate::serde::decode::Decode;
use crate::serde::encode::{Encode, Encoded};
use crate::verify::verify_bytes_read_eq;


const DEFAULT_CONSTR: u8 = 0x70;
const SMALL_UINT_CONSTR: u8 = 0x52;
const UINT_0_CONSTR: u8 = 0x43;

impl Encode for u32 {
    fn encode(&self) -> Encoded {
        match self {
            0 => Encoded::new_empty(UINT_0_CONSTR),
            x if x > &0 && x <= &255 => Encoded::new_fixed(SMALL_UINT_CONSTR, x.to_be_bytes().to_vec()),
            _ => Encoded::new_fixed(DEFAULT_CONSTR, self.to_be_bytes().to_vec()),
        }
    }
}

impl Decode for u32 {
    fn can_decode(iter: impl Iterator<Item = u8>) -> bool {
        match iter.peekable().peek() {
            Some(&DEFAULT_CONSTR) => true,
            Some(&SMALL_UINT_CONSTR) => true,
            Some(&UINT_0_CONSTR) => true,
            _ => false,
        }
    }

    fn try_decode(mut iter: impl Iterator<Item = u8>) -> Result<Self, crate::error::AppError>
    where
        Self: Sized,
    {
        match iter.next() {
            Some(DEFAULT_CONSTR) => Ok(parse_uint(iter)?),
            Some(SMALL_UINT_CONSTR) => Ok(parse_small_uint(iter)?),
            Some(UINT_0_CONSTR) => Ok(0u32),
            Some(c) => Err(AppError::DeserializationIllegalConstructorError(c)),
            None => Err(AppError::IteratorEmptyOrTooShortError),
        }
    }
}

fn parse_uint(iter: impl Iterator<Item = u8>) -> Result<u32, AppError> {
    let mut byte_vals = [0; 4];
    let mut index = 0;
    for b in iter.take(4) {
        byte_vals[index] = b;
        index += 1;
    }
    verify_bytes_read_eq(index, 4)?;
    Ok(u32::from_be_bytes(byte_vals))
}

fn parse_small_uint(mut iter: impl Iterator<Item = u8>) -> Result<u32, AppError> {
    if let Some(val) = iter.next() {
        Ok(val as u32)
    } else {
        Err(AppError::IteratorEmptyOrTooShortError)
    }
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn construct_uint() {
        let val: u32 = 500;
        assert_eq!(val.encode().constructor(), 0x70);
    }

    #[test]
    fn amqp_type_encodes_uint_value_0_as_zero_length() {
        let val: u32 = 0;
        assert_eq!(val.encode().constructor(), 0x43);
    }

    #[test]
    fn amqp_type_encodes_uint_values_smaller_than_256_as_smalluint() {
        let val: u32 = 255;
        assert_eq!(val.encode().constructor(), 0x52);
    }

    #[test]
    fn can_deocde_returns_true_if_constructor_is_valid() {
        let val_norm = vec![0x70];
        let val_small = vec![0x52];
        let val_zero = vec![0x43];
        assert_eq!(u32::can_decode(val_norm.into_iter()), true);
        assert_eq!(u32::can_decode(val_small.into_iter()), true);
        assert_eq!(u32::can_decode(val_zero.into_iter()), true);
    }

    #[test]
    fn can_decode_return_false_if_constructor_is_invalid() {
        let val = vec![0x71];
        assert_eq!(u32::can_decode(val.into_iter()), false);
    }

    #[test]
    fn try_decode_returns_correct_value() {
        let val = vec![0x70, 0x00, 0x00, 0x00, 0x10];
        assert_eq!(u32::try_decode(val.into_iter()).unwrap(), 16);
    }

    #[test]
    fn decode_returns_error_when_value_bytes_are_invalid() {
        let val = vec![0x66, 0x44];
        assert!(u32::try_decode(val.into_iter()).is_err());
    }

    #[test]
    fn decode_returns_error_when_bytes_are_missing() {
        let val = vec![0x70, 0x00, 0x00, 0x01];
        assert!(u32::try_decode(val.into_iter()).is_err());
    }

    #[test]
    fn try_decode_can_decode_zero_length_value_zero() {
        let val = vec![0x43];
        assert_eq!(u32::try_decode(val.into_iter()).unwrap(), 0);
    }

    #[test]
    fn try_decode_can_decode_smalluint_values() {
        let val = vec![0x52, 0xff];
        assert_eq!(u32::try_decode(val.into_iter()).unwrap(), 255);
    }

    #[test]
    fn try_decode_returns_error_when_parsing_small_unint_and_bytes_are_missing() {
        let val = vec![0x52];
        assert!(u32::try_decode(val.into_iter()).is_err());
    }
}
