use crate::amqp_type::AmqpType;
use crate::common::{read_bytes, read_bytes_4};
use crate::compound::encoded_vec::EncodedVec;
use crate::constants::constructors::{MAP, MAP_SHORT};
use crate::error::AppError;
use crate::serde::decode::Decode;
use crate::serde::encode::{Encode, Encoded};
use indexmap::IndexMap;
use std::hash::Hash;
use std::pin::Pin;
use tokio_stream::StreamExt;
use tokio_stream::{iter, Stream};

#[derive(Debug, Eq, PartialEq)]
pub struct Map(IndexMap<AmqpType, AmqpType>);

impl Encode for Map {
    fn encode(&self) -> Encoded {
        let mut res: Vec<Encoded> = Vec::new();
        let mut data_len = 0;
        let mut count = 0;
        for (key, value) in &self.0 {
            let k = key.encode();
            let v = value.encode();
            data_len += k.data_len() + v.data_len();
            res.push(k);
            res.push(v);
            count += 2;
        }
        match data_len {
            x if x <= 255 => Encoded::new_compound(MAP_SHORT, count, EncodedVec::new(res).into()),
            _ => Encoded::new_compound(MAP, count, EncodedVec::new(res).into()),
        }
    }
}

impl Decode for Map {
    async fn try_decode(
        constructor: u8,
        stream: &mut Pin<Box<impl Stream<Item = u8>>>,
    ) -> Result<Self, AppError>
    where
        Self: Sized,
    {
        match constructor {
            MAP_SHORT => Ok(parse_short_map(stream).await?),
            MAP => Ok(parse_map(stream).await?),
            illegal => Err(AppError::DeserializationIllegalConstructorError(illegal)),
        }
    }
}

async fn parse_short_map(stream: &mut Pin<Box<impl Stream<Item = u8>>>) -> Result<Map, AppError> {
    let size = stream
        .next()
        .await
        .ok_or(AppError::IteratorEmptyOrTooShortError)?;
    let count = stream
        .next()
        .await
        .ok_or(AppError::IteratorEmptyOrTooShortError)?;
    Ok(Map(parse_to_index_map(
        stream,
        size as usize,
        count as usize,
    )
    .await?))
}

async fn parse_map(stream: &mut Pin<Box<impl Stream<Item = u8>>>) -> Result<Map, AppError> {
    let size = u32::from_be_bytes(read_bytes_4(stream).await?);
    let count = u32::from_be_bytes(read_bytes_4(stream).await?);
    Ok(Map(parse_to_index_map(
        stream,
        size as usize,
        count as usize,
    )
    .await?))
}

async fn parse_to_index_map(
    stream: &mut Pin<Box<impl Stream<Item = u8>>>,
    size: usize,
    count: usize,
) -> Result<IndexMap<AmqpType, AmqpType>, AppError> {
    if count % 2 != 0 {
        return Err(AppError::DeserializationMapContainsOddAmountOfElementsError);
    }
    let mut buffer = Box::pin(iter(read_bytes(stream, size).await?));
    let mut result = IndexMap::with_capacity(count);
    for _ in 0..count / 2 {
        let key = Box::pin(AmqpType::try_decode(&mut buffer)).await?;
        let value = Box::pin(AmqpType::try_decode(&mut buffer)).await?;
        result.insert(key, value);
    }
    Ok(result)
}

impl Hash for Map {
    fn hash<H: std::hash::Hasher>(&self, _state: &mut H) {
        todo!()
    }
}

impl From<IndexMap<AmqpType, AmqpType>> for Map {
    fn from(value: IndexMap<AmqpType, AmqpType>) -> Self {
        Map(value)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::common::tests::ByteVecExt;
    use crate::constants::constructors::{INTEGER, MAP, MAP_SHORT, UNSIGNED_SHORT};

    const ILLEGAL_ELEMENT_CONSTRUCTOR: u8 = 0x99;

    #[test]
    fn construct_map_with_less_than_255_elements() {
        let val = Map(IndexMap::new());
        assert_eq!(val.encode().constructor(), 0xc1);
    }

    #[test]
    fn construct_map_with_less_more_255_elements() {
        let mut map = IndexMap::new();
        for i in 1..500 {
            map.insert(i.into(), i.into());
        }
        let val = Map(map);
        assert_eq!(val.encode().constructor(), 0xd1);
    }

    #[tokio::test]
    async fn try_decode_short_map_returns_correct_value() {
        let bytes = vec![
            8,
            2,
            INTEGER,
            0x00,
            0x00,
            0x00,
            21,
            UNSIGNED_SHORT,
            0x00,
            16,
        ];
        let res = Map::try_decode(MAP_SHORT, &mut bytes.into_pinned_stream())
            .await
            .unwrap();
        assert_eq!(res.0.len(), 1);
        assert!(matches!(&res.0[&AmqpType::Int(21)], AmqpType::Ushort(16)));
    }

    #[tokio::test]
    async fn try_decode_map_returns_correct_value() {
        let bytes = vec![
            0x00,
            0x00,
            0x00,
            8,
            0x00,
            0x00,
            0x00,
            2,
            INTEGER,
            0x00,
            0x00,
            0x00,
            21,
            UNSIGNED_SHORT,
            0x00,
            16,
        ];
        let res = Map::try_decode(MAP, &mut bytes.into_pinned_stream())
            .await
            .unwrap();
        assert_eq!(res.0.len(), 1);
        assert!(matches!(&res.0[&AmqpType::Int(21)], AmqpType::Ushort(16)));
    }

    #[tokio::test]
    async fn try_decode_short_map_returns_error_if_constructor_is_wrong() {
        let bytes = vec![5, 1, INTEGER, 0x00, 0x00, 0x00, 21];
        let res =
            Map::try_decode(ILLEGAL_ELEMENT_CONSTRUCTOR, &mut bytes.into_pinned_stream()).await;
        assert!(matches!(
            res,
            Err(AppError::DeserializationIllegalConstructorError(
                ILLEGAL_ELEMENT_CONSTRUCTOR
            ))
        ));
    }

    #[tokio::test]
    async fn try_decode_short_map_returns_error_if_element_constructor_is_wrong() {
        let bytes = vec![
            8,
            2,
            ILLEGAL_ELEMENT_CONSTRUCTOR,
            0x00,
            0x00,
            0x00,
            21,
            UNSIGNED_SHORT,
            0x00,
            16,
        ];
        let res = Map::try_decode(MAP_SHORT, &mut bytes.into_pinned_stream()).await;
        assert!(matches!(
            res,
            Err(AppError::DeserializationIllegalConstructorError(
                ILLEGAL_ELEMENT_CONSTRUCTOR
            ))
        ));
    }

    #[tokio::test]
    async fn try_decode_map_returns_error_if_constructor_is_wrong() {
        let bytes = vec![
            0x00, 0x00, 0x00, 4, 0x00, 0x00, 0x00, 1, INTEGER, 0x00, 0x00, 0x00, 0x15,
        ];
        let res =
            Map::try_decode(ILLEGAL_ELEMENT_CONSTRUCTOR, &mut bytes.into_pinned_stream()).await;
        assert!(matches!(
            res,
            Err(AppError::DeserializationIllegalConstructorError(
                ILLEGAL_ELEMENT_CONSTRUCTOR
            ))
        ));
    }

    #[tokio::test]
    async fn try_decode_map_returns_error_if_element_constructor_is_wrong() {
        let bytes = vec![
            0x00,
            0x00,
            0x00,
            8,
            0x00,
            0x00,
            0x00,
            2,
            ILLEGAL_ELEMENT_CONSTRUCTOR,
            0x00,
            0x00,
            0x00,
            21,
            UNSIGNED_SHORT,
            0x00,
            16,
        ];
        let res = Map::try_decode(MAP, &mut bytes.into_pinned_stream()).await;
        assert!(matches!(
            res,
            Err(AppError::DeserializationIllegalConstructorError(
                ILLEGAL_ELEMENT_CONSTRUCTOR
            ))
        ));
    }

    #[tokio::test]
    async fn try_decode_short_map_returns_error_number_of_elements_is_odd() {
        let bytes = vec![5, 1, INTEGER, 0x00, 0x00, 0x00, 21];
        let res = Map::try_decode(MAP_SHORT, &mut bytes.into_pinned_stream()).await;
        assert!(matches!(
            res,
            Err(AppError::DeserializationMapContainsOddAmountOfElementsError)
        ));
    }

    #[tokio::test]
    async fn try_decode_map_returns_error_number_of_elements_is_odd() {
        let bytes = vec![
            0x00, 0x00, 0x00, 5, 0x00, 0x00, 0x00, 1, INTEGER, 0x00, 0x00, 0x00, 21,
        ];
        let res = Map::try_decode(MAP, &mut bytes.into_pinned_stream()).await;
        assert!(matches!(
            res,
            Err(AppError::DeserializationMapContainsOddAmountOfElementsError)
        ));
    }
}
