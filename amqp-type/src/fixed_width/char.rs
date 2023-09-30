use crate::serde::encode::{Encode, Encoded};

impl Encode for char {
    fn encode(&self) -> Encoded {
        Encoded::new_fixed(0x73, self.to_string().into_bytes())
    }
}

#[cfg(test)]
mod test {

    use super::*;
    #[test]
    fn construct_char() {
        let val = 'a';
        assert_eq!(val.encode().constructor(), 0x73);
    }
}