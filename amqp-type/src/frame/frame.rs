use crate::constants::constructors::{AMQP_FRAME, SASL_FRAME};
use crate::error::AppError;
use crate::frame::amqp_frame::AmqpFrame;
use crate::frame::sasl_frame::SaslFrame;
use crate::serde::decode::Decode;
use crate::serde::encode::{Encode, Encoded};
use std::pin::Pin;
use tokio_stream::Stream;

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

impl Decode for Frame {
    async fn try_decode(
        constructor: u8,
        stream: &mut Pin<Box<impl Stream<Item = u8>>>,
    ) -> Result<Self, AppError>
    where
        Self: Sized,
    {
        match constructor {
            AMQP_FRAME => AmqpFrame::try_decode(AMQP_FRAME, stream)
                .await
                .map(Frame::AmqpFrame),
            SASL_FRAME => SaslFrame::try_decode(SASL_FRAME, stream)
                .await
                .map(Frame::SaslFrame),
            illegal => Err(AppError::DeserializationIllegalConstructorError(illegal)),
        }
    }
}
