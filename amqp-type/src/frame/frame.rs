use crate::common::{read_bytes, read_bytes_4};
use crate::constants::constructors::{AMQP_FRAME, SASL_FRAME};
use crate::error::AppError;
use crate::frame::amqp_frame::AmqpFrame;
use crate::frame::sasl_frame::SaslFrame;
use crate::serde::encode::{Encode, Encoded};
use std::pin::Pin;
use tokio_stream::{iter, Stream, StreamExt};

pub enum Frame {
    AmqpFrame(AmqpFrame),
    SaslFrame(SaslFrame),
}

impl Encode for Frame {
    fn encode(self) -> Encoded {
        match self {
            Frame::AmqpFrame(amqp) => amqp.encode(),
            Frame::SaslFrame(sasl) => sasl.encode(),
        }
    }
}

impl Frame {
    pub async fn try_decode(stream: &mut Pin<Box<impl Stream<Item = u8>>>) -> Result<Self, AppError>
    where
        Self: Sized,
    {
        let size = u32::from_be_bytes(read_bytes_4(stream).await?);
        let mut buffer = Box::pin(iter(read_bytes(stream, size as usize).await?));
        let doff = buffer
            .next()
            .await
            .ok_or(AppError::IteratorEmptyOrTooShortError)?;
        let frame_type = buffer
            .next()
            .await
            .ok_or(AppError::IteratorEmptyOrTooShortError)?;
        match frame_type {
            AMQP_FRAME => AmqpFrame::try_decode(size, doff, &mut buffer)
                .await
                .map(Frame::AmqpFrame),
            SASL_FRAME => SaslFrame::try_decode(size, doff, &mut buffer)
                .await
                .map(Frame::SaslFrame),
            illegal => Err(AppError::DeserializationIllegalConstructorError(illegal)),
        }
    }
}

impl From<AmqpFrame> for Frame {
    fn from(value: AmqpFrame) -> Self {
        Frame::AmqpFrame(value)
    }
}

impl From<SaslFrame> for Frame {
    fn from(value: SaslFrame) -> Self {
        Frame::SaslFrame(value)
    }
}
