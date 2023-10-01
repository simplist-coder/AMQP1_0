use crate::error::AppError;
use crate::serde::decode::Decode;
use crate::serde::encode::{Encode, Encoded};

impl Encode for u8 {
    fn encode(&self) -> Encoded {
        Encoded::new_fixed(0x50, self.to_be_bytes().to_vec())
    }
}

impl Decode for u8 {
    fn can_decode(data: impl Iterator<Item = u8>) -> bool {
        let mut iter = data.into_iter().peekable();
        match iter.peek() {
            Some(0x50) => true,
            _ => false,
        }
    }

    fn try_decode(mut iter: impl Iterator<Item = u8>) -> Result<Self, crate::error::AppError>
    where
        Self: Sized,
    {
        let con = iter.next();
        let val = iter.next();
        match (con, val) {
            (Some(0x50), Some(x)) => Ok(x),
            (Some(c), _) => Err(AppError::DeserializationIllegalConstructorError(c)),
            (_, _) => Err(AppError::IteratorEmptyOrTooShortError),
        }
    }
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn construct_ubyte() {
        let val: u8 = 8;
        assert_eq!(val.encode().constructor(), 0x50);
    }

    #[test]
    fn can_deocde_returns_true_if_constructor_is_valid() {
        let val = vec![0x50, 0x41];
        assert_eq!(u8::can_decode(val.into_iter()), true);
    }

    #[test]
    fn can_decode_return_false_if_constructor_is_invalid() {
        let val = vec![0x51];
        assert_eq!(u8::can_decode(val.into_iter()), false);
    }

    #[test]
    fn try_decode_returns_correct_value() {
        let val = vec![0x50, 0x10];
        assert_eq!(u8::try_decode(val.into_iter()).unwrap(), 16)
    }
}
