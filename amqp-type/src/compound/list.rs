use crate::amqp_type::AmqpType;
use crate::compound::encoded_vec::EncodedVec;
use crate::serde::encode::{Encode, Encoded};

#[derive(Hash, Eq, PartialEq)]
pub struct List(Vec<AmqpType>);

impl Encode for List {
    fn encode(&self) -> Encoded {
        let encoded: Vec<Encoded> = self.0.iter().map(|x| x.encode()).collect();
        let count = encoded.len() as u32;
        let byte_size = encoded.iter().fold(0, |acc, x| acc + x.data_len());
        match (encoded.len(), byte_size) {
            (0, _) => 0x45.into(),
            (len, size) if len <= 255 && size < 256 => {
                Encoded::new_compound(0xc0, count, EncodedVec::new(encoded).into())
            }
            (_, _) => Encoded::new_compound(0xd0, count, EncodedVec::new(encoded).into()),
        }
    }
}

impl From<Vec<AmqpType>> for List {
    fn from(value: Vec<AmqpType>) -> Self {
        List(value)
    }
}

#[cfg(test)]
mod test {

    use super::*;
    #[test]
    fn construct_empty_list() {
        let val = List(vec![]);
        assert_eq!(val.encode().constructor(), 0x45);
    }

    #[test]
    fn construct_list_with_less_than_255_elements() {
        let val = List(vec![1.into()]);
        assert_eq!(val.encode().constructor(), 0xc0);
    }

    #[test]
    fn construct_list_with_more_than_255_elements() {
        let mut arr = vec![];
        for i in 0..500 {
            arr.push(i.into())
        }
        let val = List(arr);
        assert_eq!(val.encode().constructor(), 0xd0);
    }

    #[test]
    fn construct_list_with_less_than_255_elements_and_larger_than_255_bytes() {
        let mut arr = vec![];
        for _ in 0..100 {
            arr.push("aaaaaaaaaaaaaaaaaaaa".into());
        }
        let val = List(arr);
        assert_eq!(val.encode().constructor(), 0xd0);
    }
}