use crate::constants::constructors::TIMESTAMP;
use crate::serde::decode::Decode;
use crate::serde::encode::{Encode, Encoded};
use amqp_error::AppError;
use amqp_utils::sync_util::read_bytes_8;
use std::vec::IntoIter;

#[derive(Debug, Hash, Eq, PartialEq, Clone)]
pub struct Timestamp(i64);

impl Encode for Timestamp {
    fn encode(self) -> Encoded {
        Encoded::new_fixed(TIMESTAMP, self.0.to_be_bytes().to_vec())
    }
}

impl Decode for Timestamp {
    fn try_decode(constructor: u8, stream: &mut IntoIter<u8>) -> Result<Self, AppError>
    where
        Self: Sized,
    {
        match constructor {
            TIMESTAMP => Ok(parse_timestamp(stream)?),
            c => Err(AppError::DeserializationIllegalConstructorError(c)),
        }
    }
}

fn parse_timestamp(iter: &mut IntoIter<u8>) -> Result<Timestamp, AppError> {
    let byte_vals = read_bytes_8(iter)?;
    Ok(Timestamp(i64::from_be_bytes(byte_vals)))
}

impl From<i64> for Timestamp {
    fn from(value: i64) -> Self {
        Timestamp(value)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    use crate::constants::constructors::TIMESTAMP;

    #[test]
    fn construct_timestamp() {
        let val = Timestamp(1);
        assert_eq!(val.encode().constructor(), 0x83);
    }

    #[test]
    fn test_successful_timestamp_encoding() {
        // Example Unix time in milliseconds: 2011-07-26T18:21:03.521Z
        let example_unix_time_ms: i64 = 1_311_704_463_521;
        let timestamp = Timestamp(example_unix_time_ms);

        let encoded = timestamp.encode();
        let expected_bytes = [TIMESTAMP]
            .into_iter()
            .chain(example_unix_time_ms.to_be_bytes())
            .collect::<Vec<_>>();

        assert_eq!(encoded.into_bytes(), expected_bytes.as_slice());
    }

    #[test]
    fn test_timestamp_decoding() {
        // Example Unix time in milliseconds: 2011-07-26T18:21:03.521Z
        let example_unix_time_ms: i64 = 1_311_704_463_521;
        let mut data = vec![];
        data.extend_from_slice(&example_unix_time_ms.to_be_bytes());

        match Timestamp::try_decode(TIMESTAMP, &mut data.into_iter()) {
            Ok(timestamp) => assert_eq!(timestamp.0, example_unix_time_ms),
            Err(e) => panic!("Unexpected error: {e:?}"),
        }
    }

    #[test]
    fn test_illegal_constructor_timestamp_decoding() {
        let illegal_constructor = 0xFF;
        let bytes = vec![];

        match Timestamp::try_decode(illegal_constructor, &mut bytes.into_iter()) {
            Ok(_) => panic!("Expected an error, but deserialization succeeded"),
            Err(AppError::DeserializationIllegalConstructorError(c)) => {
                assert_eq!(illegal_constructor, c);
            }
            Err(e) => panic!("Unexpected error type: {e:?}"),
        }
    }

    #[test]
    fn test_incomplete_iterator_timestamp_decoding() {
        let data = vec![TIMESTAMP]; // Missing the 8 bytes for the timestamp

        match Timestamp::try_decode(TIMESTAMP, &mut data.into_iter()) {
            Ok(_) => panic!("Expected an error, but deserialization succeeded"),
            Err(AppError::IteratorEmptyOrTooShortError) => (), // Expected outcome
            Err(e) => panic!("Unexpected error type: {e:?}"),
        }
    }
}
