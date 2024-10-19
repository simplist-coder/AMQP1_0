use crate::error::AppError;
use crate::serde::decode::Decode;
use crate::serde::encode::{Encode, Encoded};
use std::pin::Pin;
use tokio_stream::Stream;

#[derive(Debug, Hash, Eq, PartialEq, Clone)]
pub struct Decimal128();

/**
f128 is not yet supported by rust, see <https://github.com/rust-lang/rust/issues/116909>
Implement this when f128 it is available in stable.
 */
impl Encode for Decimal128 {
    fn encode(self) -> Encoded {
        0x94.into()
    }
}

impl Decode for Decimal128 {
    async fn try_decode(
        _constructor: u8,
        _stream: &mut Pin<Box<impl Stream<Item = u8>>>,
    ) -> Result<Self, AppError>
    where
        Self: Sized,
    {
        todo!("Decimal128  type is not implemented yet")
    }
}

#[cfg(test)]
mod test {}
