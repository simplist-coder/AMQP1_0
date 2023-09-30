use crate::serde::encode::Encode;
#[derive(Hash, Eq, PartialEq)]
pub struct Constructor(u8);
pub trait Decode<'a>: From<&'a [u8]> + Encode {}


impl From<u8> for Constructor {
    fn from(value: u8) -> Self {
        Constructor(value)
    }
}