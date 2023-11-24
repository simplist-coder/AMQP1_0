use crate::common::read_bytes_8;
use crate::error::AppError;
use crate::serde::decode::Decode;
use crate::serde::encode::{Encode, Encoded};

const DEFAULT_CONSTR: u8 = 0x83;

#[derive(Hash, Eq, PartialEq)]
pub struct Timestamp(i64);

impl Encode for Timestamp {
    fn encode(&self) -> Encoded {
        Encoded::new_fixed(DEFAULT_CONSTR, self.0.to_be_bytes().to_vec())
    }
}

impl Decode for Timestamp {
    fn can_decode(iter: impl Iterator<Item=u8>) -> bool {
        match iter.peekable().peek() {
            Some(&DEFAULT_CONSTR) => true,
            _ => false,
        }
    }

    fn try_decode(mut iter: impl Iterator<Item=u8>) -> Result<Self, AppError> where Self: Sized {
        match iter.next() {
            Some(DEFAULT_CONSTR) => Ok(parse_timestamp(&mut iter)?),
            Some(c) => Err(AppError::DeserializationIllegalConstructorError(c)),
            None => Err(AppError::IteratorEmptyOrTooShortError),
        }
    }
}

fn parse_timestamp(iter: &mut impl Iterator<Item=u8>) -> Result<Timestamp, AppError> {
    let byte_vals = read_bytes_8(iter)?;
    Ok(Timestamp(i64::from_be_bytes(byte_vals)))
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn construct_timestamp() {
        let val = Timestamp(1);
        assert_eq!(val.encode().constructor(), 0x83);
    }

    #[test]
    fn test_successful_timestamp_encoding() {
        // Example Unix time in milliseconds: 2011-07-26T18:21:03.521Z
        let example_unix_time_ms: i64 = 1311704463521;
        let timestamp = Timestamp(example_unix_time_ms);

        let encoded = timestamp.encode();
        let expected_bytes = [DEFAULT_CONSTR].into_iter()
            .chain(example_unix_time_ms.to_be_bytes())
            .collect::<Vec<_>>();

        assert_eq!(encoded.to_bytes(), expected_bytes.as_slice());
    }

    #[test]
    fn test_timestamp_decoding() {
        // Example Unix time in milliseconds: 2011-07-26T18:21:03.521Z
        let example_unix_time_ms: i64 = 1311704463521;
        let mut data = vec![DEFAULT_CONSTR];
        data.extend_from_slice(&example_unix_time_ms.to_be_bytes());

        match Timestamp::try_decode(data.into_iter()) {
            Ok(timestamp) => assert_eq!(timestamp.0, example_unix_time_ms),
            Err(e) => panic!("Unexpected error: {:?}", e),
        }
    }

    #[test]
    fn test_illegal_constructor_timestamp_decoding() {
        let illegal_constructor = 0xFF;
        let bytes = vec![illegal_constructor];

        match Timestamp::try_decode(&mut bytes.into_iter()) {
            Ok(_) => panic!("Expected an error, but deserialization succeeded"),
            Err(AppError::DeserializationIllegalConstructorError(c)) => assert_eq!(illegal_constructor, c),
            Err(e) => panic!("Unexpected error type: {:?}", e),
        }
    }

    #[test]
    fn test_incomplete_iterator_timestamp_decoding() {
        let data = vec![DEFAULT_CONSTR]; // Missing the 8 bytes for the timestamp

        match Timestamp::try_decode(&mut data.into_iter()) {
            Ok(_) => panic!("Expected an error, but deserialization succeeded"),
            Err(AppError::IteratorEmptyOrTooShortError) => (), // Expected outcome
            Err(e) => panic!("Unexpected error type: {:?}", e),
        }
    }
}
