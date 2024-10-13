use crate::amqp_type::AmqpType;
use crate::common::{read_bytes, read_bytes_4};
use crate::compound::encoded_vec::EncodedVec;
use crate::constants::constructors::{ARRAY, ARRAY_SHORT, NULL};
use crate::error::AppError;
use crate::serde::decode::Decode;
use crate::serde::encode::{Encode, Encoded};
use std::pin::Pin;
use tokio_stream::{iter, Stream, StreamExt};

#[derive(Debug, Hash, Eq, PartialEq)]
pub struct Array(Vec<AmqpType>);

impl Encode for Array {
    fn encode(&self) -> Encoded {
        match self.0.len() {
            0 => Encoded::new_array(ARRAY_SHORT, 0, NULL, vec![]),
            len => {
                let encoded: Vec<Encoded> = self.0.iter().map(|x| x.encode()).collect();
                let byte_size = encoded.iter().fold(0, |acc, x| acc + x.data_len());
                match (len, byte_size) {
                    (len, size) if len <= 255 && size < 256 => Encoded::new_array(
                        ARRAY_SHORT,
                        len as u32,
                        encoded[0].constructor(),
                        EncodedVec::new(encoded).serialize_without_constructors(),
                    ),
                    (_, _) => Encoded::new_array(
                        ARRAY,
                        len as u32,
                        encoded[0].constructor(),
                        EncodedVec::new(encoded).serialize_without_constructors(),
                    ),
                }
            }
        }
    }
}

impl Decode for Array {
    async fn try_decode(
        constructor: u8,
        stream: &mut Pin<Box<impl Stream<Item = u8>>>,
    ) -> Result<Self, AppError>
    where
        Self: Sized,
    {
        match constructor {
            ARRAY_SHORT => Ok(parse_short_array(stream).await?),
            ARRAY => Ok(parse_array(stream).await?),
            illegal => Err(AppError::DeserializationIllegalConstructorError(illegal)),
        }
    }
}

async fn parse_short_array(
    stream: &mut Pin<Box<impl Stream<Item = u8>>>,
) -> Result<Array, AppError> {
    let size = stream
        .next()
        .await
        .ok_or(AppError::IteratorEmptyOrTooShortError)?;
    let count = stream
        .next()
        .await
        .ok_or(AppError::IteratorEmptyOrTooShortError)?;
    let element_constructor = stream
        .next()
        .await
        .ok_or(AppError::IteratorEmptyOrTooShortError)?;
    Ok(Array(
        parse_raw_to_vec(stream, size as usize, count as usize, element_constructor).await?,
    ))
}

async fn parse_array(stream: &mut Pin<Box<impl Stream<Item = u8>>>) -> Result<Array, AppError> {
    let size = u32::from_be_bytes(read_bytes_4(stream).await?);
    let count = u32::from_be_bytes(read_bytes_4(stream).await?);
    let element_constructor = stream
        .next()
        .await
        .ok_or(AppError::IteratorEmptyOrTooShortError)?;
    Ok(Array(
        parse_raw_to_vec(stream, size as usize, count as usize, element_constructor).await?,
    ))
}

async fn parse_raw_to_vec(
    stream: &mut Pin<Box<impl Stream<Item = u8>>>,
    size: usize,
    count: usize,
    element_constructor: u8,
) -> Result<Vec<AmqpType>, AppError> {
    let mut result = Vec::with_capacity(count);
    let mut buffer = Box::pin(iter(read_bytes(stream, size).await?));
    for _ in 0..count {
        let amqp_type = Box::pin(AmqpType::try_decode_with_constructor(
            element_constructor,
            &mut buffer,
        ))
        .await?;
        result.push(amqp_type);
    }
    Ok(result)
}

impl From<Vec<AmqpType>> for Array {
    fn from(value: Vec<AmqpType>) -> Self {
        Array(value)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::common::tests::ByteVecExt;
    use crate::constants::constructors::{INTEGER, UNSIGNED_BYTE};

    #[test]
    fn construct_empty_array() {
        let val = Array(Vec::new());
        assert_eq!(val.encode().constructor(), 0xe0);
    }

    #[test]
    fn construct_array_with_less_than_255_elements() {
        let val = Array(vec![AmqpType::Char('a')]);
        assert_eq!(val.encode().constructor(), 0xe0);
    }

    #[test]
    fn construct_array_with_more_than_255_elements() {
        let mut arr = vec![];
        for i in 0..500 {
            arr.push(i.into())
        }
        let val = Array(arr);
        assert_eq!(val.encode().constructor(), 0xf0);
    }

    #[test]
    fn construct_array_with_less_than_255_elements_and_larger_than_255_bytes() {
        let mut arr = vec![];
        for _ in 0..100 {
            arr.push("aaaaaaaaaaaaaaaaaaaa".into());
        }
        let val = Array(arr);
        assert_eq!(val.encode().constructor(), 0xf0);
    }

    #[test]
    fn test_encode_empty_array() {
        // using Vec<u8> because it makes the result easier to reason about
        let array = Array(vec![]);

        let encoded: Vec<u8> = array.encode().into();

        let size = u32::from_be_bytes(encoded.get(0..1).unwrap().try_into().unwrap());
        let count = u32::from_be_bytes(encoded.get(1..2).unwrap().try_into().unwrap());
        let element_constructor = encoded.get(3).unwrap();
        assert_eq!(encoded.len(), 3); // 1 byte size, 1 byte count, 1 byte element constructor, 0 bytes data
        assert_eq!(size, 0);
        assert_eq!(count, 0);
        assert_eq!(element_constructor, &NULL);
    }

    #[test]
    fn test_encode_long_array() {
        // using Vec<u8> because it makes the result easier to reason about
        let raw_data = vec![5u8].repeat(1000);
        let values = raw_data
            .clone()
            .into_iter()
            .map(|x| AmqpType::Ubyte(x))
            .collect();
        let array = Array(values);

        let encoded: Vec<u8> = array.encode().into();

        let size = u32::from_be_bytes(encoded.get(0..4).unwrap().try_into().unwrap());
        let count = u32::from_be_bytes(encoded.get(4..8).unwrap().try_into().unwrap());
        let element_constructor = encoded.get(9).unwrap();
        assert_eq!(encoded.len(), 1009); // 4 bytes size, 4 bytes count, 1 byte element constructor, 1000 bytes data
        assert_eq!(size, 1000);
        assert_eq!(count, 1000);
        assert_eq!(element_constructor, &UNSIGNED_BYTE);
        assert!(encoded.ends_with(raw_data.as_slice()));
    }

    #[test]
    fn test_encode_short_array() {
        // using Vec<u8> because it makes the result easier to reason about
        let raw_data = vec![5u8].repeat(100);
        let values = raw_data
            .clone()
            .into_iter()
            .map(|x| AmqpType::Ubyte(x))
            .collect();
        let array = Array(values);

        let encoded: Vec<u8> = array.encode().into();

        let size = u8::from_be_bytes(encoded.get(0..1).unwrap().try_into().unwrap());
        let count = u8::from_be_bytes(encoded.get(1..2).unwrap().try_into().unwrap());
        let element_constructor = encoded.get(3).unwrap();
        assert_eq!(encoded.len(), 103); // 1 byte size, 1 byte count, 1 byte element constructor, 100 bytes data
        assert_eq!(size, 100);
        assert_eq!(count, 100);
        assert_eq!(element_constructor, &UNSIGNED_BYTE);
        assert!(encoded.ends_with(raw_data.as_slice()));
    }

    #[tokio::test]
    async fn try_decode_short_array_returns_correct_value() {
        let bytes = vec![0x04, 0x01, INTEGER, 0x00, 0x00, 0x00, 0x15];
        let res = Array::try_decode(ARRAY_SHORT, &mut bytes.into_pinned_stream())
            .await
            .unwrap();
        assert_eq!(res.0.len(), 1);
        match res.0[0] {
            AmqpType::Int(value) => assert_eq!(value, 21),
            _ => panic!("Could not destructure expected int value"),
        }
    }

    #[tokio::test]
    async fn try_decode_array_returns_correct_value() {
        let bytes = vec![
            0x00, 0x00, 0x00, 0x04, 0x00, 0x00, 0x00, 0x01, INTEGER, 0x00, 0x00, 0x00, 0x15,
        ];
        let res = Array::try_decode(ARRAY, &mut bytes.into_pinned_stream())
            .await
            .unwrap();
        assert_eq!(res.0.len(), 1);
        match res.0[0] {
            AmqpType::Int(value) => assert_eq!(value, 21),
            _ => panic!("Could not destructure expected int value"),
        }
    }

    #[tokio::test]
    async fn try_decode_short_array_returns_error_if_constructor_is_wrong() {
        let bytes = vec![0x04, 0x01, INTEGER, 0x00, 0x00, 0x00, 0x15];
        let res = Array::try_decode(0x99, &mut bytes.into_pinned_stream()).await;
        assert!(matches!(
            res,
            Err(AppError::DeserializationIllegalConstructorError(_))
        ));
    }

    #[tokio::test]
    async fn try_decode_short_array_returns_error_if_element_constructor_is_wrong() {
        let bytes = vec![
            0x04, 0x01, 0x99, /*<---wrong element constructor*/
            0x00, 0x00, 0x00, 0x15,
        ];
        let res = Array::try_decode(ARRAY_SHORT, &mut bytes.into_pinned_stream()).await;
        assert!(matches!(
            res,
            Err(AppError::DeserializationIllegalConstructorError(_))
        ));
    }

    #[tokio::test]
    async fn try_decode_array_returns_error_if_constructor_is_wrong() {
        let bytes = vec![
            0x00, 0x00, 0x00, 0x04, 0x00, 0x00, 0x00, 0x01, INTEGER, 0x00, 0x00, 0x00, 0x15,
        ];
        let res = Array::try_decode(0x99, &mut bytes.into_pinned_stream()).await;
        assert!(matches!(
            res,
            Err(AppError::DeserializationIllegalConstructorError(_))
        ));
    }

    #[tokio::test]
    async fn try_decode_array_returns_error_if_element_constructor_is_wrong() {
        let bytes = vec![
            0x00, 0x00, 0x00, 0x04, 0x00, 0x00, 0x00, 0x01,
            0x99, /*<---wrong element constructor*/
            0x00, 0x00, 0x00, 0x15,
        ];
        let res = Array::try_decode(ARRAY, &mut bytes.into_pinned_stream()).await;
        assert!(matches!(
            res,
            Err(AppError::DeserializationIllegalConstructorError(_))
        ));
    }
}
