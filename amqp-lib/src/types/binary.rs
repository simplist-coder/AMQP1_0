use crate::types::amqp_type::{Encode, Constructor};

#[derive(Hash, Eq, PartialEq)]
pub struct Binary(Vec<u8>);


impl Encode for Binary {
    fn constructor(&self) -> Constructor {
        todo!()
    }

    fn encode(&self) -> Vec<u8> {
        todo!()
    }
}

impl From<Vec<u8>> for Binary {
    fn from(value: Vec<u8>) -> Self {
        Binary(value)
    }
}
