use amqp_type::error::AppError;
use std::pin::Pin;
use tokio_stream::Stream;

pub struct SaslFrame {}

impl SaslFrame {
    pub(crate) fn encode(self) -> Vec<u8> {
        todo!()
    }
}

impl SaslFrame {
    pub async fn try_decode(
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
