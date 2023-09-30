use crate::serde::encode::{Encode, Encoded};

impl Encode for u8 {
    fn encode(&self) -> Encoded {
        Encoded::new_fixed(0x50, self.to_be_bytes().to_vec())
    }
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn construct_ubyte() {
        let val: u8 = 8;
        assert_eq!(val.encode().constructor(), 0x50);
    }
}