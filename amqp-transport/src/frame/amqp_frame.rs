use amqp_error::AppError;
use amqp_type::composite::performative::Performative;
use std::pin::Pin;
use std::vec::IntoIter;
use tokio_stream::Stream;
use tokio_stream::StreamExt;

#[allow(dead_code)]
pub struct AmqpFrame {
    size: u32,
    doff: u8,
    channel: u16,
    performative: Performative,
}

impl AmqpFrame {
    pub fn encode(self) -> Vec<u8> {
        todo!()
    }

    pub fn try_decode(_size: u32, _doff: u8, _stream: &mut IntoIter<u8>) -> Result<Self, AppError>
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

#[cfg(test)]
mod tests {

    //TODO: write tests
}
