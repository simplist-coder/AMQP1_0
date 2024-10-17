use crate::common::{drain, read_bytes_2};
use crate::error::AppError;
use crate::frame::performative::performative::Performative;
use crate::serde::encode::{Encode, Encoded};
use std::pin::Pin;
use tokio_stream::Stream;
use tokio_stream::StreamExt;

pub struct AmqpFrame {
    size: u32,
    doff: u8,
    channel: u16,
    performative: Performative,
    body: Vec<u8>,
}

impl Encode for AmqpFrame {
    fn encode(self) -> Encoded {
        todo!()
    }
}

impl AmqpFrame {
    pub(crate) async fn try_decode(
        size: u32,
        doff: u8,
        stream: &mut Pin<Box<impl Stream<Item = u8>>>,
    ) -> Result<Self, AppError>
    where
        Self: Sized,
    {
        let channel = u16::from_be_bytes(read_bytes_2(stream).await?);
        skip_extended_header(doff, stream).await;
        let performative = Performative::try_decode(stream).await?;
        let body = drain(stream).await;
        Ok(AmqpFrame {
            size,
            doff,
            channel,
            performative,
            body,
        })
    }
}

async fn skip_extended_header(doff: u8, stream: &mut Pin<Box<impl Stream<Item = u8>>>) {
    for _ in 0..(doff * 4) - 8 {
        stream.next().await;
    }
}
