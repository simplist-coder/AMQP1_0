use crate::serde::encode::{Encode, Encoded};

impl Encode for u64 {
    fn encode(&self) -> Encoded {
        match self {
            0 => Encoded::new_empty(0x44),
            x if x > &&0 && x <= &255 => Encoded::new_fixed(0x53, x.to_be_bytes().to_vec()),
            _ => Encoded::new_fixed(0x80, self.to_be_bytes().to_vec()),
        }
    }
}

#[cfg(test)]
mod test {

    use super::*;


    #[test]
    fn construct_ulong() {
        let val: u64 = 500;
        assert_eq!(val.encode().constructor(), 0x80);
    }

    #[test]
    fn amqp_type_encodes_ulong_smaller_than_256_as_smallulong() {
        let val: u64 = 255;
        assert_eq!(val.encode().constructor(), 0x53);
    }

    #[test]
    fn amqp_type_encodes_ulong_value_0_as_zero_length() {
        let val: u64 = 0;
        assert_eq!(val.encode().constructor(), 0x44);
    }
}