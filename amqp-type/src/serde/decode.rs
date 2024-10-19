use amqp_error::AppError;
use std::vec::IntoIter;

pub(crate) trait Decode {
    fn try_decode(constructor: u8, stream: &mut IntoIter<u8>) -> Result<Self, AppError>
    where
        Self: Sized;
}
