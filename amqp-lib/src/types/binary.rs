use crate::types::amqp_type::{Constructor, Encode};

use super::amqp_type::Encoded;

#[derive(Hash, Eq, PartialEq)]
pub struct Binary(Vec<u8>);

impl Encode for Binary {
    fn encode(&self) -> Encoded {
        match self.0.len() {
            x if x <= 255 => Encoded::new(0xa0, Some(self.0.to_owned())),
            _ => Encoded::new(0xb0, Some(self.0.to_owned()))
        }
    }
}

impl From<Vec<u8>> for Binary {
    fn from(value: Vec<u8>) -> Self {
        Binary(value)
    }
}
