use crate::common::{read_bytes, read_bytes_4};
use crate::constants::constructors::{LIST, LIST_EMPTY, LIST_SHORT};
use crate::error::AppError;
use crate::primitive::compound::encoded_vec::EncodedVec;
use crate::primitive::primitive::Primitive;
use crate::serde::decode::Decode;
use crate::serde::encode::{Encode, Encoded};
use std::pin::Pin;
use tokio_stream::{iter, Stream, StreamExt};

#[derive(Debug, Hash, Eq, PartialEq)]
pub struct List(Vec<Primitive>);

impl Encode for List {
    fn encode(&self) -> Encoded {
        let count = self.0.len();
        let encoded_elements = self.0.iter().map(|x| x.encode()).collect();
        let encoded = EncodedVec::new(encoded_elements).serialize();
        match (count, encoded.len()) {
            (0, _) => LIST_EMPTY.into(),
            (len, size) if len <= 255 && size < 256 => {
                Encoded::new_compound(LIST_SHORT, count, encoded)
            }
            (_, _) => Encoded::new_compound(LIST, count, encoded),
        }
    }
}

impl Decode for List {
    async fn try_decode(
        constructor: u8,
        stream: &mut Pin<Box<impl Stream<Item = u8>>>,
    ) -> Result<Self, AppError>
    where
        Self: Sized,
    {
        match constructor {
            LIST_EMPTY => Ok(List(vec![])),
            LIST_SHORT => Ok(parse_short_list(stream).await?),
            LIST => Ok(parse_list(stream).await?),
            illegal => Err(AppError::DeserializationIllegalConstructorError(illegal)),
        }
    }
}

async fn parse_short_list(stream: &mut Pin<Box<impl Stream<Item = u8>>>) -> Result<List, AppError> {
    let size = stream
        .next()
        .await
        .ok_or(AppError::IteratorEmptyOrTooShortError)?;
    let count = stream
        .next()
        .await
        .ok_or(AppError::IteratorEmptyOrTooShortError)?;
    Ok(List(
        parse_list_to_vec(stream, size as usize, count as usize).await?,
    ))
}

async fn parse_list(stream: &mut Pin<Box<impl Stream<Item = u8>>>) -> Result<List, AppError> {
    let size = u32::from_be_bytes(read_bytes_4(stream).await?);
    let count = u32::from_be_bytes(read_bytes_4(stream).await?);
    Ok(List(
        parse_list_to_vec(stream, size as usize, count as usize).await?,
    ))
}

async fn parse_list_to_vec(
    stream: &mut Pin<Box<impl Stream<Item = u8>>>,
    size: usize,
    count: usize,
) -> Result<Vec<Primitive>, AppError> {
    let vec = read_bytes(stream, size).await?;
    let mut buffer = Box::pin(iter(vec));
    let mut result = Vec::with_capacity(count);
    for _ in 0..count {
        let decoded = Box::pin(Primitive::try_decode(&mut buffer)).await?;
        result.push(decoded);
    }

    Ok(result)
}

impl From<Vec<Primitive>> for List {
    fn from(value: Vec<Primitive>) -> Self {
        List(value)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::common::tests::ByteVecExt;
    use crate::constants::constructors::{INTEGER, UNSIGNED_SHORT};
    #[test]
    fn construct_empty_list() {
        let val = List(vec![]);
        assert_eq!(val.encode().constructor(), 0x45);
    }

    #[test]
    fn construct_list_with_less_than_255_elements() {
        let val = List(vec![1.into()]);
        assert_eq!(val.encode().constructor(), 0xc0);
    }

    #[test]
    fn construct_list_with_more_than_255_elements() {
        let mut arr = vec![];
        for i in 0..500 {
            arr.push(i.into())
        }
        let val = List(arr);
        assert_eq!(val.encode().constructor(), 0xd0);
    }

    #[test]
    fn construct_list_with_less_than_255_elements_and_larger_than_255_bytes() {
        let mut arr = vec![];
        for _ in 0..100 {
            arr.push("aaaaaaaaaaaaaaaaaaaa".into());
        }
        let val = List(arr);
        assert_eq!(val.encode().constructor(), 0xd0);
    }

    #[tokio::test]
    async fn try_decode_short_list_returns_correct_value() {
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
        let res = List::try_decode(LIST_SHORT, &mut bytes.into_pinned_stream())
            .await
            .unwrap();
        assert_eq!(res.0.len(), 2);
        assert!(matches!(res.0[0], Primitive::Int(21)));
        assert!(matches!(res.0[1], Primitive::Ushort(16)));
    }

    #[tokio::test]
    async fn try_decode_list_returns_correct_value() {
        let bytes = vec![
            0x00, 0x00, 0x00, 5, 0x00, 0x00, 0x00, 1, INTEGER, 0x00, 0x00, 0x00, 21,
        ];
        let res = List::try_decode(LIST, &mut bytes.into_pinned_stream())
            .await
            .unwrap();
        assert_eq!(res.0.len(), 1);
        assert!(matches!(res.0[0], Primitive::Int(21)));
    }

    #[tokio::test]
    async fn try_decode_short_list_returns_error_if_constructor_is_wrong() {
        let bytes = vec![4, 1, INTEGER, 0x00, 0x00, 0x00, 21];
        let res = List::try_decode(0x99, &mut bytes.into_pinned_stream()).await;
        assert!(matches!(
            res,
            Err(AppError::DeserializationIllegalConstructorError(0x99))
        ));
    }

    #[tokio::test]
    async fn try_decode_short_list_returns_error_if_element_constructor_is_wrong() {
        let bytes = vec![
            4, 1, 0x99, /*<---wrong element constructor*/
            0x00, 0x00, 0x00, 0x15,
        ];
        let res = List::try_decode(LIST_SHORT, &mut bytes.into_pinned_stream()).await;
        assert!(matches!(
            res,
            Err(AppError::DeserializationIllegalConstructorError(0x99))
        ));
    }

    #[tokio::test]
    async fn try_decode_list_returns_error_if_constructor_is_wrong() {
        let bytes = vec![
            0x00, 0x00, 0x00, 4, 0x00, 0x00, 0x00, 1, INTEGER, 0x00, 0x00, 0x00, 0x15,
        ];
        let res = List::try_decode(0x98, &mut bytes.into_pinned_stream()).await;
        assert!(matches!(
            res,
            Err(AppError::DeserializationIllegalConstructorError(0x98))
        ));
    }

    #[tokio::test]
    async fn try_decode_list_returns_error_if_element_constructor_is_wrong() {
        let bytes = vec![
            0x00, 0x00, 0x00, 4, 0x00, 0x00, 0x00, 1, 0x99, /*<---wrong element constructor*/
            0x00, 0x00, 0x00, 0x15,
        ];
        let res = List::try_decode(LIST, &mut bytes.into_pinned_stream()).await;
        assert!(matches!(
            res,
            Err(AppError::DeserializationIllegalConstructorError(0x99))
        ));
    }

    #[tokio::test]
    async fn try_decode_list_returns_empty_list_on_empty_list_constructor() {
        let bytes = vec![0x13, 0x05, 0x01];
        let res = List::try_decode(LIST_EMPTY, &mut bytes.into_pinned_stream())
            .await
            .unwrap();
        assert!(res.0.is_empty());
    }

    #[tokio::test]
    async fn try_decode_empty_list_does_not_advance_stream() {
        let bytes = vec![1, 2, 3];
        let mut stream = bytes.into_pinned_stream();
        let res = List::try_decode(LIST_EMPTY, &mut stream).await.unwrap();
        assert!(res.0.is_empty());
        assert_eq!(stream.next().await, Some(1));
        assert_eq!(stream.next().await, Some(2));
        assert_eq!(stream.next().await, Some(3));
        assert_eq!(stream.next().await, None);
        assert_eq!(stream.next().await, None);
    }
}
