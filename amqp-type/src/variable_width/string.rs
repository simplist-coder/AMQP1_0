use crate::serde::encode::{Encode, Encoded};

impl Encode for String {
    fn encode(&self) -> Encoded {
        match self.len() {
            x if x >= 0usize && x <= 255usize => {
                Encoded::new_variable(0xa1, self.as_bytes().to_vec())
            }
            _ => Encoded::new_variable(0xb1, self.as_bytes().to_vec()),
        }
    }
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn amqp_type_encodes_strings_up_to_255_bytes_as_str8() {
        let val = "hello".to_string();
        assert_eq!(val.encode().constructor(), 0xa1);
    }

    #[test]
    fn amqp_type_encodes_strings_longer_than_255_bytes_as_str32() {
        let val = "hhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhh\
        hhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhh\
        hhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhh\
        hhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhh\
        hhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhh\
        hhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhh".to_string();
        assert_eq!(val.encode().constructor(), 0xb1);
    }
}