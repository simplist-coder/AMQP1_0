use crate::common::read_bytes_4;
use crate::constants::constructors::{SMALL_UNSIGNED_INTEGER, UNSIGNED_INTEGER, UNSIGNED_INTEGER_ZERO};
use crate::error::AppError;
use crate::serde::decode::Decode;
use crate::serde::encode::{Encode, Encoded};



impl Encode for u32 {
    fn encode(&self) -> Encoded {
        match self {
            0 => Encoded::new_empty(UNSIGNED_INTEGER_ZERO),
            x if x > &0 && x <= &255 => {
                Encoded::new_fixed(SMALL_UNSIGNED_INTEGER, x.to_be_bytes().to_vec())
            }
            _ => Encoded::new_fixed(UNSIGNED_INTEGER, self.to_be_bytes().to_vec()),
        }
    }
}

impl Decode for u32 {
    fn can_decode(iter: impl Iterator<Item=u8>) -> bool {
        match iter.peekable().peek() {
            Some(&UNSIGNED_INTEGER) => true,
            Some(&SMALL_UNSIGNED_INTEGER) => true,
            Some(&UNSIGNED_INTEGER_ZERO) => true,
            _ => false,
        }
    }

    fn try_decode(mut iter: impl Iterator<Item=u8>) -> Result<Self, crate::error::AppError>
        where
            Self: Sized,
    {
        match iter.next() {
            Some(UNSIGNED_INTEGER) => Ok(parse_uint(&mut iter)?),
            Some(SMALL_UNSIGNED_INTEGER) => Ok(parse_small_uint(&mut iter)?),
            Some(UNSIGNED_INTEGER_ZERO) => Ok(0u32),
            Some(c) => Err(AppError::DeserializationIllegalConstructorError(c)),
            None => Err(AppError::IteratorEmptyOrTooShortError),
        }
    }
}

fn parse_uint(iter: &mut impl Iterator<Item=u8>) -> Result<u32, AppError> {
    let val_bytes = read_bytes_4(iter)?;
    Ok(u32::from_be_bytes(val_bytes))
}

fn parse_small_uint(iter: &mut impl Iterator<Item=u8>) -> Result<u32, AppError> {
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
    fn test_encode_u32() {
        let test_cases = [
            (0_u32, vec![0x43]),                             // Test with zero
            (1_u32, vec![0x52, 0, 0, 0, 1]),                // Test with a small positive value
            (255_u32, vec![0x52, 0, 0, 0, 255]),            // Test with upper boundary of small uint
            (256_u32, vec![0x70, 0, 0, 1, 0]),              // Test just outside upper boundary
            (u32::MAX, vec![0x70, 0xff, 0xff, 0xff, 0xff]), // Test with the maximum u32 value
        ];

        for (input, expected) in test_cases {
            let encoded = input.encode();
            assert_eq!(encoded.to_bytes(), expected, "Failed encoding for u32 value: {}", input);
        }
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
