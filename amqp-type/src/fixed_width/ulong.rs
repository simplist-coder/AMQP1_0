use crate::common::read_bytes_8;
use crate::error::AppError;
use crate::serde::decode::Decode;
use crate::serde::encode::{Encode, Encoded};

const DEFAULT_CONSTR: u8 = 0x80;
const SMALL_ULONG_CONSTR: u8 = 0x53;
const ULONG_0_CONSTR: u8 = 0x44;

impl Encode for u64 {
    fn encode(&self) -> Encoded {
        match self {
            0 => Encoded::new_empty(ULONG_0_CONSTR),
            x if x > &&0 && x <= &255 => {
                Encoded::new_fixed(SMALL_ULONG_CONSTR, x.to_be_bytes().to_vec())
            }
            _ => Encoded::new_fixed(DEFAULT_CONSTR, self.to_be_bytes().to_vec()),
        }
    }
}

impl Decode for u64 {
    fn can_decode(iter: impl Iterator<Item=u8>) -> bool {
        match iter.peekable().peek() {
            Some(&DEFAULT_CONSTR) => true,
            Some(&SMALL_ULONG_CONSTR) => true,
            Some(&ULONG_0_CONSTR) => true,
            _ => false,
        }
    }

    fn try_decode(mut iter: impl Iterator<Item=u8>) -> Result<Self, crate::error::AppError>
        where
            Self: Sized,
    {
        match iter.next() {
            Some(DEFAULT_CONSTR) => Ok(parse_ulong(iter)?),
            Some(SMALL_ULONG_CONSTR) => Ok(parse_small_ulong(iter)?),
            Some(ULONG_0_CONSTR) => Ok(0),
            Some(c) => Err(AppError::DeserializationIllegalConstructorError(c)),
            None => Err(AppError::IteratorEmptyOrTooShortError),
        }
    }
}

fn parse_ulong(iter: impl Iterator<Item=u8>) -> Result<u64, AppError> {
    let byte_vals = read_bytes_8(iter)?;
    Ok(u64::from_be_bytes(byte_vals))
}

fn parse_small_ulong(mut iter: impl Iterator<Item=u8>) -> Result<u64, AppError> {
    if let Some(val) = iter.next() {
        Ok(val as u64)
    } else {
        Err(AppError::IteratorEmptyOrTooShortError)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn construct_ulong() {
        let val: u64 = 500;
        assert_eq!(val.encode().constructor(), 0x80);
    }

    #[test]
    fn test_encode_u64() {
        let test_cases = [
            (0_u64, vec![0x44]),                             // Test with zero
            (1_u64, vec![0x53, 0, 0, 0, 0, 0, 0, 0, 1]),    // Test with a small positive value
            (255_u64, vec![0x53, 0, 0, 0, 0, 0, 0, 0, 255]), // Test with upper boundary of small ulong
            (256_u64, vec![0x80, 0, 0, 0, 0, 0, 0, 1, 0]),   // Test just outside upper boundary
            (u64::MAX, vec![0x80, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff]), // Test with the maximum u64 value
        ];

        for (input, expected) in test_cases {
            let encoded = input.encode();
            assert_eq!(encoded.to_bytes(), expected, "Failed encoding for u64 value: {}", input);
        }
    }

    #[test]
    fn amqp_type_encodes_ulong_smaller_than_256_as_smallulong() {
        let val: u64 = 255;
        assert_eq!(val.encode().constructor(), 0x53);
    }

    #[test]
    fn amqp_type_encodes_ulong_value_0_as_zero_length() {
        let val: u64 = 0;
        assert_eq!(val.encode().constructor(), 0x44);
    }

    #[test]
    fn can_deocde_returns_true_if_constructor_is_valid() {
        let val_norm = vec![0x80];
        let val_small = vec![0x53];
        let val_zero = vec![0x44];
        assert_eq!(u64::can_decode(val_norm.into_iter()), true);
        assert_eq!(u64::can_decode(val_small.into_iter()), true);
        assert_eq!(u64::can_decode(val_zero.into_iter()), true);
    }

    #[test]
    fn can_decode_return_false_if_constructor_is_invalid() {
        let val = vec![0x71];
        assert_eq!(u64::can_decode(val.into_iter()), false);
    }

    #[test]
    fn try_decode_returns_correct_value() {
        let val = vec![0x80, 0x01, 0x01, 0x11, 0x10, 0x10, 0x00, 0x00, 0x10];
        assert_eq!(u64::try_decode(val.into_iter()).unwrap(), 72357829700222992);
    }

    #[test]
    fn decode_returns_error_when_value_bytes_are_invalid() {
        let val = vec![0x66, 0x44];
        assert!(u64::try_decode(val.into_iter()).is_err());
    }

    #[test]
    fn decode_returns_error_when_bytes_are_missing() {
        let val = vec![0x70, 0x00, 0x00, 0x01];
        assert!(u64::try_decode(val.into_iter()).is_err());
    }

    #[test]
    fn try_decode_can_decode_zero_length_value_zero() {
        let val = vec![0x44];
        assert_eq!(u64::try_decode(val.into_iter()).unwrap(), 0);
    }

    #[test]
    fn try_decode_can_decode_smallulong_values() {
        let val = vec![0x53, 0xff];
        assert_eq!(u64::try_decode(val.into_iter()).unwrap(), 255);
    }

    #[test]
    fn try_decode_returns_error_when_parsing_small_ulong_and_bytes_are_missing() {
        let val = vec![0x53];
        assert!(u64::try_decode(val.into_iter()).is_err());
    }
}
