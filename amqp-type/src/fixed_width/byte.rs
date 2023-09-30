use crate::serde::encode::{Encode, Encoded};

impl Encode for i8 {
    fn encode(&self) -> Encoded {
        Encoded::new_fixed(0x51, self.to_be_bytes().to_vec())
    }
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn construct_byte() {
        let val: i8 = 8;
        assert_eq!(val.encode().constructor(), 0x51);
    }
}
