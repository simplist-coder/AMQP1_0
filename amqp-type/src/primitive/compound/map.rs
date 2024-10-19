use crate::constants::{MAP, MAP_SHORT};
use crate::primitive::compound::encoded_vec::EncodedVec;
use crate::primitive::Primitive;
use crate::serde::decode::Decode;
use crate::serde::encode::{Encode, Encoded};
use amqp_error::AppError;
use amqp_utils::sync_util::{read_bytes, read_bytes_4};
use indexmap::IndexMap;
use std::hash::Hash;
use std::vec::IntoIter;

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Map(IndexMap<Primitive, Primitive>);

impl Encode for Map {
    fn encode(self) -> Encoded {
        let mut res: Vec<Encoded> = Vec::new();
        let mut count = 0;
        for (key, value) in self.0 {
            let k = key.encode();
            let v = value.encode();
            res.push(k);
            res.push(v);
            count += 2;
        }
        let encoded: Vec<u8> = EncodedVec::new(res).into();
        match encoded.len() {
            x if x <= 255 => Encoded::new_compound(MAP_SHORT, count, encoded),
            _ => Encoded::new_compound(MAP, count, encoded),
        }
    }
}

impl Decode for Map {
    fn try_decode(constructor: u8, stream: &mut IntoIter<u8>) -> Result<Self, AppError>
    where
        Self: Sized,
    {
        match constructor {
            MAP_SHORT => Ok(parse_short_map(stream)?),
            MAP => Ok(parse_map(stream)?),
            illegal => Err(AppError::DeserializationIllegalConstructorError(illegal)),
        }
    }
}

fn parse_short_map(stream: &mut IntoIter<u8>) -> Result<Map, AppError> {
    let size = stream
        .next()
        .ok_or(AppError::IteratorEmptyOrTooShortError)?;
    let count = stream
        .next()
        .ok_or(AppError::IteratorEmptyOrTooShortError)?;
    Ok(Map(parse_to_index_map(
        stream,
        size as usize,
        count as usize,
    )?))
}

fn parse_map(stream: &mut IntoIter<u8>) -> Result<Map, AppError> {
    let size = u32::from_be_bytes(read_bytes_4(stream)?);
    let count = u32::from_be_bytes(read_bytes_4(stream)?);
    Ok(Map(parse_to_index_map(
        stream,
        size as usize,
        count as usize,
    )?))
}

fn parse_to_index_map(
    stream: &mut IntoIter<u8>,
    size: usize,
    count: usize,
) -> Result<IndexMap<Primitive, Primitive>, AppError> {
    if count % 2 != 0 {
        return Err(AppError::DeserializationMapContainsOddAmountOfElementsError);
    }
    let mut buffer = read_bytes(stream, size)?.into_iter();
    let mut result = IndexMap::with_capacity(count);
    for _ in 0..count / 2 {
        let key = Primitive::try_decode(&mut buffer)?;
        let value = Primitive::try_decode(&mut buffer)?;
        result.insert(key, value);
    }
    Ok(result)
}

impl Hash for Map {
    fn hash<H: std::hash::Hasher>(&self, _state: &mut H) {
        todo!()
    }
}

impl From<IndexMap<Primitive, Primitive>> for Map {
    fn from(value: IndexMap<Primitive, Primitive>) -> Self {
        Map(value)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    use crate::constants::{INTEGER, MAP, MAP_SHORT, UNSIGNED_SHORT};

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

    #[test]
    fn try_decode_short_map_returns_correct_value() {
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
        let res = Map::try_decode(MAP_SHORT, &mut bytes.into_iter()).unwrap();
        assert_eq!(res.0.len(), 1);
        assert!(matches!(&res.0[&Primitive::Int(21)], Primitive::Ushort(16)));
    }

    #[test]
    fn try_decode_map_returns_correct_value() {
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
        let res = Map::try_decode(MAP, &mut bytes.into_iter()).unwrap();
        assert_eq!(res.0.len(), 1);
        assert!(matches!(&res.0[&Primitive::Int(21)], Primitive::Ushort(16)));
    }

    #[test]
    fn try_decode_short_map_returns_error_if_constructor_is_wrong() {
        let bytes = vec![5, 1, INTEGER, 0x00, 0x00, 0x00, 21];
        let res = Map::try_decode(ILLEGAL_ELEMENT_CONSTRUCTOR, &mut bytes.into_iter());
        assert!(matches!(
            res,
            Err(AppError::DeserializationIllegalConstructorError(
                ILLEGAL_ELEMENT_CONSTRUCTOR
            ))
        ));
    }

    #[test]
    fn try_decode_short_map_returns_error_if_element_constructor_is_wrong() {
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
        let res = Map::try_decode(MAP_SHORT, &mut bytes.into_iter());
        assert!(matches!(
            res,
            Err(AppError::DeserializationIllegalConstructorError(
                ILLEGAL_ELEMENT_CONSTRUCTOR
            ))
        ));
    }

    #[test]
    fn try_decode_map_returns_error_if_constructor_is_wrong() {
        let bytes = vec![
            0x00, 0x00, 0x00, 4, 0x00, 0x00, 0x00, 1, INTEGER, 0x00, 0x00, 0x00, 0x15,
        ];
        let res = Map::try_decode(ILLEGAL_ELEMENT_CONSTRUCTOR, &mut bytes.into_iter());
        assert!(matches!(
            res,
            Err(AppError::DeserializationIllegalConstructorError(
                ILLEGAL_ELEMENT_CONSTRUCTOR
            ))
        ));
    }

    #[test]
    fn try_decode_map_returns_error_if_element_constructor_is_wrong() {
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
        let res = Map::try_decode(MAP, &mut bytes.into_iter());
        assert!(matches!(
            res,
            Err(AppError::DeserializationIllegalConstructorError(
                ILLEGAL_ELEMENT_CONSTRUCTOR
            ))
        ));
    }

    #[test]
    fn try_decode_short_map_returns_error_number_of_elements_is_odd() {
        let bytes = vec![5, 1, INTEGER, 0x00, 0x00, 0x00, 21];
        let res = Map::try_decode(MAP_SHORT, &mut bytes.into_iter());
        assert!(matches!(
            res,
            Err(AppError::DeserializationMapContainsOddAmountOfElementsError)
        ));
    }

    #[test]
    fn try_decode_map_returns_error_number_of_elements_is_odd() {
        let bytes = vec![
            0x00, 0x00, 0x00, 5, 0x00, 0x00, 0x00, 1, INTEGER, 0x00, 0x00, 0x00, 21,
        ];
        let res = Map::try_decode(MAP, &mut bytes.into_iter());
        assert!(matches!(
            res,
            Err(AppError::DeserializationMapContainsOddAmountOfElementsError)
        ));
    }
}
