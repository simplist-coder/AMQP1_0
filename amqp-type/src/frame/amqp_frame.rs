use crate::error::AppError;
use crate::serde::decode::Decode;
use crate::serde::encode::{Encode, Encoded};
use std::pin::Pin;
use tokio_stream::Stream;

pub struct AmqpFrame {}

impl Encode for AmqpFrame {
    fn encode(self) -> Encoded {
        todo!()
    }
}

impl Decode for AmqpFrame {
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
