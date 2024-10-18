use amqp_type::amqp_type::AmqpType;
use amqp_type::error::AppError;
use std::pin::Pin;
use tokio_stream::Stream;
use tokio_stream::StreamExt;

#[allow(dead_code)]
pub struct AmqpFrame {
    size: u32,
    doff: u8,
    channel: u16,
    amqp_type: AmqpType,
}

impl AmqpFrame {
    pub fn encode(self) -> Vec<u8> {
        todo!()
    }

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

#[allow(dead_code)]
async fn skip_extended_header(doff: u8, stream: &mut Pin<Box<impl Stream<Item = u8>>>) {
    for _ in 0..(doff * 4) - 8 {
        stream.next().await;
    }
}
