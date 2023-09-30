use std::hash::Hash;
use indexmap::IndexMap;
use crate::amqp_type::AmqpType;
use crate::compound::encoded_vec::EncodedVec;
use crate::serde::encode::{Encode, Encoded};

#[derive(Eq, PartialEq)]
pub struct Map(IndexMap<AmqpType, AmqpType>);


impl Encode for Map {
    fn encode(&self) -> Encoded {
        let mut res: Vec<Encoded> = Vec::new();
        let mut data_len = 0;
        let mut count = 0;
        for (key, value) in &self.0 {
            let k = key.encode();
            let v = value.encode();
            data_len += k.data_len() + v.data_len();
            res.push(k);
            res.push(v);
            count += 2;
        }
        match data_len {
            x if x <= 255 => Encoded::new_compound(0xc1, count, EncodedVec::new(res).into()),
            _ => Encoded::new_compound(0xd1, count, EncodedVec::new(res).into())
        }
    }
}

impl Hash for Map {
    fn hash<H: std::hash::Hasher>(&self, _state: &mut H) {
        todo!()
    }
}

impl From<IndexMap<AmqpType, AmqpType>> for Map {
    fn from(value: IndexMap<AmqpType, AmqpType>) -> Self {
        Map(value)
    }
}

#[cfg(test)]
mod test {

    use super::*;
    #[test]
    fn construct_map_with_less_than_255_elements() {
        let val = AmqpType::Map(IndexMap::new().into());
        assert_eq!(val.encode().constructor(), 0xc1);
    }

    #[test]
    fn construct_map_with_less_more_255_elements() {
        let mut map = IndexMap::new();
        for i in 1..500 {
            map.insert(i.into(), i.into());
        }
        let val = AmqpType::Map(map.into());
        assert_eq!(val.encode().constructor(), 0xd1);
    }
}