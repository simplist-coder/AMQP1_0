use crate::serde::encode::{Encode, Encoded};
use crate::serde::decode::Decode;

const BYTE_LEN: usize = 2;

impl Encode for u16 {
    fn encode(&self) -> Encoded {
        Encoded::new_fixed(0x60, self.to_be_bytes().to_vec())
    }
}

impl Decode for u16 {
    fn can_decode(iter: impl Iterator<Item = u8>) -> bool {
        match iter.peekable().peek() {
            Some(0x60) => true,
            _ => false
        }
    }

    fn try_decode(mut iter: impl Iterator<Item = u8>) -> Result<Self, crate::error::AppError>
        where
            Self: Sized {
        let mut val_bytes = [0; BYTE_LEN];
        let con = iter.next();
        let mut index = 0;
        for b in iter.take(BYTE_LEN) {
            val_bytes[index] = b;
            index += 1;
        }
        Ok(u16::from_be_bytes(val_bytes))
    }
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
}
