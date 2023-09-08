
use indexmap::IndexMap;
use crate::types::amqp_type::{Encode, Constructor, AmqpType};


#[derive(Hash, Eq, PartialEq)]
pub struct List(Vec<AmqpType>);
#[derive(Hash, Eq, PartialEq)]
pub struct Array(Vec<AmqpType>);
#[derive(Hash, Eq, PartialEq)]
pub struct Map(IndexMap<AmqpType, AmqpType>);

impl Encode for List {
    fn constructor(&self) -> Constructor {
        todo!()
    }

    fn encode(&self) -> Vec<u8> {
        todo!()
    }
}

impl Encode for Map {
    fn constructor(&self) -> Constructor {
        todo!()
    }

    fn encode(&self) -> Vec<u8> {
        todo!()
    }
}

impl Encode for Array {
    fn constructor(&self) -> Constructor {
        todo!()
    }

    fn encode(&self) -> Vec<u8> {
        todo!()
    }
}

impl From<Vec<AmqpType>> for List{
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
