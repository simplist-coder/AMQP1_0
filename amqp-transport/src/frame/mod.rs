pub mod amqp_frame;
pub mod performative;
pub mod performatives;
pub mod sasl_frame;

use crate::constants::{AMQP_FRAME, SASL_FRAME};
use crate::frame::amqp_frame::AmqpFrame;
use crate::frame::sasl_frame::SaslFrame;
use amqp_type::error::AppError;
use amqp_type::utils::async_util::{read_bytes, read_bytes_4};
use std::pin::Pin;
use tokio_stream::Stream;
use amqp_type::error::amqp_error::AmqpError;

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
        // size adjusted by -4 to account for already read size bytes
        let mut buffer = read_bytes(stream, size as usize - 4).await?.into_iter();
        let doff = buffer
            .next()
            .ok_or(AmqpError::DecodeError)?;
        let frame_type = buffer
            .next()
            .ok_or(AmqpError::DecodeError)?;
        match frame_type {
            AMQP_FRAME => AmqpFrame::try_decode(doff, &mut buffer).map(Frame::AmqpFrame),
            SASL_FRAME => SaslFrame::try_decode(doff, &mut buffer).map(Frame::SaslFrame),
            _ => Err(AmqpError::DecodeError)?
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
