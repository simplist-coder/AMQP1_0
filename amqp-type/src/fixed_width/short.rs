use crate::serde::encode::{Encode, Encoded};

impl Encode for i16 {
    fn encode(&self) -> Encoded {
        Encoded::new_fixed(0x61, self.to_be_bytes().to_vec())
    }
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn construct_short() {
        let val : i16 = 8;
        assert_eq!(val.encode().constructor(), 0x61);
    }
}