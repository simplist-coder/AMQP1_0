use crate::error::AppError;
use crate::serde::decode::Decode;
use crate::serde::encode::{Encode, Encoded};
use crate::verify::verify_bytes_read_eq;

const DEFAULT_CONSTR: u8 = 0x61;

impl Encode for i16 {
    fn encode(&self) -> Encoded {
        Encoded::new_fixed(0x61, self.to_be_bytes().to_vec())
    }
}
impl Decode for i16 {
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
            Some(DEFAULT_CONSTR) => Ok(parse_i16(iter)?),
            Some(c) => Err(AppError::DeserializationIllegalConstructorError(c)),
            None => Err(AppError::IteratorEmptyOrTooShortError),
        }
    }
}

fn parse_i16(iter: impl Iterator<Item = u8>) -> Result<i16, AppError> {
    let mut val_bytes = [0; 2];
    let mut index = 0;
    for b in iter.take(2) {
        val_bytes[index] = b;
        index += 1;
    }
    verify_bytes_read_eq(index, 2)?;
    Ok(i16::from_be_bytes(val_bytes))
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn construct_ushort() {
        let val: i16 = 16;
        assert_eq!(val.encode().constructor(), 0x61);
    }

    #[test]
    fn can_deocde_returns_true_if_constructor_is_valid() {
        let val = vec![0x61, 0x41];
        assert_eq!(i16::can_decode(val.into_iter()), true);
    }

    #[test]
    fn can_decode_return_false_if_constructor_is_invalid() {
        let val = vec![0x60];
        assert_eq!(i16::can_decode(val.into_iter()), false);
    }

    #[test]
    fn try_decode_returns_correct_value() {
        let val = vec![0x61, 0x00, 0x10];
        assert_eq!(i16::try_decode(val.into_iter()).unwrap(), 16)
    }

    #[test]
    fn decode_returns_error_when_value_bytes_are_invalid() {
        let val = vec![0x56, 0x44];
        assert!(i16::try_decode(val.into_iter()).is_err());
    }

    #[test]
    fn decode_returns_error_when_bytes_are_missing() {
        let val = vec![0x61, 0x01];
        assert!(i16::try_decode(val.into_iter()).is_err());
    }
}