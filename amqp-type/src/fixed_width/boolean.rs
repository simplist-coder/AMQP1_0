use crate::error::AppError;
use crate::serde::decode::Decode;
use crate::serde::encode::{Encode, Encoded};

#[cfg(not(feature = "zero-length-bools"))]
const DEFAULT_CONSTR: u8 = 0x56;

#[cfg(feature = "zero-length-bools")]
const DEFAULT_CONSTR_TRUE: u8 = 0x41;
#[cfg(feature = "zero-length-bools")]
const DEFAULT_CONSTR_FALSE: u8 = 0x42;

#[cfg(not(feature = "zero-length-bools"))]
impl Encode for bool {
    fn encode(&self) -> Encoded {
        match self {
            true => Encoded::new_fixed(DEFAULT_CONSTR, vec![0x01]),
            false => Encoded::new_fixed(DEFAULT_CONSTR, vec![0x00]),
        }
    }
}

#[cfg(feature = "zero-length-bools")]
impl Encode for bool {
    fn encode(&self) -> Encoded {
        match self {
            true => DEFAULT_CONSTR_TRUE.into(),
            false => DEFAULT_CONSTR_FALSE.into(),
        }
    }
}

#[cfg(not(feature = "zero-length-bools"))]
impl Decode for bool {
    fn can_decode(data: impl Iterator<Item=u8>) -> bool {
        let mut iter = data.into_iter().peekable();
        match iter.peek() {
            Some(&DEFAULT_CONSTR) => true,
            _ => false,
        }
    }

    fn try_decode(mut iter: impl Iterator<Item=u8>) -> Result<Self, AppError>
        where
            Self: Sized,
    {
        let con = iter.next();
        let val = iter.next();
        match (con, val) {
            (Some(c), Some(v)) if c == DEFAULT_CONSTR && v == 0x00 => Ok(false),
            (Some(c), Some(v)) if c == DEFAULT_CONSTR && v == 0x01 => Ok(true),
            (Some(c), _) => Err(AppError::DeserializationIllegalConstructorError(c)),
            (None, _) => Err(AppError::IteratorEmptyOrTooShortError),
        }
    }
}

#[cfg(feature = "zero-length-bools")]
impl Decode for bool {
    fn can_decode(data: Iterator<Item=u8>) -> bool {
        let mut iter = data.into_iter().peekable();
        match iter.peek() {
            Some(DEFAULT_CONSTR_TRUE) => true,
            Some(DEFAULT_CONSTR_FALSE) => true,
            _ => false,
        }
    }

    fn try_decode(data: Iterator<Item=u8>) -> Result<Self, AppError>
        where
            Self: Sized,
    {
        if let Some(val) = iter.next() {
            return match val {
                DEFAULT_CONSTR_TRUE => Ok(true),
                DEFAULT_CONSTR_FALSE => Ok(false),
                _ => Err(AppError::DeserializationIllegalConstructorError(val)),
            };
        }
        Err(AppError::IteratorEmptyOrTooShortError)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    #[cfg(not(feature = "zero-length-bools"))]
    fn construct_bool() {
        assert_eq!(true.encode().constructor(), 0x56);
    }

    #[test]
    #[cfg(not(feature = "zero-length-bools"))]
    fn bool_gets_encoded_correctly() {
        assert_eq!(true.encode().to_bytes(), vec![0x56, 0x01]);
        assert_eq!(false.encode().to_bytes(), vec![0x56, 0x00]);
    }

    #[test]
    #[cfg(not(feature = "zero-length-bools"))]
    fn can_decode_returns_true_if_constructor_is_valid() {
        let val_true = vec![0x56, 0x01];
        let val_false = vec![0x56, 0x00];
        assert_eq!(bool::can_decode(val_true.into_iter()), true);
        assert_eq!(bool::can_decode(val_false.into_iter()), true);
    }

    #[test]
    #[cfg(not(feature = "zero-length-bools"))]
    fn can_decode_returns_false_if_constructor_invalid() {
        let val_true = vec![0x88, 0x01];
        let val_false = vec![0x97, 0x00];
        assert_eq!(bool::can_decode(val_true.into_iter()), false);
        assert_eq!(bool::can_decode(val_false.into_iter()), false);
    }

    #[test]
    #[cfg(not(feature = "zero-length-bools"))]
    fn decode_returns_error_when_value_bytes_are_invalid() {
        let val_true = vec![0x56, 0x34];
        let val_false = vec![0x56, 0x44];
        assert!(bool::try_decode(val_true.into_iter()).is_err());
        assert!(bool::try_decode(val_false.into_iter()).is_err());
    }

    #[test]
    #[cfg(not(feature = "zero-length-bools"))]
    fn try_decode_returns_correct_value_if_bytes_are_valid() {
        let val_true = vec![0x56, 0x01];
        let val_false = vec![0x56, 0x00];
        assert_eq!(bool::try_decode(val_true.into_iter()).unwrap(), true);
        assert_eq!(bool::try_decode(val_false.into_iter()).unwrap(), false);
    }

    #[test]
    #[cfg(feature = "zero-length-bools")]
    fn amqp_type_constructs_bool_false_as_zero_length() {
        assert_eq!(false.encode().constructor(), 0x42);
    }

    #[test]
    #[cfg(feature = "zero-length-bools")]
    fn amqp_type_constructs_bool_true_as_zero_length() {
        assert_eq!(true.encode().constructor(), 0x41)
    }
}
