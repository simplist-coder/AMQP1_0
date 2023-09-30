use crate::serde::encode::{Encode, Encoded};

impl Encode for i32 {
    fn encode(&self) -> Encoded {
        match self {
            x if x >= &-128 && x <= &127 => Encoded::new_fixed(0x54, x.to_be_bytes().to_vec()),
            _ => Encoded::new_fixed(0x71, self.to_be_bytes().to_vec()),
        }
    }
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn construct_int() {
        let val = 500;
        assert_eq!(val.encode().constructor(), 0x71);
    }

    #[test]
    fn amqp_encodes_ints_between_neg_128_and_127_as_smallint() {
        let lower = -128;
        let higher = 127;
        assert_eq!(lower.encode().constructor(), 0x54);
        assert_eq!(higher.encode().constructor(), 0x54);
    }
}