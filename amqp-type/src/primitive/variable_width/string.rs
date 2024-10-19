use crate::constants::constructors::{STRING, STRING_SHORT};
use crate::serde::decode::Decode;
use crate::serde::encode::{Encode, Encoded};
use amqp_error::AppError;
use amqp_utils::sync_util::{read_bytes, read_bytes_4};
use std::vec::IntoIter;

impl Encode for String {
    fn encode(self) -> Encoded {
        match self.len() as i32 {
            x if (0..=255).contains(&x) => {
                Encoded::new_variable(STRING_SHORT, self.as_bytes().to_vec())
            }
            _ => Encoded::new_variable(STRING, self.as_bytes().to_vec()),
        }
    }
}

impl Decode for String {
    fn try_decode(constructor: u8, stream: &mut IntoIter<u8>) -> Result<Self, AppError>
    where
        Self: Sized,
    {
        match constructor {
            STRING_SHORT => Ok(parse_small_string(stream)?),
            STRING => Ok(parse_large_string(stream)?),
            illegal => Err(AppError::DeserializationIllegalConstructorError(illegal)),
        }
    }
}

fn parse_small_string(iter: &mut IntoIter<u8>) -> Result<String, AppError> {
    match iter.next() {
        Some(size) => Ok(String::from_utf8(read_bytes(iter, size as usize)?)?),
        None => Err(AppError::IteratorEmptyOrTooShortError),
    }
}

fn parse_large_string(iter: &mut IntoIter<u8>) -> Result<String, AppError> {
    let size = u32::from_be_bytes(read_bytes_4(iter)?);
    Ok(String::from_utf8(read_bytes(iter, size as usize)?)?)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_encode_empty_string() {
        let empty_string = String::new();
        let encoded = empty_string.clone().encode().into_bytes();

        let mut expected = vec![STRING_SHORT];
        let len = empty_string.into_bytes().len() as u8;
        expected.append(&mut len.to_be_bytes().to_vec());

        assert_eq!(encoded, expected);
    }

    #[test]
    fn test_encode_small_string() {
        let small_string = "Test".to_string();
        let encoded = small_string.clone().encode().into_bytes();

        let mut expected = vec![STRING_SHORT];
        let mut bytes = small_string.into_bytes();
        expected.append(&mut (bytes.len() as u8).to_be_bytes().to_vec());
        expected.append(&mut bytes);

        assert_eq!(encoded, expected);
    }

    #[test]
    fn test_encode_boundary_string() {
        let boundary_string = "a".repeat(255);
        let encoded = boundary_string.clone().encode().into_bytes();

        let mut expected = vec![STRING_SHORT];
        let mut bytes = boundary_string.into_bytes();
        expected.append(&mut (bytes.len() as u8).to_be_bytes().to_vec());
        expected.append(&mut bytes);

        assert_eq!(encoded, expected);
    }

    #[test]
    fn test_encode_large_string() {
        let large_string = "a".repeat(256);
        let encoded = large_string.clone().encode().into_bytes();

        let mut expected = vec![STRING];
        let mut bytes = large_string.into_bytes();
        expected.append(&mut (bytes.len() as u32).to_be_bytes().to_vec());
        expected.append(&mut bytes);

        assert_eq!(encoded, expected);
    }

    #[test]
    fn test_decode_small_string() {
        let data = vec![5, b'H', b'e', b'l', b'l', b'o'];
        let result = String::try_decode(STRING_SHORT, &mut data.into_iter()).unwrap();
        assert_eq!(result, "Hello".to_string());
    }

    #[test]
    fn test_decode_large_string() {
        let size_bytes = 11u32.to_be_bytes();
        let mut data = vec![size_bytes[0], size_bytes[1], size_bytes[2], size_bytes[3]];
        data.extend_from_slice(b"Hello World");
        let result = String::try_decode(STRING, &mut data.into_iter()).unwrap();
        assert_eq!(result, "Hello World".to_string());
    }

    #[test]
    fn test_illegal_constructor() {
        let data = vec![5, b'E', b'r', b'r', b'o', b'r'];
        let result = String::try_decode(0xFF, &mut data.into_iter());
        assert!(matches!(
            result,
            Err(AppError::DeserializationIllegalConstructorError(0xFF))
        ));
    }

    #[test]
    fn test_iterator_empty_or_too_short() {
        let data = vec![];
        let result = String::try_decode(STRING, &mut data.into_iter());
        assert!(matches!(
            result,
            Err(AppError::IteratorEmptyOrTooShortError)
        ));
    }

    #[test]
    fn test_utf8_compliance() {
        let data = vec![2, 0xC3, 0xA9]; // 'é' in UTF-8
        let result = String::try_decode(STRING_SHORT, &mut data.into_iter()).unwrap();
        assert_eq!(result, "é".to_string());
    }
}
