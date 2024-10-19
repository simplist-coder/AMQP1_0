use crate::constants::constructors::UUID;
use crate::serde::decode::Decode;
use crate::serde::encode::{Encode, Encoded};
use amqp_error::AppError;
use amqp_utils::read_bytes_16;
use std::pin::Pin;
use tokio_stream::Stream;

#[derive(Debug, Hash, Eq, PartialEq, Clone)]
pub struct Uuid(uuid::Uuid);

impl Encode for Uuid {
    fn encode(self) -> Encoded {
        Encoded::new_fixed(UUID, self.0.into_bytes().to_vec())
    }
}

impl Decode for Uuid {
    async fn try_decode(
        constructor: u8,
        stream: &mut Pin<Box<impl Stream<Item = u8>>>,
    ) -> Result<Self, AppError>
    where
        Self: Sized,
    {
        match constructor {
            UUID => Ok(parse_uuid(stream).await?),
            c => Err(AppError::DeserializationIllegalConstructorError(c)),
        }
    }
}

async fn parse_uuid(iter: &mut Pin<Box<impl Stream<Item = u8>>>) -> Result<Uuid, AppError> {
    let byte_vals = read_bytes_16(iter).await?;
    Ok(Uuid(uuid::Uuid::from_bytes(byte_vals)))
}

impl From<uuid::Uuid> for Uuid {
    fn from(value: uuid::Uuid) -> Self {
        Uuid(value)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use amqp_utils::ByteVecExt;

    use crate::constants::constructors::UUID;

    #[test]
    fn construct_uuid() {
        let val = Uuid(uuid::Uuid::new_v4());
        assert_eq!(val.encode().constructor(), 0x98);
    }

    #[test]
    fn test_encode_correctness() {
        let uuid = Uuid(uuid::Uuid::new_v4());
        let encoded = uuid.clone().encode();
        let mut expected_bytes = Vec::new();
        expected_bytes.push(UUID);
        expected_bytes.extend_from_slice(&uuid.0.into_bytes());

        assert_eq!(encoded.into_bytes(), expected_bytes);
    }

    #[tokio::test]
    async fn test_decode_success() {
        let uuid = uuid::Uuid::new_v4();
        let mut bytes = vec![];
        bytes.extend(uuid.into_bytes().to_vec());
        let decoded = Uuid::try_decode(UUID, &mut bytes.into_pinned_stream()).await;
        assert!(decoded.is_ok());
        assert_eq!(decoded.unwrap().0, uuid);
    }

    #[tokio::test]
    async fn test_decode_incorrect_constructor() {
        let uuid = uuid::Uuid::new_v4().into_bytes();
        let mut bytes = vec![];
        bytes.extend(uuid.to_vec());
        let decoded = Uuid::try_decode(0x99, &mut bytes.into_pinned_stream()).await;
        assert!(matches!(
            decoded,
            Err(AppError::DeserializationIllegalConstructorError(_))
        ));
    }

    #[tokio::test]
    async fn test_decode_short_byte_sequence() {
        let short_bytes = vec![UUID]; // Not enough bytes for a UUID
        let decoded = Uuid::try_decode(UUID, &mut short_bytes.into_pinned_stream()).await;
        assert!(matches!(
            decoded,
            Err(AppError::IteratorEmptyOrTooShortError)
        ));
    }

    #[tokio::test]
    async fn test_decode_empty_iterator() {
        let val = vec![];
        let decoded = Uuid::try_decode(UUID, &mut val.into_pinned_stream()).await;
        assert!(matches!(
            decoded,
            Err(AppError::IteratorEmptyOrTooShortError)
        ));
    }
}
