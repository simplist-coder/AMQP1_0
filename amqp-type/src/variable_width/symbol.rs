use std::pin::Pin;
use tokio_stream::Stream;
use crate::error::AppError;
use crate::serde::decode::Decode;
use crate::serde::encode::{Encode, Encoded};

#[derive(Hash, Eq, PartialEq)]
pub struct Symbol(String);

impl Encode for Symbol {
    fn encode(&self) -> Encoded {
        match self.0.len() {
            x if x <= 255 => Encoded::new_variable(0xa3, self.0.as_bytes().to_vec()),
            _ => Encoded::new_variable(0xb1, self.0.as_bytes().to_vec()),
        }
    }
}

impl Decode for Symbol {

    async fn try_decode(_constructor: u8, _iter: Pin<Box<impl Stream<Item=u8>>>) -> Result<Self, AppError>
    where
        Self: Sized
    {
        todo!()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn construct_symbol() {
        let val = Symbol("".to_string());
        assert_eq!(val.encode().constructor(), 0xa3);
    }
}
