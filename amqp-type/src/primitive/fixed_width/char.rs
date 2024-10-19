use crate::constants::constructors::CHAR;
use crate::primitive::fixed_width::char;
use crate::serde::decode::Decode;
use crate::serde::encode::{Encode, Encoded};
use amqp_error::AppError;
use amqp_utils::sync_util::read_bytes_4;
use std::vec::IntoIter;

impl Encode for char {
    fn encode(self) -> Encoded {
        Encoded::new_fixed(CHAR, (self as u32).to_be_bytes().to_vec())
    }
}

impl Decode for char {
    fn try_decode(constructor: u8, stream: &mut IntoIter<u8>) -> Result<Self, AppError>
    where
        Self: Sized,
    {
        match constructor {
            CHAR => Ok(parse_char(stream)?),
            c => Err(AppError::DeserializationIllegalConstructorError(c)),
        }
    }
}

fn parse_char(iter: &mut IntoIter<u8>) -> Result<char, AppError> {
    let byte_vals = read_bytes_4(iter)?;
    match char::from_u32(u32::from_be_bytes(byte_vals)) {
        None => Err(AppError::InvalidChar),
        Some(c) => Ok(c),
    }
}

#[cfg(test)]
mod test {
    use super::*;

    use crate::constants::constructors::CHAR;

    #[test]
    fn construct_char() {
        let val = 'a';
        assert_eq!(val.encode().constructor(), 0x73);
    }

    #[test]
    fn test_encode_char() {
        let mut ascii = vec![CHAR];
        let mut non_ascii = vec![CHAR];
        let mut utf_8 = vec![CHAR];
        ascii.extend(('a' as u32).to_be_bytes().to_vec());
        non_ascii.extend(('ñ' as u32).to_be_bytes().to_vec());
        utf_8.extend(('😊' as u32).to_be_bytes().to_vec());

        assert_eq!(ascii, 'a'.encode().serialize());
        assert_eq!(non_ascii, 'ñ'.encode().serialize());
        assert_eq!(utf_8, '😊'.encode().serialize());
    }

    #[test]
    fn test_successful_deserialization() {
        let value = ('A' as u32).to_be_bytes().to_vec();

        match char::try_decode(CHAR, &mut value.into_iter()) {
            Ok(decoded_char) => assert_eq!('A', decoded_char),
            Err(e) => panic!("Unexpected error: {e:?}"),
        }
    }

    #[test]
    fn test_illegal_constructor_deserialization() {
        let illegal_constructor = 0xFF;
        let bytes = vec![];

        match char::try_decode(illegal_constructor, &mut bytes.into_iter()) {
            Ok(_) => panic!("Expected an error, but deserialization succeeded"),
            Err(AppError::DeserializationIllegalConstructorError(c)) => {
                assert_eq!(illegal_constructor, c);
            }
            Err(e) => panic!("Unexpected error type: {e:?}"),
        }
    }

    #[test]
    fn test_empty_iterator_deserialization() {
        let bytes = vec![]; // Empty vector

        match char::try_decode(CHAR, &mut bytes.into_iter()) {
            Ok(_) => panic!("Expected an error, but deserialization succeeded"),
            Err(AppError::IteratorEmptyOrTooShortError) => (), // Expected outcome
            Err(e) => panic!("Unexpected error type: {e:?}"),
        }
    }

    #[test]
    fn test_invalid_char_deserialization() {
        let bytes = vec![CHAR, 0xFF, 0xFF, 0xFF, 0xFF]; // Invalid Unicode sequence

        match char::try_decode(CHAR, &mut bytes.into_iter()) {
            Ok(_) => panic!("Expected an error, but deserialization succeeded"),
            Err(AppError::InvalidChar) => (), // Expected outcome
            Err(e) => panic!("Unexpected error type: {e:?}"),
        }
    }
}
