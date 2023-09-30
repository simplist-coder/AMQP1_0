use std::hash::Hash;

use indexmap::IndexMap;

use crate::amqp_type::{AmqpType, Encode};

use super::amqp_type::Encoded;

#[derive(Hash, Eq, PartialEq)]
pub struct List(Vec<AmqpType>);

#[derive(Hash, Eq, PartialEq)]
pub struct Array(Vec<AmqpType>);

#[derive(Eq, PartialEq)]
pub struct Map(IndexMap<AmqpType, AmqpType>);

struct EncodedVec(Vec<Encoded>);

impl Encode for List {
    fn encode(&self) -> Encoded {
        let encoded: Vec<Encoded> = self.0.iter().map(|x| x.encode()).collect();
        let count = encoded.len() as u32;
        let byte_size = encoded.iter().fold(0, |acc, x| acc + x.data_len());
        match (encoded.len(), byte_size) {
            (0, _) => 0x45.into(),
            (len, size) if len <= 255 && size < 256 => {
                Encoded::new_compound(0xc0, count, EncodedVec(encoded).into())
            }
            (_, _) => Encoded::new_compound(0xd0, count, EncodedVec(encoded).into()),
        }
    }
}

impl Encode for Map {
    fn encode(&self) -> Encoded {
        todo!()
    }
}

impl Encode for Array {
    fn encode(&self) -> Encoded {
        let encoded: Vec<Encoded> = self.0.iter().map(|x| x.encode()).collect();
        let byte_size = encoded.iter().fold(0, |acc, x| acc + x.data_len());
        match (encoded.len(), byte_size) {
            (len, size) if len <= 255 && size < 256 => 0xe0.into(),
            (_, _) => 0xf0.into(),
        }
    }
}

impl From<EncodedVec> for Vec<u8> {
    fn from(value: EncodedVec) -> Self {
        let mut res = Vec::new();
        for val in value.0 {
            let mut enc: Vec<u8> = val.into();
            res.append(&mut enc);
        }
        res
    }
}

impl From<Vec<AmqpType>> for List {
    fn from(value: Vec<AmqpType>) -> Self {
        List(value)
    }
}

impl From<IndexMap<AmqpType, AmqpType>> for Map {
    fn from(value: IndexMap<AmqpType, AmqpType>) -> Self {
        Map(value)
    }
}

impl From<Vec<AmqpType>> for Array {
    fn from(value: Vec<AmqpType>) -> Self {
        Array(value)
    }
}

impl Hash for Map {
    fn hash<H: std::hash::Hasher>(&self, _state: &mut H) {
        todo!()
    }
}
