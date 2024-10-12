use std::pin::Pin;
use tokio_stream::{Stream, StreamExt};
use crate::common::read_bytes_16;
use crate::constants::constructors::UUID;
use crate::error::AppError;
use crate::serde::decode::Decode;
use crate::serde::encode::{Encode, Encoded};


#[derive(Hash, Eq, PartialEq)]
pub struct Uuid(uuid::Uuid);

impl Encode for Uuid {
    fn encode(&self) -> Encoded {
        Encoded::new_fixed(UUID, self.0.into_bytes().to_vec())
    }
}

impl Decode for Uuid {
    async fn can_decode(iter: Pin<Box<impl Stream<Item=u8>>>) -> bool {
        match iter.peekable().peek().await {
            Some(&UUID) => true,
            _ => false,
        }
    }

    async fn try_decode(mut iter: Pin<Box<impl Stream<Item=u8>>>) -> Result<Self, AppError> where Self: Sized {
        match iter.next().await {
            Some(UUID) => Ok(parse_uuid(&mut iter).await?),
            Some(c) => Err(AppError::DeserializationIllegalConstructorError(c)),
            None => Err(AppError::IteratorEmptyOrTooShortError),
        }
    }
}

async fn parse_uuid(iter: &mut Pin<Box<impl Stream<Item=u8>>>) -> Result<Uuid, AppError> {
    let byte_vals = read_bytes_16(iter).await?;
    Ok(Uuid(uuid::Uuid::from_bytes(byte_vals)))
}

#[cfg(test)]
mod test {
    use crate::common::tests::ByteVecExt;
    use crate::constants::constructors::UUID;
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
        expected_bytes.push(UUID);
        expected_bytes.extend_from_slice(&uuid.0.into_bytes());

        assert_eq!(encoded.to_bytes(), expected_bytes);
    }

    #[tokio::test]
    async fn test_decode_success() {
        let uuid = uuid::Uuid::new_v4();
        let mut bytes = vec![UUID];
        bytes.extend(uuid.into_bytes().to_vec());
        let decoded = Uuid::try_decode(bytes.into_pinned_stream()).await;
        assert!(decoded.is_ok());
        assert_eq!(decoded.unwrap().0, uuid)
    }

    #[tokio::test]
    async fn test_decode_incorrect_constructor() {
        let uuid = uuid::Uuid::new_v4().into_bytes();
        let mut bytes = vec![0x99];
        bytes.extend(uuid.to_vec());
        let decoded = Uuid::try_decode(bytes.into_pinned_stream()).await;
        assert!(matches!(decoded, Err(AppError::DeserializationIllegalConstructorError(_))));
    }

    #[tokio::test]
    async fn test_decode_short_byte_sequence() {
        let short_bytes = vec![UUID];  // Not enough bytes for a UUID
        let decoded = Uuid::try_decode(short_bytes.into_pinned_stream()).await;
        assert!(matches!(decoded, Err(AppError::IteratorEmptyOrTooShortError)));
    }

    #[tokio::test]
    async fn test_decode_empty_iterator() {
        let val = vec![];
        let decoded = Uuid::try_decode(val.into_pinned_stream()).await;
        assert!(matches!(decoded, Err(AppError::IteratorEmptyOrTooShortError)));
    }
}
