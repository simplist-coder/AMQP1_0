use crate::serde::decode::Decode;
use crate::verify::verify_bytes_read_eq;
use crate::{
    error::AppError,
    serde::encode::{Encode, Encoded},
};

const DEFAULT_CONSTR: u8 = 0x71;
const SMALL_INT_CONSTR: u8 = 0x54;

impl Encode for i32 {
    fn encode(&self) -> Encoded {
        match self {
            x if x >= &-128 && x <= &127 => Encoded::new_fixed(0x54, x.to_be_bytes().to_vec()),
            _ => Encoded::new_fixed(0x71, self.to_be_bytes().to_vec()),
        }
    }
}

impl Decode for i32 {
    fn can_decode(iter: impl Iterator<Item = u8>) -> bool {
        match iter.peekable().peek() {
            Some(&DEFAULT_CONSTR) => true,
            Some(&SMALL_INT_CONSTR) => true,
            _ => false,
        }
    }

    fn try_decode(mut iter: impl Iterator<Item = u8>) -> Result<Self, crate::error::AppError>
    where
        Self: Sized,
    {
        match iter.next() {
            Some(DEFAULT_CONSTR) => Ok(parse_i32(iter)?),
            Some(SMALL_INT_CONSTR) => Ok(parse_small_i32(iter)?),
            Some(c) => Err(AppError::DeserializationIllegalConstructorError(c)),
            None => Err(AppError::IteratorEmptyOrTooShortError),
        }
    }
}

fn parse_i32(iter: impl Iterator<Item = u8>) -> Result<i32, AppError> {
    let mut val_bytes = [0; 4];
    let mut index = 0;
    for b in iter.take(4) {
        val_bytes[index] = b;
        index += 1;
    }
    verify_bytes_read_eq(index, 4)?;
    Ok(i32::from_be_bytes(val_bytes))
}

fn parse_small_i32(mut iter: impl Iterator<Item = u8>) -> Result<i32, AppError> {
    if let Some(val) = iter.next() {
        Ok(val as i32)
    } else {
        Err(AppError::IteratorEmptyOrTooShortError)
    }
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn can_deocde_returns_true_if_constructor_is_valid() {
        let val = vec![0x71, 0x41];
        let small_val = vec![0x54, 0x41];
        assert_eq!(i32::can_decode(val.into_iter()), true);
        assert_eq!(i32::can_decode(small_val.into_iter()), true);
    }

    #[test]
    fn can_decode_return_false_if_constructor_is_invalid() {
        let val = vec![0x70];
        assert_eq!(i32::can_decode(val.into_iter()), false);
    }

    #[test]
    fn try_decode_returns_correct_value() {
        let val = vec![0x71, 0x00, 0x00, 0x00, 0x10];
        assert_eq!(i32::try_decode(val.into_iter()).unwrap(), 16)
    }

    #[test]
    fn decode_returns_error_when_value_bytes_are_invalid() {
        let val = vec![0x56, 0x44];
        assert!(i32::try_decode(val.into_iter()).is_err());
    }

    #[test]
    fn decode_returns_error_when_bytes_are_missing() {
        let val = vec![0x71, 0x01];
        assert!(i32::try_decode(val.into_iter()).is_err());
    }

    #[test]
    fn try_decode_can_decode_smallulong_values() {
        let val = vec![0x54, 0xff];
        assert_eq!(i32::try_decode(val.into_iter()).unwrap(), 255);
    }

    #[test]
    fn try_decode_returns_error_when_parsing_small_ulong_and_bytes_are_missing() {
        let val = vec![0x54];
        assert!(i32::try_decode(val.into_iter()).is_err());
    }

    #[test]
    fn construct_int() {
        let val = 500;
        assert_eq!(val.encode().constructor(), 0x71);
    }

    #[test]
    fn amqp_encodes_ints_between_neg_128_and_127_as_smallint() {
        let lower = -128;
        let higher = 127;
        assert_eq!(lower.encode().constructor(), 0x54);
        assert_eq!(higher.encode().constructor(), 0x54);
    }
}