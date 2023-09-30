use crate::amqp_type::Encode;

use super::amqp_type::Encoded;

#[derive(Hash, Eq, PartialEq)]
pub struct Binary(Vec<u8>);

impl Encode for Binary {
    fn encode(&self) -> Encoded {
        match self.0.len() {
            x if x <= 255 => Encoded::new_variable(0xa0, self.0.to_owned()),
            _ => Encoded::new_variable(0xb0, self.0.to_owned()),
        }
    }
}

impl From<Vec<u8>> for Binary {
    fn from(value: Vec<u8>) -> Self {
        Binary(value)
    }
}
