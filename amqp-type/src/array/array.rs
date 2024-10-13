use crate::amqp_type::AmqpType;
use crate::common::{read_bytes, read_bytes_4};
use crate::constants::constructors::{ARRAY, ARRAY_SHORT};
use crate::error::AppError;
use crate::serde::decode::Decode;
use crate::serde::encode::{Encode, Encoded};
use std::pin::Pin;
use tokio_stream::{iter, Stream, StreamExt};

#[derive(Debug, Hash, Eq, PartialEq)]
pub struct Array(Vec<AmqpType>);

impl Encode for Array {
    fn encode(&self) -> Encoded {
        let encoded: Vec<Encoded> = self.0.iter().map(|x| x.encode()).collect();
        let byte_size = encoded.iter().fold(0, |acc, x| acc + x.data_len());
        match (encoded.len(), byte_size) {
            (len, size) if len <= 255 && size < 256 => ARRAY_SHORT.into(),
            (_, _) => ARRAY.into(),
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
    use crate::constants::constructors::INTEGER;

    #[test]
    fn construct_array_with_less_than_255_elements() {
        let val = Array(Vec::new());
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
