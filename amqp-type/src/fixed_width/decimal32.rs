use std::hash::{Hash, Hasher};
use crate::error::AppError;
use crate::serde::decode::Decode;

use crate::serde::encode::{Encode, Encoded};
use crate::verify::verify_bytes_read_eq;

// 7 digits
const DEFAULT_CONSTR: u8 = 0x74;

#[derive(Debug)]
pub struct Decimal32(f32);

impl Encode for Decimal32 {
    fn encode(&self) -> Encoded {
        Encoded::new_fixed(
            DEFAULT_CONSTR,
            self.0.to_be_bytes().to_vec(),
        )
    }
}

impl Decode for Decimal32 {
    fn can_decode(iter: impl Iterator<Item=u8>) -> bool {
        match iter.peekable().peek() {
            Some(&DEFAULT_CONSTR) => true,
            _ => false,
        }
    }

    fn try_decode(mut iter: impl Iterator<Item=u8>) -> Result<Self, AppError> where Self: Sized {
        match iter.next() {
            Some(DEFAULT_CONSTR) => Ok(parse_decimal32(iter)?),
            Some(c) => Err(AppError::DeserializationIllegalConstructorError(c)),
            None => Err(AppError::IteratorEmptyOrTooShortError),
        }
    }
}

fn parse_decimal32(iter: impl Iterator<Item=u8>) -> Result<Decimal32, AppError> {
    let mut byte_vals = [0; 4];
    let mut index = 0;
    for b in iter.take(4) {
        byte_vals[index] = b;
        index += 1;
    }
    verify_bytes_read_eq(index, 4)?;
    Ok(Decimal32(f32::from_be_bytes(byte_vals)))
}

impl Hash for Decimal32 {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.0.to_bits().hash(state)
    }
}

impl PartialEq for Decimal32 {
    fn eq(&self, other: &Self) -> bool {
        self.0.to_bits().eq(&other.0.to_bits())
    }
}

impl Eq for Decimal32 {}

impl From<f32> for Decimal32 {
    fn from(value: f32) -> Self {
        Decimal32(value)
    }
}

#[cfg(test)]
mod test {
    use bytes::BufMut;
    use super::*;

    #[test]
    fn construct_decimal_32() {
        let val: Decimal32 = 32f32.into();
        assert_eq!(val.encode().constructor(), 0x74);
    }

    #[test]
    fn test_successful_deserialization() {
        let value = 1.2345f32;
        let mut data = vec![DEFAULT_CONSTR];
        data.put_f32(value);
        let mut iter = data.into_iter();

        match Decimal32::try_decode(&mut iter) {
            Ok(decimal) => assert_eq!(value, decimal.0),
            Err(e) => panic!("Unexpected error: {:?}", e),
        }
    }

    #[test]
    fn test_illegal_constructor_deserialization() {
        let illegal_constructor = 0xFF;
        let bytes = vec![illegal_constructor, /* other bytes */];
        let mut iter = bytes.into_iter();

        match Decimal32::try_decode(&mut iter) {
            Ok(_) => panic!("Expected an error, but deserialization succeeded"),
            Err(AppError::DeserializationIllegalConstructorError(c)) => assert_eq!(illegal_constructor, c),
            Err(e) => panic!("Unexpected error type: {:?}", e),
        }
    }

    #[test]
    fn test_empty_iterator_deserialization() {
        let bytes = vec![]; // Empty vector
        let mut iter = bytes.into_iter();

        match Decimal32::try_decode(&mut iter) {
            Ok(_) => panic!("Expected an error, but deserialization succeeded"),
            Err(AppError::IteratorEmptyOrTooShortError) => (), // Expected outcome
            Err(e) => panic!("Unexpected error type: {:?}", e),
        }
    }
}
