use amqp_error::AppError;
use std::pin::Pin;
use tokio_stream::Stream;

pub(crate) trait Decode {
    async fn try_decode(
        constructor: u8,
        stream: &mut Pin<Box<impl Stream<Item = u8>>>,
    ) -> Result<Self, AppError>
    where
        Self: Sized;
}
