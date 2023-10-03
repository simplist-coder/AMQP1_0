use crate::error::AppError;

#[derive(Hash, Eq, PartialEq)]
pub struct Constructor(u8);

pub trait Decode {
    /// This function determines if an implementor can decode itself from the current position in the iterator
    /// by checking whether the next byte in the iterator is a valid constructor for this type.
    /// TODO: make this function async
    fn can_decode(iter: impl Iterator<Item = u8>) -> bool;

    /// Tries to decode the implementing type from the byte iterator. this advances the iterator until the 
    /// type and its value are completely decoded.
    /// TODO: Make this function async
    fn try_decode(iter: impl Iterator<Item = u8>) -> Result<Self, AppError>
    where
        Self: Sized;
}

impl From<u8> for Constructor {
    fn from(value: u8) -> Self {
        Constructor(value)
    }
}
