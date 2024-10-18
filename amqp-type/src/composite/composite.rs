use crate::constants::constructors::DESCRIBED_TYPE;
use crate::error::AppError;
use crate::primitive::compound::list::List;
use crate::primitive::variable_width::symbol::Symbol;
use crate::serde::decode::Decode;
use crate::serde::encode::{Encode, Encoded};
use std::pin::Pin;
use tokio_stream::Stream;

pub enum Descriptor {
    Symbol(Symbol),
    Code(u64),
}
pub struct Composite(Descriptor, List);

impl Encode for Composite {
    fn encode(self) -> Encoded {
        let descriptor = match self.0 {
            Descriptor::Symbol(x) => x.encode().to_bytes(),
            Descriptor::Code(x) => x.encode().to_bytes(),
        };
        let data = self.1.encode().to_bytes();
        Encoded::new_composite(DESCRIBED_TYPE, descriptor, data)
    }
}

impl Decode for Composite {
    async fn try_decode(
        _constructor: u8,
        _stream: &mut Pin<Box<impl Stream<Item = u8>>>,
    ) -> Result<Self, AppError>
    where
        Self: Sized,
    {
        todo!()
    }
}
