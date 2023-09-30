use crate::serde::encode::{Encode, Encoded};

impl Encode for bool {
    #[cfg(feature = "zero-length-bools")]
    fn encode(&self) -> Encoded {
        match self {
            true => 0x41.into(),
            false => 0x42.into(),
        }
    }

    #[cfg(not(feature = "zero-length-bools"))]
    fn encode(&self) -> Encoded {
        match self {
            true => Encoded::new_fixed(0x56, vec![0x01]),
            false => Encoded::new_fixed(0x56, vec![0x00]),
        }
    }
}

#[cfg(test)]
mod test {
    use crate::serde::encode::Encode;

    use super::*;

    #[test]
    #[cfg(not(feature = "zero-length-bools"))]
    fn construct_bool() {
        assert_eq!(true.encode().constructor(), 0x56);
    }

    #[test]
    #[cfg(feature = "zero-length-bools")]
    fn amqp_type_constructs_bool_false_as_zero_length() {
        assert_eq!(false.encode().constructor(), 0x42);
    }

    #[test]
    #[cfg(feature = "zero-length-bools")]
    fn amqp_type_constructs_bool_true_as_zero_length() {
        assert_eq!(true.encode().constructor(), 0x41)
    }
}