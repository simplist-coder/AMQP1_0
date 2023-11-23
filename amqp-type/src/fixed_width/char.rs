use crate::common::read_bytes_4;
use crate::error::AppError;
use crate::fixed_width::char;
use crate::serde::decode::Decode;
use crate::serde::encode::{Encode, Encoded};

const DEFAULT_CONSTR: u8 = 0x73;

impl Encode for char {
    fn encode(&self) -> Encoded {
        Encoded::new_fixed(0x73, self.to_string().into_bytes())
    }
}

impl Decode for char {
    fn can_decode(iter: impl Iterator<Item=u8>) -> bool {
        match iter.peekable().peek() {
            Some(&DEFAULT_CONSTR) => true,
            _ => false,
        }
    }

    fn try_decode(mut iter: impl Iterator<Item=u8>) -> Result<Self, AppError> where Self: Sized {
        match iter.next() {
            Some(DEFAULT_CONSTR) => Ok(parse_char(iter)?),
            Some(c) => Err(AppError::DeserializationIllegalConstructorError(c)),
            None => Err(AppError::IteratorEmptyOrTooShortError),
        }
    }
}

fn parse_char(iter: impl Iterator<Item=u8> + Sized) -> Result<char, AppError> {
    let byte_vals = read_bytes_4(iter)?;
    match char::from_u32(u32::from_be_bytes(byte_vals)) {
        None => Err(AppError::InvalidChar),
        Some(c) => Ok(c)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn construct_char() {
        let val = 'a';
        assert_eq!(val.encode().constructor(), 0x73);
    }

    #[test]
    fn test_successful_deserialization() {
        let value = 'A';
        let mut data = vec![DEFAULT_CONSTR];
        data.extend_from_slice(&(value as u32).to_be_bytes());
        let mut iter = data.into_iter();

        match char::try_decode(&mut iter) {
            Ok(decoded_char) => assert_eq!(value, decoded_char),
            Err(e) => panic!("Unexpected error: {:?}", e),
        }
    }

    #[test]
    fn test_illegal_constructor_deserialization() {
        let illegal_constructor = 0xFF;
        let bytes = vec![illegal_constructor];
        let mut iter = bytes.into_iter();

        match char::try_decode(&mut iter) {
            Ok(_) => panic!("Expected an error, but deserialization succeeded"),
            Err(AppError::DeserializationIllegalConstructorError(c)) => assert_eq!(illegal_constructor, c),
            Err(e) => panic!("Unexpected error type: {:?}", e),
        }
    }

    #[test]
    fn test_empty_iterator_deserialization() {
        let bytes = vec![]; // Empty vector
        let mut iter = bytes.into_iter();

        match char::try_decode(&mut iter) {
            Ok(_) => panic!("Expected an error, but deserialization succeeded"),
            Err(AppError::IteratorEmptyOrTooShortError) => (), // Expected outcome
            Err(e) => panic!("Unexpected error type: {:?}", e),
        }
    }

    #[test]
    fn test_invalid_char_deserialization() {
        let bytes = vec![DEFAULT_CONSTR, 0xFF, 0xFF, 0xFF, 0xFF]; // Invalid Unicode sequence
        let mut iter = bytes.into_iter();

        match char::try_decode(&mut iter) {
            Ok(_) => panic!("Expected an error, but deserialization succeeded"),
            Err(AppError::InvalidChar) => (), // Expected outcome
            Err(e) => panic!("Unexpected error type: {:?}", e),
        }
    }
}
