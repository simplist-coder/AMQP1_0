use crate::serde::encode::{Encode, Encoded};
use std::hash::Hash;

pub struct Float(f32);

impl Hash for Float {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.0.to_bits().hash(state)
    }
}

impl PartialEq for Float {
    fn eq(&self, other: &Self) -> bool {
        self.0.to_bits() == other.0.to_bits()
    }

    fn ne(&self, other: &Self) -> bool {
        !self.eq(other)
    }
}

impl Eq for Float {}

impl Encode for Float {
    fn encode(&self) -> Encoded {
        Encoded::new_fixed(0x72, self.0.to_be_bytes().to_vec())
    }
}

impl From<f32> for Float {
    fn from(value: f32) -> Self {
        Float(value)
    }
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn construct_float() {
        let val: Float = 32.0.into();
        assert_eq!(val.encode().constructor(), 0x72);
    }
}
