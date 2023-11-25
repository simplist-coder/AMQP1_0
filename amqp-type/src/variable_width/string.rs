use crate::common::{read_bytes, read_bytes_4};
use crate::error::AppError;
use crate::serde::decode::Decode;
use crate::serde::encode::{Encode, Encoded};


const DEFAULT_CONSTR_SIZE_1: u8 = 0xa1;
const DEFAULT_CONSTR_SIZE_4: u8 = 0xb1;

impl Encode for String {
    fn encode(&self) -> Encoded {
        match self.len() as i32 {
            x if x >= 0 && x <= 255 => Encoded::new_variable(DEFAULT_CONSTR_SIZE_1, self.as_bytes().to_vec()),
            _ => Encoded::new_variable(DEFAULT_CONSTR_SIZE_4, self.as_bytes().to_vec()),
        }
    }
}

impl Decode for String {
    fn can_decode(iter: impl Iterator<Item=u8>) -> bool {
        match iter.peekable().peek() {
            Some(&DEFAULT_CONSTR_SIZE_1) => true,
            Some(&DEFAULT_CONSTR_SIZE_4) => true,
            _ => false,
        }
    }

    fn try_decode(mut iter: impl Iterator<Item=u8>) -> Result<Self, AppError> where Self: Sized {
        match iter.next() {
            Some(DEFAULT_CONSTR_SIZE_1) => Ok(parse_small_string(&mut iter)?),
            Some(DEFAULT_CONSTR_SIZE_4) => Ok(parse_large_string(&mut iter)?),
            Some(illegal) => Err(AppError::DeserializationIllegalConstructorError(illegal)),
            None => Err(AppError::IteratorEmptyOrTooShortError),
        }
    }
}

fn parse_small_string(iter: &mut impl Iterator<Item=u8>) -> Result<String, AppError> {
    match iter.next() {
        Some(size) => Ok(String::from_utf8(read_bytes(iter, size as usize)?)?),
        None => Err(AppError::IteratorEmptyOrTooShortError),
    }
}

fn parse_large_string(iter: &mut impl Iterator<Item=u8>) -> Result<String, AppError> {
    let size = u32::from_be_bytes(read_bytes_4(iter)?);
    Ok(String::from_utf8(read_bytes(iter, size as usize)?)?)
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn test_encode_empty_string() {
        let empty_string = String::new();
        let encoded = empty_string.encode().to_bytes();

        let mut expected = vec![DEFAULT_CONSTR_SIZE_1];
        let len = empty_string.into_bytes().len() as u8;
        expected.append(&mut len.to_be_bytes().to_vec());

        assert_eq!(encoded, expected);
    }

    #[test]
    fn test_encode_small_string() {
        let small_string = "Test".to_string();
        let encoded = small_string.encode().to_bytes();

        let mut expected = vec![DEFAULT_CONSTR_SIZE_1];
        let mut bytes = small_string.into_bytes();
        expected.append(&mut (bytes.len() as u8).to_be_bytes().to_vec());
        expected.append(&mut bytes);

        assert_eq!(encoded, expected);
    }

    #[test]
    fn test_encode_boundary_string() {
        let boundary_string = "a".repeat(255);
        let encoded = boundary_string.encode().to_bytes();

        let mut expected = vec![DEFAULT_CONSTR_SIZE_1];
        let mut bytes = boundary_string.into_bytes();
        expected.append(&mut (bytes.len() as u8).to_be_bytes().to_vec());
        expected.append(&mut bytes);

        assert_eq!(encoded, expected);
    }

    #[test]
    fn test_encode_large_string() {
        let large_string = "a".repeat(256);
        let encoded = large_string.encode().to_bytes();

        let mut expected = vec![DEFAULT_CONSTR_SIZE_4];
        let mut bytes = large_string.into_bytes();
        expected.append(&mut (bytes.len() as u32).to_be_bytes().to_vec());
        expected.append(&mut bytes);

        assert_eq!(encoded, expected);
    }
}
