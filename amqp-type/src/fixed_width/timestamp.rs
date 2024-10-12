use std::pin::Pin;
use tokio_stream::{Stream, StreamExt};
use crate::common::read_bytes_8;
use crate::constants::constructors::TIMESTAMP;
use crate::error::AppError;
use crate::serde::decode::Decode;
use crate::serde::encode::{Encode, Encoded};



#[derive(Hash, Eq, PartialEq)]
pub struct Timestamp(i64);

impl Encode for Timestamp {
    fn encode(&self) -> Encoded {
        Encoded::new_fixed(TIMESTAMP, self.0.to_be_bytes().to_vec())
    }
}

impl Decode for Timestamp {
    async fn can_decode(iter: Pin<Box<impl Stream<Item=u8>>>) -> bool {
        match iter.peekable().peek().await {
            Some(&TIMESTAMP) => true,
            _ => false,
        }
    }

    async fn try_decode(mut iter: Pin<Box<impl Stream<Item=u8>>>) -> Result<Self, AppError> where Self: Sized {
        match iter.next().await {
            Some(TIMESTAMP) => Ok(parse_timestamp(&mut iter).await?),
            Some(c) => Err(AppError::DeserializationIllegalConstructorError(c)),
            None => Err(AppError::IteratorEmptyOrTooShortError),
        }
    }
}

async fn parse_timestamp(iter: &mut Pin<Box<impl Stream<Item=u8>>>) -> Result<Timestamp, AppError> {
    let byte_vals = read_bytes_8(iter).await?;
    Ok(Timestamp(i64::from_be_bytes(byte_vals)))
}

#[cfg(test)]
mod test {
    use crate::common::tests::ByteVecExt;
    use crate::constants::constructors::TIMESTAMP;
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
        let expected_bytes = [TIMESTAMP].into_iter()
            .chain(example_unix_time_ms.to_be_bytes())
            .collect::<Vec<_>>();

        assert_eq!(encoded.to_bytes(), expected_bytes.as_slice());
    }

    #[tokio::test]
    async fn test_timestamp_decoding() {
        // Example Unix time in milliseconds: 2011-07-26T18:21:03.521Z
        let example_unix_time_ms: i64 = 1311704463521;
        let mut data = vec![TIMESTAMP];
        data.extend_from_slice(&example_unix_time_ms.to_be_bytes());

        match Timestamp::try_decode(data.into_pinned_stream()).await {
            Ok(timestamp) => assert_eq!(timestamp.0, example_unix_time_ms),
            Err(e) => panic!("Unexpected error: {:?}", e),
        }
    }

    #[tokio::test]
    async fn test_illegal_constructor_timestamp_decoding() {
        let illegal_constructor = 0xFF;
        let bytes = vec![illegal_constructor];

        match Timestamp::try_decode(bytes.into_pinned_stream()).await {
            Ok(_) => panic!("Expected an error, but deserialization succeeded"),
            Err(AppError::DeserializationIllegalConstructorError(c)) => assert_eq!(illegal_constructor, c),
            Err(e) => panic!("Unexpected error type: {:?}", e),
        }
    }

    #[tokio::test]
    async fn test_incomplete_iterator_timestamp_decoding() {
        let data = vec![TIMESTAMP]; // Missing the 8 bytes for the timestamp

        match Timestamp::try_decode(data.into_pinned_stream()).await {
            Ok(_) => panic!("Expected an error, but deserialization succeeded"),
            Err(AppError::IteratorEmptyOrTooShortError) => (), // Expected outcome
            Err(e) => panic!("Unexpected error type: {:?}", e),
        }
    }
}
