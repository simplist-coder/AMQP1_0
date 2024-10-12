use crate::error::AppError;
use std::pin::Pin;
use tokio_stream::Stream;


#[allow(dead_code)]
pub(crate) trait Decode {
    /// Tries to decode the implementing type from the byte iterator. this advances the iterator until the
    /// type and its value are completely decoded.
    /// TODO: Make this function async
    async fn try_decode(constructor: u8, stream: &mut Pin<Box<impl Stream<Item=u8>>>) -> Result<Self, AppError>
    where
        Self: Sized;
}
