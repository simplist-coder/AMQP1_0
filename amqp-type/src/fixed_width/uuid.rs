use crate::serde::encode::{Encode, Encoded};

#[derive(Hash, Eq, PartialEq)]
pub struct Uuid(uuid::Uuid);

impl Encode for Uuid {
    fn encode(&self) -> Encoded {
        Encoded::new_fixed(0x98, self.0.into_bytes().to_vec())
    }
}

#[cfg(test)]
mod test {

    use super::*;
    #[test]
    fn construct_uuid() {
        let val = Uuid(uuid::Uuid::new_v4());
        assert_eq!(val.encode().constructor(), 0x98);
    }
}
