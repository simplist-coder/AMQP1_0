use crate::common::read_bytes_2;
use crate::error::AppError;
use crate::serde::decode::Decode;
use crate::serde::encode::{Encode, Encoded};

const DEFAULT_CONSTR: u8 = 0x60;

impl Encode for u16 {
    fn encode(&self) -> Encoded {
        Encoded::new_fixed(DEFAULT_CONSTR, self.to_be_bytes().to_vec())
    }
}

impl Decode for u16 {
    fn can_decode(iter: impl Iterator<Item=u8>) -> bool {
        match iter.peekable().peek() {
            Some(&DEFAULT_CONSTR) => true,
            _ => false,
        }
    }

    fn try_decode(mut iter: impl Iterator<Item=u8>) -> Result<Self, crate::error::AppError>
        where
            Self: Sized,
    {
        match iter.next() {
            Some(DEFAULT_CONSTR) => Ok(parse_u16(iter)?),
            Some(c) => Err(AppError::DeserializationIllegalConstructorError(c)),
            None => Err(AppError::IteratorEmptyOrTooShortError),
        }
    }
}

fn parse_u16(iter: impl Iterator<Item=u8>) -> Result<u16, AppError> {
    let val_bytes = read_bytes_2(iter)?;
    Ok(u16::from_be_bytes(val_bytes))
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn construct_ushort() {
        let val: u16 = 16;
        assert_eq!(val.encode().constructor(), 0x60);
    }

    #[test]
    fn test_encode_u16() {
        let test_cases = [
            (0_u16, vec![0x60, 0, 0]),                   // Test with zero
            (1_u16, vec![0x60, 0, 1]),                   // Test with a small positive value
            (u16::MAX, vec![0x60, 0xff, 0xff]),          // Test with the maximum u16 value
            (256_u16, vec![0x60, 1, 0]),                 // Test with a typical number
        ];

        for (input, expected) in test_cases {
            let encoded = input.encode();
            assert_eq!(encoded.to_bytes(), expected, "Failed encoding for u16 value: {}", input);
        }
    }

    #[test]
    fn can_deocde_returns_true_if_constructor_is_valid() {
        let val = vec![0x60, 0x41];
        assert_eq!(u16::can_decode(val.into_iter()), true);
    }

    #[test]
    fn can_decode_return_false_if_constructor_is_invalid() {
        let val = vec![0x61];
        assert_eq!(u16::can_decode(val.into_iter()), false);
    }

    #[test]
    fn try_decode_returns_correct_value() {
        let val = vec![0x60, 0x00, 0x10];
        assert_eq!(u16::try_decode(val.into_iter()).unwrap(), 16)
    }

    #[test]
    fn decode_returns_error_when_value_bytes_are_invalid() {
        let val = vec![0x56, 0x44];
        assert!(u16::try_decode(val.into_iter()).is_err());
    }

    #[test]
    fn decode_returns_error_when_bytes_are_missing() {
        let val = vec![0x60, 0x01];
        assert!(u16::try_decode(val.into_iter()).is_err());
    }
}
