use crate::common::read_bytes_16;
use crate::error::AppError;
use crate::serde::decode::Decode;
use crate::serde::encode::{Encode, Encoded};

const DEFAULT_CONSTR: u8 = 0x98;

#[derive(Hash, Eq, PartialEq)]
pub struct Uuid(uuid::Uuid);

impl Encode for Uuid {
    fn encode(&self) -> Encoded {
        Encoded::new_fixed(DEFAULT_CONSTR, self.0.into_bytes().to_vec())
    }
}

impl Decode for Uuid {
    fn can_decode(iter: impl Iterator<Item=u8>) -> bool {
        match iter.peekable().peek() {
            Some(&DEFAULT_CONSTR) => true,
            _ => false,
        }
    }

    fn try_decode(mut iter: impl Iterator<Item=u8>) -> Result<Self, AppError> where Self: Sized {
        match iter.next() {
            Some(DEFAULT_CONSTR) => Ok(parse_uuid(iter)?),
            Some(c) => Err(AppError::DeserializationIllegalConstructorError(c)),
            None => Err(AppError::IteratorEmptyOrTooShortError),
        }
    }
}

fn parse_uuid(iter: impl Iterator<Item=u8> + Sized) -> Result<Uuid, AppError> {
    let byte_vals = read_bytes_16(iter)?;
    Ok(Uuid(uuid::Uuid::from_bytes(byte_vals)))
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn construct_uuid() {
        let val = Uuid(uuid::Uuid::new_v4());
        assert_eq!(val.encode().constructor(), 0x98);
    }

    #[test]
    fn test_encode_correctness() {
        let uuid = Uuid(uuid::Uuid::new_v4());
        let encoded = uuid.encode();
        let mut expected_bytes = Vec::new();
        expected_bytes.push(DEFAULT_CONSTR);
        expected_bytes.extend_from_slice(&uuid.0.into_bytes());

        assert_eq!(encoded.to_bytes(), expected_bytes);
    }

    #[test]
    fn test_decode_success() {
        let uuid = uuid::Uuid::new_v4();
        let bytes = uuid.into_bytes();
        let mut iter = std::iter::once(DEFAULT_CONSTR).chain(bytes.iter().cloned());
        let decoded = Uuid::try_decode(&mut iter);
        assert!(decoded.is_ok());
        assert_eq!(decoded.unwrap().0, uuid)
    }

    #[test]
    fn test_decode_incorrect_constructor() {
        let uuid_bytes = uuid::Uuid::new_v4().into_bytes();
        let incorrect_constructor = 0x99;
        let mut iter = std::iter::once(incorrect_constructor).chain(uuid_bytes.iter().cloned());
        let decoded = Uuid::try_decode(&mut iter);
        assert!(matches!(decoded, Err(AppError::DeserializationIllegalConstructorError(_))));
    }

    #[test]
    fn test_decode_short_byte_sequence() {
        let short_bytes = vec![DEFAULT_CONSTR];  // Not enough bytes for a UUID
        let mut iter = short_bytes.into_iter();
        let decoded = Uuid::try_decode(&mut iter);
        assert!(matches!(decoded, Err(AppError::IteratorEmptyOrTooShortError)));
    }

    #[test]
    fn test_decode_empty_iterator() {
        let mut iter = std::iter::empty();
        let decoded = Uuid::try_decode(&mut iter);
        assert!(matches!(decoded, Err(AppError::IteratorEmptyOrTooShortError)));
    }
}
