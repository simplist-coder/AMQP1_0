use std::pin::Pin;
use tokio_stream::Stream;
use crate::amqp_type::AmqpType;
use crate::constants::constructors::{ARRAY, ARRAY_SHORT};
use crate::error::AppError;
use crate::serde::decode::Decode;
use crate::serde::encode::{Encode, Encoded};

#[derive(Hash, Eq, PartialEq)]
pub struct Array(Vec<AmqpType>);

impl Encode for Array {
    fn encode(&self) -> Encoded {
        let encoded: Vec<Encoded> = self.0.iter().map(|x| x.encode()).collect();
        let byte_size = encoded.iter().fold(0, |acc, x| acc + x.data_len());
        match (encoded.len(), byte_size) {
            (len, size) if len <= 255 && size < 256 => ARRAY_SHORT.into(),
            (_, _) => ARRAY.into(),
        }
    }
}

impl Decode for Array {

    async fn try_decode(_constructor: u8, _iter: Pin<Box<impl Stream<Item=u8>>>) -> Result<Self, AppError>
    where
        Self: Sized
    {
        todo!()
    }
}

impl From<Vec<AmqpType>> for Array {
    fn from(value: Vec<AmqpType>) -> Self {
        Array(value)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn construct_array_with_less_than_255_elements() {
        let val = Array(Vec::new());
        assert_eq!(val.encode().constructor(), 0xe0);
    }

    #[test]
    fn construct_array_with_more_than_255_elements() {
        let mut arr = vec![];
        for i in 0..500 {
            arr.push(i.into())
        }
        let val = Array(arr);
        assert_eq!(val.encode().constructor(), 0xf0);
    }

    #[test]
    fn construct_array_with_less_than_255_elements_and_larger_than_255_bytes() {
        let mut arr = vec![];
        for _ in 0..100 {
            arr.push("aaaaaaaaaaaaaaaaaaaa".into());
        }
        let val = Array(arr);
        assert_eq!(val.encode().constructor(), 0xf0);
    }
}
