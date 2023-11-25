use crate::common::{read_bytes, read_bytes_4};
use crate::error::AppError;
use crate::serde::decode::Decode;
use crate::serde::encode::{Encode, Encoded};

const DEFAULT_CONSTR_SIZE_1: u8 = 0xa0;
const DEFAULT_CONSTR_SIZE_4: u8 = 0xb0;

#[derive(Hash, Eq, PartialEq)]
pub struct Binary(Vec<u8>);

impl Encode for Binary {
    fn encode(&self) -> Encoded {
        match self.0.len() {
            x if x <= 255 => Encoded::new_variable(DEFAULT_CONSTR_SIZE_1, self.0.to_owned()),
            _ => Encoded::new_variable(DEFAULT_CONSTR_SIZE_4, self.0.to_owned()),
        }
    }
}

impl Decode for Binary {
    fn can_decode(iter: impl Iterator<Item=u8>) -> bool {
        match iter.peekable().peek() {
            Some(&DEFAULT_CONSTR_SIZE_1) => true,
            Some(&DEFAULT_CONSTR_SIZE_4) => true,
            _ => false,
        }
    }

    fn try_decode(mut iter: impl Iterator<Item=u8>) -> Result<Self, AppError> where Self: Sized {
        match iter.next() {
            Some(DEFAULT_CONSTR_SIZE_1) => Ok(parse_small_binary(&mut iter)?),
            Some(DEFAULT_CONSTR_SIZE_4) => Ok(parse_large_binary(&mut iter)?),
            Some(illegal) => Err(AppError::DeserializationIllegalConstructorError(illegal)),
            None => Err(AppError::IteratorEmptyOrTooShortError),
        }
    }
}

fn parse_small_binary(iter: &mut impl Iterator<Item=u8>) -> Result<Binary, AppError> {
    match iter.next() {
        Some(size) => Ok(Binary(read_bytes(iter, size as usize)?)),
        None => Err(AppError::IteratorEmptyOrTooShortError),
    }
}

fn parse_large_binary(iter: &mut impl Iterator<Item=u8>) -> Result<Binary, AppError> {
    let size = u32::from_be_bytes(read_bytes_4(iter)?);
    Ok(Binary(read_bytes(iter, size as usize)?))
}

impl From<Vec<u8>> for Binary {
    fn from(value: Vec<u8>) -> Self {
        Binary(value)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn construct_binary() {
        let val = Binary(Vec::new());
        assert_eq!(val.encode().constructor(), 0xa0);
    }

    #[test]
    fn test_encode_short_data() {
        let data = vec![0; 255]; // 255 bytes of data
        let binary = Binary(data.clone());
        let encoded = binary.encode();
        let mut expected = vec![DEFAULT_CONSTR_SIZE_1];
        let x = (data.len() as u8).to_be_bytes();
        expected.append(&mut x.to_vec());
        expected.append(&mut data.clone());

        assert_eq!(encoded.constructor(), DEFAULT_CONSTR_SIZE_1);
        assert_eq!(encoded.to_bytes(), expected);
    }

    #[test]
    fn test_encode_long_data() {
        let data = vec![0; 256]; // 256 bytes of data
        let binary = Binary(data.clone());
        let encoded = binary.encode();
        let mut expected = vec![DEFAULT_CONSTR_SIZE_4];
        let x = (data.len() as u32).to_be_bytes();
        expected.append(&mut x.to_vec());
        expected.append(&mut data.clone());

        assert_eq!(encoded.constructor(), DEFAULT_CONSTR_SIZE_4);
        assert_eq!(encoded.to_bytes(), expected);
    }

    #[test]
    fn test_decode_small_binary() {
        let data = vec![DEFAULT_CONSTR_SIZE_1, 3, 0x01, 0x02, 0x03];
        let result = Binary::try_decode(data.into_iter()).unwrap();
        assert_eq!(result.0, vec![0x01, 0x02, 0x03]);
    }

    #[test]
    fn test_decode_large_binary() {
        let size_bytes = (4u32).to_be_bytes();
        let data = vec![
            DEFAULT_CONSTR_SIZE_4,
            size_bytes[0],
            size_bytes[1],
            size_bytes[2],
            size_bytes[3],
            0x01, 0x02, 0x03, 0x04
        ];
        let result = Binary::try_decode(data.into_iter()).unwrap();
        assert_eq!(result.0, vec![0x01, 0x02, 0x03, 0x04]);
    }

    #[test]
    fn test_illegal_constructor() {
        let data = vec![0xFF, 3, 0x01, 0x02, 0x03];
        let result = Binary::try_decode(data.into_iter());
        assert!(matches!(result, Err(AppError::DeserializationIllegalConstructorError(0xFF))));
    }

    #[test]
    fn test_iterator_empty_or_too_short() {
        let data: Vec<u8> = vec![];
        let result = Binary::try_decode(data.into_iter());
        assert!(matches!(result, Err(AppError::IteratorEmptyOrTooShortError)));
    }
}
