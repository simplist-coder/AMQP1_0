use crate::constants::BOOLEAN;
use crate::constants::BOOLEAN_FALSE;
use crate::constants::BOOLEAN_TRUE;
use crate::serde::decode::Decode;
use crate::serde::encode::{Encode, Encoded};
use amqp_error::AppError;
use std::vec::IntoIter;

#[cfg(not(feature = "zero-length-encoding"))]
impl Encode for bool {
    fn encode(self) -> Encoded {
        if self {
            Encoded::new_fixed(BOOLEAN, vec![0x01])
        } else {
            Encoded::new_fixed(BOOLEAN, vec![0x00])
        }
    }
}

#[cfg(feature = "zero-length-encoding")]
impl Encode for bool {
    fn encode(self) -> Encoded {
        match self {
            true => BOOLEAN_TRUE.into(),
            false => BOOLEAN_FALSE.into(),
        }
    }
}

impl Decode for bool {
    fn try_decode(constructor: u8, iter: &mut IntoIter<u8>) -> Result<Self, AppError>
    where
        Self: Sized,
    {
        match constructor {
            BOOLEAN_TRUE => Ok(true),
            BOOLEAN_FALSE => Ok(false),
            BOOLEAN => {
                let val = iter.next();
                match (constructor, val) {
                    (BOOLEAN, Some(0x00)) => Ok(false),
                    (BOOLEAN, Some(0x01)) => Ok(true),
                    (c, _) => Err(AppError::DeserializationIllegalConstructorError(c)),
                }
            }
            c => Err(AppError::DeserializationIllegalConstructorError(c)),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    #[cfg(not(feature = "zero-length-encoding"))]
    fn construct_bool() {
        assert_eq!(true.encode().constructor(), 0x56);
    }

    #[test]
    #[cfg(not(feature = "zero-length-encoding"))]
    fn bool_gets_encoded_correctly() {
        assert_eq!(true.encode().into_bytes(), vec![0x56, 0x01]);
        assert_eq!(false.encode().into_bytes(), vec![0x56, 0x00]);
    }

    #[test]
    #[cfg(feature = "zero-length-encoding")]
    fn amqp_type_constructs_bool_false_as_zero_length() {
        assert_eq!(false.encode().constructor(), 0x42);
    }

    #[test]
    #[cfg(feature = "zero-length-encoding")]
    fn amqp_type_constructs_bool_true_as_zero_length() {
        assert_eq!(true.encode().constructor(), 0x41)
    }

    #[test]
    fn decode_returns_error_when_value_bytes_are_invalid() {
        let val_true = vec![0x34];
        let val_false = vec![0x44];
        assert!(bool::try_decode(0x56, &mut val_true.into_iter()).is_err());
        assert!(bool::try_decode(0x56, &mut val_false.into_iter()).is_err());
    }

    #[test]
    fn try_decode_returns_correct_value_if_bytes_are_valid() {
        let val_true = vec![0x01];
        let val_false = vec![0x00];
        let val_true_zero_length = vec![];
        let val_false_zero_length = vec![];
        assert!(bool::try_decode(0x56, &mut val_true.into_iter()).unwrap());
        assert!(!bool::try_decode(0x56, &mut val_false.into_iter()).unwrap());
        assert!(bool::try_decode(BOOLEAN_TRUE, &mut val_true_zero_length.into_iter()).unwrap());
        assert!(!bool::try_decode(BOOLEAN_FALSE, &mut val_false_zero_length.into_iter()).unwrap());
    }

    #[test]
    fn try_decode_zero_length_encoded_bool_does_not_advance_the_stream() {
        let vals = vec![1, 2, 3];
        let mut stream = vals.into_iter();
        assert!(bool::try_decode(BOOLEAN_TRUE, &mut stream).unwrap());
        assert!(!bool::try_decode(BOOLEAN_FALSE, &mut stream).unwrap());
        assert_eq!(stream.next(), Some(1));
        assert_eq!(stream.next(), Some(2));
        assert_eq!(stream.next(), Some(3));
        assert_eq!(stream.next(), None);
        assert_eq!(stream.next(), None);
    }
}
