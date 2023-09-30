use crate::serde::encode::{Encode, Encoded};

impl Encode for u32 {
    fn encode(&self) -> Encoded {
        match self {
            0 => Encoded::new_empty(0x43),
            x if x > &0 && x <= &255 => Encoded::new_fixed(0x52, x.to_be_bytes().to_vec()),
            _ => Encoded::new_fixed(0x70, self.to_be_bytes().to_vec()),
        }
    }
}

#[cfg(test)]
mod test {

    use super::*;


    #[test]
    fn construct_uint() {
        let val: u32 = 500;
        assert_eq!(val.encode().constructor(), 0x70);
    }

    #[test]
    fn amqp_type_encodes_uint_value_0_as_zero_length() {
        let val: u32 = 0;
        assert_eq!(val.encode().constructor(), 0x43);
    }

    #[test]
    fn amqp_type_encodes_uint_values_smaller_than_256_as_smalluint() {
        let val: u32 = 255;
        assert_eq!(val.encode().constructor(), 0x52);
    }
}