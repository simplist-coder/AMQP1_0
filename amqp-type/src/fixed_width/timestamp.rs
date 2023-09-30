use crate::serde::encode::{Encode, Encoded};

#[derive(Hash, Eq, PartialEq)]
pub struct Timestamp(u64);

impl Encode for Timestamp {
    fn encode(&self) -> Encoded {
        0x83.into()
    }
}


#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn construct_timestamp() {
        let val = Timestamp(1);
        assert_eq!(val.encode().constructor(), 0x83);
    }
}