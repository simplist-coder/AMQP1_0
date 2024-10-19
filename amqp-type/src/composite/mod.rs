use crate::constants::constructors::DESCRIBED_TYPE;
use crate::primitive::compound::list::List;
use crate::primitive::variable_width::symbol::Symbol;
use crate::serde::decode::Decode;
use crate::serde::encode::{Encode, Encoded};
use amqp_error::AppError;
use std::vec::IntoIter;

pub enum Descriptor {
    Symbol(Symbol),
    Code(u64),
}
pub struct Composite(Descriptor, List);

impl Encode for Composite {
    fn encode(self) -> Encoded {
        let descriptor = match self.0 {
            Descriptor::Symbol(x) => x.encode().serialize(),
            Descriptor::Code(x) => x.encode().serialize(),
        };
        let data = self.1.encode().serialize();
        Encoded::new_composite(DESCRIBED_TYPE, descriptor, data)
    }
}

impl Decode for Composite {
    fn try_decode(_constructor: u8, _stream: &mut IntoIter<u8>) -> Result<Self, AppError>
    where
        Self: Sized,
    {
        todo!()
    }
}
