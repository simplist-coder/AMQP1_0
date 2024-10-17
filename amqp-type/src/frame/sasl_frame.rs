use crate::error::AppError;
use crate::serde::encode::{Encode, Encoded};
use std::pin::Pin;
use tokio_stream::Stream;

pub struct SaslFrame {}

impl Encode for SaslFrame {
    fn encode(self) -> Encoded {
        todo!()
    }
}

impl SaslFrame {
    pub(crate) async fn try_decode(
        _size: u32,
        _doff: u8,
        _stream: &mut Pin<Box<impl Stream<Item = u8>>>,
    ) -> Result<Self, AppError>
    where
        Self: Sized,
    {
        todo!()
    }
}
