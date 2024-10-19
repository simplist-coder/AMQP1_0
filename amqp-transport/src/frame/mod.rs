pub mod amqp_frame;
pub mod performative;
pub mod performatives;
pub mod sasl_frame;

use crate::constants::{AMQP_FRAME, SASL_FRAME};
use crate::frame::amqp_frame::AmqpFrame;
use crate::frame::sasl_frame::SaslFrame;
use amqp_error::AppError;
use amqp_utils::async_util::{read_bytes, read_bytes_4};
use std::pin::Pin;
use tokio_stream::Stream;

pub enum Frame {
    AmqpFrame(AmqpFrame),
    SaslFrame(SaslFrame),
}

impl Frame {
    pub fn encode(self) -> Vec<u8> {
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
        let mut buffer = read_bytes(stream, size as usize).await?.into_iter();
        let doff = buffer
            .next()
            .ok_or(AppError::IteratorEmptyOrTooShortError)?;
        let frame_type = buffer
            .next()
            .ok_or(AppError::IteratorEmptyOrTooShortError)?;
        match frame_type {
            AMQP_FRAME => AmqpFrame::try_decode(size, doff, &mut buffer).map(Frame::AmqpFrame),
            SASL_FRAME => SaslFrame::try_decode(size, doff, &mut buffer).map(Frame::SaslFrame),
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

#[cfg(test)]
mod tests {

    //TODO: write tests
}
