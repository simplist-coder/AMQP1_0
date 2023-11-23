use crate::error::AppError;
use crate::serde::decode::Decode;
use crate::serde::encode::{Encode, Encoded};

const DEFAULT_CONSTR: u8 = 0x51;

impl Encode for i8 {
    fn encode(&self) -> Encoded {
        Encoded::new_fixed(DEFAULT_CONSTR, self.to_be_bytes().to_vec())
    }
}

impl Decode for i8 {
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
            Some(DEFAULT_CONSTR) => Ok(parse_i8(iter)?),
            Some(c) => Err(AppError::DeserializationIllegalConstructorError(c)),
            None => Err(AppError::IteratorEmptyOrTooShortError),
        }
    }
}

fn parse_i8(mut iter: impl Iterator<Item = u8>) -> Result<i8, AppError> {
    if let Some(val) = iter.next() {
        Ok(val as i8)
    } else {
        Err(AppError::IteratorEmptyOrTooShortError)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn construct_byte() {
        let val: i8 = 8;
        assert_eq!(val.encode().constructor(), 0x51);
    }

    #[test]
    fn test_encode_i8() {
        let test_cases = [
            (0_i8, vec![DEFAULT_CONSTR, 0]),          // Test with zero
            (1_i8, vec![DEFAULT_CONSTR, 1]),          // Test with a positive value
            (-1_i8, vec![DEFAULT_CONSTR, 0xff]),      // Test with a negative value
            (i8::MAX, vec![DEFAULT_CONSTR, 0x7f]),    // Test with the maximum i8 value
            (i8::MIN, vec![DEFAULT_CONSTR, 0x80]),    // Test with the minimum i8 value
        ];

        for (input, expected) in test_cases {
            let encoded = input.encode();
            assert_eq!(encoded.to_bytes(), expected, "Failed encoding for i8 value: {}", input);
        }
    }

    #[test]
    fn can_decode_returns_true_if_constructor_is_valid() {
        let val = vec![0x51];
        assert_eq!(i8::can_decode(val.into_iter()), true);
    }

    #[test]
    fn can_decode_return_false_if_constructor_is_invalid() {
        let val = vec![0x71];
        assert_eq!(i8::can_decode(val.into_iter()), false);
    }

    #[test]
    fn try_decode_returns_correct_value() {
        let val = vec![0x51, 0x10];
        assert_eq!(i8::try_decode(val.into_iter()).unwrap(), 16);
    }

    #[test]
    fn decode_returns_error_when_value_bytes_are_invalid() {
        let val = vec![0x66, 0x44];
        assert!(i8::try_decode(val.into_iter()).is_err());
    }

    #[test]
    fn decode_returns_error_when_bytes_are_missing() {
        let val = vec![0x51];
        assert!(i8::try_decode(val.into_iter()).is_err());
    }
}
