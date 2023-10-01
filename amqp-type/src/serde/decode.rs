use crate::error::AppError;


#[derive(Hash, Eq, PartialEq)]
pub struct Constructor(u8);

pub trait Decode {
    fn can_decode(data: impl Iterator<Item = u8>) -> bool;
    fn try_decode(data: impl Iterator<Item = u8>) -> Result<Self, AppError>
    where
        Self: Sized;
}

impl From<u8> for Constructor {
    fn from(value: u8) -> Self {
        Constructor(value)
    }
}
