use crate::common::read_bytes_8;
use crate::error::AppError;
use crate::serde::decode::Decode;
use crate::serde::encode::{Encode, Encoded};

const DEFAULT_CONSTR: u8 = 0x81;
const SMALL_LONG_CONSTR: u8 = 0x55;

impl Encode for i64 {
    fn encode(&self) -> Encoded {
        match self {
            x if x >= &-128 && x <= &127 => Encoded::new_fixed(0x55, x.to_be_bytes().to_vec()),
            _ => Encoded::new_fixed(0x81, self.to_be_bytes().to_vec()),
        }
    }
}

impl Decode for i64 {
    fn can_decode(iter: impl Iterator<Item=u8>) -> bool {
        match iter.peekable().peek() {
            Some(&DEFAULT_CONSTR) => true,
            Some(&SMALL_LONG_CONSTR) => true,
            _ => false,
        }
    }

    fn try_decode(mut iter: impl Iterator<Item=u8>) -> Result<Self, crate::error::AppError>
        where
            Self: Sized,
    {
        match iter.next() {
            Some(DEFAULT_CONSTR) => Ok(parse_i64(iter)?),
            Some(SMALL_LONG_CONSTR) => Ok(parse_small_i64(iter)?),
            Some(c) => Err(AppError::DeserializationIllegalConstructorError(c)),
            None => Err(AppError::IteratorEmptyOrTooShortError),
        }
    }
}

fn parse_i64(iter: impl Iterator<Item=u8>) -> Result<i64, AppError> {
    let byte_vals = read_bytes_8(iter)?;
    Ok(i64::from_be_bytes(byte_vals))
}

fn parse_small_i64(mut iter: impl Iterator<Item=u8>) -> Result<i64, AppError> {
    if let Some(val) = iter.next() {
        Ok(val as i64)
    } else {
        Err(AppError::IteratorEmptyOrTooShortError)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn construct_long() {
        let val: i64 = 500;
        assert_eq!(val.encode().constructor(), 0x81);
    }

    #[test]
    fn test_encode_i64() {
        let test_cases = [
            (127_i64, vec![0x55, 0, 0, 0, 0, 0, 0, 0, 127]),         // Test with upper boundary of small long
            (-128_i64, vec![0x55, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0x80]), // Test with lower boundary of small long
            (128_i64, vec![0x81, 0, 0, 0, 0, 0, 0, 0, 128]),         // Test just outside upper boundary
            (-129_i64, vec![0x81, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0x7f]), // Test just outside lower boundary
            (i64::MAX, vec![0x81, 0x7f, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff]), // Test with the maximum i64 value
            (i64::MIN, vec![0x81, 0x80, 0, 0, 0, 0, 0, 0, 0]),       // Test with the minimum i64 value
        ];

        for (input, expected) in test_cases {
            let encoded = input.encode();
            assert_eq!(encoded.to_bytes(), expected, "Failed encoding for i64 value: {}", input);
        }
    }

    #[test]
    fn amqp_encodes_longs_between_neg_128_and_127_as_smalllong() {
        let lower: i64 = -128;
        let higher: i64 = 127;
        assert_eq!(lower.encode().constructor(), 0x55);
        assert_eq!(higher.encode().constructor(), 0x55);
    }

    #[test]
    fn can_deocde_returns_true_if_constructor_is_valid() {
        let val_norm = vec![0x81];
        let val_small = vec![0x55];
        assert_eq!(i64::can_decode(val_norm.into_iter()), true);
        assert_eq!(i64::can_decode(val_small.into_iter()), true);
    }

    #[test]
    fn can_decode_return_false_if_constructor_is_invalid() {
        let val = vec![0x71];
        assert_eq!(i64::can_decode(val.into_iter()), false);
    }

    #[test]
    fn try_decode_returns_correct_value() {
        let val = vec![0x81, 0x00, 0x00, 0x00, 0x00, 0x00, 0x10, 0x00, 0x10];
        assert_eq!(i64::try_decode(val.into_iter()).unwrap(), 1048592);
    }

    #[test]
    fn try_decode_returns_error_when_value_bytes_are_invalid() {
        let val = vec![0x66, 0x44];
        assert!(i64::try_decode(val.into_iter()).is_err());
    }

    #[test]
    fn try_decode_returns_error_when_bytes_are_missing() {
        let val = vec![0x81, 0x00, 0x00, 0x01];
        assert!(i64::try_decode(val.into_iter()).is_err());
    }

    #[test]
    fn try_decode_can_decode_smalli64_values() {
        let val = vec![0x55, 0xff];
        assert_eq!(i64::try_decode(val.into_iter()).unwrap(), 255);
    }

    #[test]
    fn try_decode_returns_error_when_parsing_small_i64_and_bytes_are_missing() {
        let val = vec![0x55];
        assert!(i64::try_decode(val.into_iter()).is_err());
    }
}
