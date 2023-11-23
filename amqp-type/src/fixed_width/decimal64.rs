use std::hash::{Hash, Hasher};

use crate::common::read_bytes_8;
use crate::error::AppError;
use crate::serde::decode::Decode;
use crate::serde::encode::{Encode, Encoded};

const DEFAULT_CONSTR: u8 = 0x84;

pub struct Decimal64(f64);

impl Encode for Decimal64 {
    fn encode(&self) -> Encoded {
        Encoded::new_fixed(
            DEFAULT_CONSTR,
            self.0.to_be_bytes().to_vec()
        )
    }
}

impl Decode for Decimal64 {
    fn can_decode(iter: impl Iterator<Item=u8>) -> bool {
        match iter.peekable().peek() {
            Some(&DEFAULT_CONSTR) => true,
            _ => false,
        }
    }

    fn try_decode(mut iter: impl Iterator<Item=u8>) -> Result<Self, AppError> where Self: Sized {
        match iter.next() {
            Some(DEFAULT_CONSTR) => Ok(parse_decimal64(iter)?),
            Some(c) => Err(AppError::DeserializationIllegalConstructorError(c)),
            None => Err(AppError::IteratorEmptyOrTooShortError),
        }
    }
}

fn parse_decimal64(iter: impl Iterator<Item=u8>) -> Result<Decimal64, AppError> {
    let byte_vals = read_bytes_8(iter)?;
    Ok(Decimal64(f64::from_be_bytes(byte_vals)))
}

impl From<f64> for Decimal64 {
    fn from(value: f64) -> Self {
        Decimal64(value)
    }
}

impl PartialEq for Decimal64 {
    fn eq(&self, other: &Self) -> bool {
        self.0.to_bits().eq(&other.0.to_bits())
    }
}

impl Eq for Decimal64 {}

impl Hash for Decimal64 {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.0.to_bits().hash(state)
    }
}

#[cfg(test)]
mod test {
    use bytes::BufMut;

    use super::*;

    #[test]
    fn construct_decimal_64() {
        let val: Decimal64 = 64.0.into();
        assert_eq!(val.encode().constructor(), 0x84);
    }

    #[test]
    fn test_successful_deserialization() {
        let value = 1.2345f64;
        let mut data = vec![DEFAULT_CONSTR];
        data.put_f64(value); // Put an f64 into the buffer
        let mut iter = data.into_iter();

        match Decimal64::try_decode(&mut iter) {
            Ok(decimal) => assert_eq!(value, decimal.0),
            Err(e) => panic!("Unexpected error: {:?}", e),
        }
    }

    #[test]
    fn test_illegal_constructor_deserialization() {
        let illegal_constructor = 0xFF; // Assuming this is not DEFAULT_CONSTR
        let bytes = vec![illegal_constructor, /* other bytes */];
        let mut iter = bytes.into_iter();

        match Decimal64::try_decode(&mut iter) {
            Ok(_) => panic!("Expected an error, but deserialization succeeded"),
            Err(AppError::DeserializationIllegalConstructorError(c)) => assert_eq!(illegal_constructor, c),
            Err(e) => panic!("Unexpected error type: {:?}", e),
        }
    }

    #[test]
    fn test_empty_iterator_deserialization() {
        let bytes = vec![]; // Empty vector
        let mut iter = bytes.into_iter();

        match Decimal64::try_decode(&mut iter) {
            Ok(_) => panic!("Expected an error, but deserialization succeeded"),
            Err(AppError::IteratorEmptyOrTooShortError) => (), // Expected outcome
            Err(e) => panic!("Unexpected error type: {:?}", e),
        }
    }
}
