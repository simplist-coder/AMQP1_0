use crate::common::read_bytes_2;
use crate::error::AppError;
use crate::serde::decode::Decode;
use crate::serde::encode::{Encode, Encoded};

impl Encode for u16 {
    fn encode(&self) -> Encoded {
        Encoded::new_fixed(0x60, self.to_be_bytes().to_vec())
    }
}

impl Decode for u16 {
    fn can_decode(iter: impl Iterator<Item = u8>) -> bool {
        match iter.peekable().peek() {
            Some(0x60) => true,
            _ => false,
        }
    }

    fn try_decode(mut iter: impl Iterator<Item = u8>) -> Result<Self, crate::error::AppError>
    where
        Self: Sized,
    {
        match iter.next() {
            Some(0x60) => Ok(parse_u16(iter)?),
            Some(c) => Err(AppError::DeserializationIllegalConstructorError(c)),
            None => Err(AppError::IteratorEmptyOrTooShortError),
        }
    }
}

fn parse_u16(iter: impl Iterator<Item = u8>) -> Result<u16, AppError> {
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
