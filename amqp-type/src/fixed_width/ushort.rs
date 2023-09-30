use crate::serde::encode::{Encode, Encoded};

impl Encode for u16 {
    fn encode(&self) -> Encoded {
        Encoded::new_fixed(0x60, self.to_be_bytes().to_vec())
    }
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn construct_ushort() {
        let val: u16 = 16;
        assert_eq!(val.encode().constructor(), 0x60);
    }
}
