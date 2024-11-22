use crate::constants::{LIST, LIST_EMPTY, LIST_SHORT};
use crate::primitive::compound::encoded_vec::EncodedVec;
use crate::primitive::Primitive;
use crate::serde::decode::Decode;
use crate::serde::encode::{Encode, Encoded};
use crate::error::AppError;
use crate::utils::sync_util::{read_bytes, read_bytes_4};
use std::vec::IntoIter;
use crate::error::amqp_error::AmqpError;

#[derive(Debug, Hash, Eq, PartialEq, Clone)]
pub struct List(Vec<Primitive>);

impl Encode for List {
    fn encode(self) -> Encoded {
        let count = self.0.len();
        let encoded_elements = self
            .0
            .into_iter()
            .map(crate::serde::encode::Encode::encode)
            .collect();
        let encoded = EncodedVec::new(encoded_elements).into_bytes();
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
    fn try_decode(constructor: u8, stream: &mut IntoIter<u8>) -> Result<Self, AppError>
    where
        Self: Sized,
    {
        match constructor {
            LIST_EMPTY => Ok(List(vec![])),
            LIST_SHORT => Ok(parse_short_list(stream)?),
            LIST => Ok(parse_list(stream)?),
            _ => Err(AmqpError::DecodeError)?,
        }
    }
}

fn parse_short_list(stream: &mut IntoIter<u8>) -> Result<List, AppError> {
    let size = stream
        .next()
        .ok_or(AmqpError::DecodeError)?;
    let count = stream
        .next()
        .ok_or(AmqpError::DecodeError)?;
    Ok(List(parse_list_to_vec(
        stream,
        size as usize,
        count as usize,
    )?))
}

fn parse_list(stream: &mut IntoIter<u8>) -> Result<List, AppError> {
    let size = u32::from_be_bytes(read_bytes_4(stream)?);
    let count = u32::from_be_bytes(read_bytes_4(stream)?);
    Ok(List(parse_list_to_vec(
        stream,
        size as usize,
        count as usize,
    )?))
}

fn parse_list_to_vec(
    stream: &mut IntoIter<u8>,
    size: usize,
    count: usize,
) -> Result<Vec<Primitive>, AppError> {
    let mut vec = read_bytes(stream, size)?.into_iter();
    let mut result = Vec::with_capacity(count);
    for _ in 0..count {
        let decoded = Primitive::try_decode(&mut vec)?;
        result.push(decoded);
    }

    Ok(result)
}

impl List {
    pub fn inner(&self) -> &[Primitive] {
        &self.0
    }

    pub fn into_inner(self) -> Vec<Primitive> {
        self.0
    }
}

impl From<Vec<Primitive>> for List {
    fn from(value: Vec<Primitive>) -> Self {
        List(value)
    }
}


#[cfg(test)]
mod test {
    use super::*;

    use crate::constants::{INTEGER, UNSIGNED_SHORT};
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
            arr.push(i.into());
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

    #[test]
    fn try_decode_short_list_returns_correct_value() {
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
        let res = List::try_decode(LIST_SHORT, &mut bytes.into_iter()).unwrap();
        assert_eq!(res.0.len(), 2);
        assert!(matches!(res.0[0], Primitive::Int(21)));
        assert!(matches!(res.0[1], Primitive::Ushort(16)));
    }

    #[test]
    fn try_decode_list_returns_correct_value() {
        let bytes = vec![
            0x00, 0x00, 0x00, 5, 0x00, 0x00, 0x00, 1, INTEGER, 0x00, 0x00, 0x00, 21,
        ];
        let res = List::try_decode(LIST, &mut bytes.into_iter()).unwrap();
        assert_eq!(res.0.len(), 1);
        assert!(matches!(res.0[0], Primitive::Int(21)));
    }

    #[test]
    fn try_decode_short_list_returns_error_if_constructor_is_wrong() {
        let bytes = vec![4, 1, INTEGER, 0x00, 0x00, 0x00, 21];
        let res = List::try_decode(0x99, &mut bytes.into_iter());
        assert!(matches!(
            res,
            Err(AppError::Amqp(AmqpError::DecodeError))
        ));
    }

    #[test]
    fn try_decode_short_list_returns_error_if_element_constructor_is_wrong() {
        let bytes = vec![
            4, 1, 0x99, /*<---wrong element constructor*/
            0x00, 0x00, 0x00, 0x15,
        ];
        let res = List::try_decode(LIST_SHORT, &mut bytes.into_iter());
        assert!(matches!(
            res,
            Err(AppError::Amqp(AmqpError::DecodeError))
        ));
    }

    #[test]
    fn try_decode_list_returns_error_if_constructor_is_wrong() {
        let bytes = vec![
            0x00, 0x00, 0x00, 4, 0x00, 0x00, 0x00, 1, INTEGER, 0x00, 0x00, 0x00, 0x15,
        ];
        let res = List::try_decode(0x98, &mut bytes.into_iter());
        assert!(matches!(
            res,
            Err(AppError::Amqp(AmqpError::DecodeError))
        ));
    }

    #[test]
    fn try_decode_list_returns_error_if_element_constructor_is_wrong() {
        let bytes = vec![
            0x00, 0x00, 0x00, 4, 0x00, 0x00, 0x00, 1, 0x99, /*<---wrong element constructor*/
            0x00, 0x00, 0x00, 0x15,
        ];
        let res = List::try_decode(LIST, &mut bytes.into_iter());
        assert!(matches!(
            res,
            Err(AppError::Amqp(AmqpError::DecodeError))
        ));
    }

    #[test]
    fn try_decode_list_returns_empty_list_on_empty_list_constructor() {
        let bytes = vec![0x13, 0x05, 0x01];
        let res = List::try_decode(LIST_EMPTY, &mut bytes.into_iter()).unwrap();
        assert!(res.0.is_empty());
    }

    #[test]
    fn try_decode_empty_list_does_not_advance_stream() {
        let bytes = vec![1, 2, 3];
        let mut stream = bytes.into_iter();
        let res = List::try_decode(LIST_EMPTY, &mut stream).unwrap();
        assert!(res.0.is_empty());
        assert_eq!(stream.next(), Some(1));
        assert_eq!(stream.next(), Some(2));
        assert_eq!(stream.next(), Some(3));
        assert_eq!(stream.next(), None);
        assert_eq!(stream.next(), None);
    }
}
