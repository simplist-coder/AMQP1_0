use crate::serde::encode::{Encode, Encoded};

#[derive(Hash, Eq, PartialEq)]
pub struct Symbol(String);

impl Encode for Symbol {
    fn encode(&self) -> Encoded {
        match self.0.len() {
            x if x <= 255 => Encoded::new_variable(0xa3, self.0.as_bytes().to_vec()),
            _ => Encoded::new_variable(0xb1, self.0.as_bytes().to_vec()),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn construct_symbol() {
        let val = Symbol("".to_string());
        assert_eq!(val.encode().constructor(), 0xa3);
    }
}
