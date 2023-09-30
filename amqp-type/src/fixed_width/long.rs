use crate::serde::encode::{Encode, Encoded};

impl Encode for i64 {
    fn encode(&self) -> Encoded {
        match self {
            x if x >= &-128 && x <= &127 => Encoded::new_fixed(0x55, x.to_be_bytes().to_vec()),
            _ => Encoded::new_fixed(0x81, self.to_be_bytes().to_vec()),
        }
    }
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn construct_long() {
        let val: i64 = 500;
        assert_eq!(val.encode().constructor(), 0x81);
    }

    #[test]
    fn amqp_encodes_longs_between_neg_128_and_127_as_smalllong() {
        let lower: i64 = -128;
        let higher: i64 = 127;
        assert_eq!(lower.encode().constructor(), 0x55);
        assert_eq!(higher.encode().constructor(), 0x55);
    }
}
