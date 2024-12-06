use crate::error::AppError;
use std::vec::IntoIter;

pub trait Decode {
    fn try_decode(constructor: u8, stream: &mut IntoIter<u8>) -> Result<Self, AppError>
    where
        Self: Sized;
}
