use crate::constants::{ARRAY, ARRAY_SHORT, NULL};
use crate::primitive::compound::encoded_vec::EncodedVec;
use crate::primitive::Primitive;
use crate::serde::decode::Decode;
use crate::serde::encode::{Encode, Encoded};
use crate::error::AppError;
use crate::utils::sync_util::{read_bytes, read_bytes_4};
use std::vec::IntoIter;
use crate::error::amqp_error::AmqpError;

#[derive(Debug, Hash, Eq, PartialEq, Clone)]
pub struct Array(Vec<Primitive>);

impl Encode for Array {
    fn encode(self) -> Encoded {
        match self.0.len() {
            0 => Encoded::new_array(ARRAY_SHORT, 0, NULL, vec![]),
            len => {
                let encoded: Vec<Encoded> = self
                    .0
                    .into_iter()
                    .map(crate::serde::encode::Encode::encode)
                    .collect();
                let element_constructor = encoded[0].constructor();
                let bytes = EncodedVec::new(encoded).serialize_without_constructors();
                match (len, bytes.len()) {
                    (len, size) if len <= 255 && size < 256 => {
                        Encoded::new_array(ARRAY_SHORT, len, element_constructor, bytes)
                    }
                    (_, _) => Encoded::new_array(ARRAY, len, element_constructor, bytes),
                }
            }
        }
    }
}

impl Decode for Array {
    fn try_decode(constructor: u8, stream: &mut IntoIter<u8>) -> Result<Self, AppError>
    where
        Self: Sized,
    {
        match constructor {
            ARRAY_SHORT => Ok(parse_short_array(stream)?),
            ARRAY => Ok(parse_array(stream)?),
            _ => Err(AmqpError::DecodeError.into()),
        }
    }
}

fn parse_short_array(stream: &mut IntoIter<u8>) -> Result<Array, AppError> {
    let size = stream
        .next()
        .ok_or(AmqpError::DecodeError)?;
    let count = stream
        .next()
        .ok_or(AmqpError::DecodeError)?;
    let element_constructor = stream
        .next()
        .ok_or(AmqpError::DecodeError)?;
    Ok(Array(parse_raw_to_vec(
        stream,
        size as usize,
        count as usize,
        element_constructor,
    )?))
}

fn parse_array(stream: &mut IntoIter<u8>) -> Result<Array, AppError> {
    let size = u32::from_be_bytes(read_bytes_4(stream)?);
    let count = u32::from_be_bytes(read_bytes_4(stream)?);
    let element_constructor = stream
        .next()
        .ok_or(AmqpError::DecodeError)?;
    Ok(Array(parse_raw_to_vec(
        stream,
        size as usize,
        count as usize,
        element_constructor,
    )?))
}

fn parse_raw_to_vec(
    stream: &mut IntoIter<u8>,
    size: usize,
    count: usize,
    element_constructor: u8,
) -> Result<Vec<Primitive>, AppError> {
    let mut result = Vec::with_capacity(count);
    let mut buffer = read_bytes(stream, size)?.into_iter();
    for _ in 0..count {
        let amqp_type = Primitive::try_decode_with_constructor(element_constructor, &mut buffer)?;
        result.push(amqp_type);
    }
    Ok(result)
}

impl From<Vec<Primitive>> for Array {
    fn from(value: Vec<Primitive>) -> Self {
        Array(value)
    }
}

impl Array {
    pub fn inner(&self) -> &[Primitive] {
        &self.0
    }

    pub fn into_inner(self) -> Vec<Primitive> {
        self.0
    }
}

#[cfg(test)]
mod test {
    use super::*;

    use crate::constants::{INTEGER, UNSIGNED_BYTE};

    #[test]
    fn construct_empty_array() {
        let val = Array(Vec::new());
        assert_eq!(val.encode().constructor(), 0xe0);
    }

    #[test]
    fn construct_array_with_less_than_255_elements() {
        let val = Array(vec![Primitive::Char('a')]);
        assert_eq!(val.encode().constructor(), 0xe0);
    }

    #[test]
    fn construct_array_with_more_than_255_elements() {
        let mut arr = vec![];
        for i in 0..500 {
            arr.push(i.into());
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

        let constructor = *encoded.first().unwrap();
        let size = *encoded.get(1).unwrap();
        let count = *encoded.get(2).unwrap();
        let element_constructor = *encoded.get(3).unwrap();
        assert_eq!(encoded.len(), 4); //1 byte constructor, 1 byte size, 1 byte count, 1 byte element constructor, 0 bytes data
        assert_eq!(constructor, ARRAY_SHORT);
        assert_eq!(size, 0);
        assert_eq!(count, 0);
        assert_eq!(element_constructor, NULL);
    }

    #[test]
    fn test_encode_long_array() {
        // using Vec<u8> because it makes the result easier to reason about
        let raw_data = vec![5; 1000];
        let values = raw_data.clone().into_iter().map(Primitive::Ubyte).collect();
        let array = Array(values);

        let encoded: Vec<u8> = array.encode().into();

        let constructor = *encoded.first().unwrap();
        let size = u32::from_be_bytes(encoded.get(1..5).unwrap().try_into().unwrap());
        let count = u32::from_be_bytes(encoded.get(5..9).unwrap().try_into().unwrap());
        let element_constructor = *encoded.get(9).unwrap();
        assert_eq!(constructor, ARRAY);
        assert_eq!(encoded.len(), 1010); // 1 byte constructor, 4 bytes size, 4 bytes count, 1 byte element constructor, 1000 bytes data
        assert_eq!(size, 1000);
        assert_eq!(count, 1000);
        assert_eq!(element_constructor, UNSIGNED_BYTE);
        assert!(encoded.ends_with(raw_data.as_slice()));
    }

    #[test]
    fn test_encode_short_array() {
        // using Vec<u8> because it makes the result easier to reason about
        let raw_data = vec![5; 100];
        let values = raw_data.clone().into_iter().map(Primitive::Ubyte).collect();
        let array = Array(values);

        let encoded: Vec<u8> = array.encode().into();

        let constructor = *encoded.first().unwrap();
        let size = *encoded.get(1).unwrap();
        let count = *encoded.get(2).unwrap();
        let element_constructor = *encoded.get(3).unwrap();
        assert_eq!(encoded.len(), 104); // 1 byte constructor, 1 byte size, 1 byte count, 1 byte element constructor, 100 bytes data
        assert_eq!(constructor, ARRAY_SHORT);
        assert_eq!(size, 100);
        assert_eq!(count, 100);
        assert_eq!(element_constructor, UNSIGNED_BYTE);
        assert!(encoded.ends_with(raw_data.as_slice()));
    }

    #[test]
    fn try_decode_short_array_returns_correct_value() {
        let bytes = vec![0x04, 0x01, INTEGER, 0x00, 0x00, 0x00, 0x15];
        let res = Array::try_decode(ARRAY_SHORT, &mut bytes.into_iter()).unwrap();
        assert_eq!(res.0.len(), 1);
        match res.0[0] {
            Primitive::Int(value) => assert_eq!(value, 21),
            _ => panic!("Could not destructure expected int value"),
        }
    }

    #[test]
    fn try_decode_array_returns_correct_value() {
        let bytes = vec![
            0x00, 0x00, 0x00, 0x04, 0x00, 0x00, 0x00, 0x01, INTEGER, 0x00, 0x00, 0x00, 0x15,
        ];
        let res = Array::try_decode(ARRAY, &mut bytes.into_iter()).unwrap();
        assert_eq!(res.0.len(), 1);
        match res.0[0] {
            Primitive::Int(value) => assert_eq!(value, 21),
            _ => panic!("Could not destructure expected int value"),
        }
    }

    #[test]
    fn try_decode_short_array_returns_error_if_constructor_is_wrong() {
        let bytes = vec![0x04, 0x01, INTEGER, 0x00, 0x00, 0x00, 0x15];
        let res = Array::try_decode(0x99, &mut bytes.into_iter());
        assert!(matches!(
            res,
            Err(AppError::Amqp(AmqpError::DecodeError))
        ));
    }

    #[test]
    fn try_decode_short_array_returns_error_if_element_constructor_is_wrong() {
        let bytes = vec![
            0x04, 0x01, 0x99, /*<---wrong element constructor*/
            0x00, 0x00, 0x00, 0x15,
        ];
        let res = Array::try_decode(ARRAY_SHORT, &mut bytes.into_iter());
        assert!(matches!(
            res,
            Err(AppError::Amqp(AmqpError::DecodeError))
        ));
    }

    #[test]
    fn try_decode_array_returns_error_if_constructor_is_wrong() {
        let bytes = vec![
            0x00, 0x00, 0x00, 0x04, 0x00, 0x00, 0x00, 0x01, INTEGER, 0x00, 0x00, 0x00, 0x15,
        ];
        let res = Array::try_decode(0x99, &mut bytes.into_iter());
        assert!(matches!(
            res,
            Err(AppError::Amqp(AmqpError::DecodeError))
        ));
    }

    #[test]
    fn try_decode_array_returns_error_if_element_constructor_is_wrong() {
        let bytes = vec![
            0x00, 0x00, 0x00, 0x04, 0x00, 0x00, 0x00, 0x01,
            0x99, /*<---wrong element constructor*/
            0x00, 0x00, 0x00, 0x15,
        ];
        let res = Array::try_decode(ARRAY, &mut bytes.into_iter());
        assert!(matches!(
            res,
            Err(AppError::Amqp(AmqpError::DecodeError))
        ));
    }
}
