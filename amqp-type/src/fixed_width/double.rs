use std::hash::Hash;
use crate::serde::encode::{Encode, Encoded};


/// Crate assumes nothing about the values being passed to it.
/// Any kind of f32 value is handled as is.
/// This means that the hash function considers only the bytes of a float.
/// Two f32 or f64 values are considered Equal if and only if, the entirety of their by sequences match.
/// This ensures that no information is lost.
/// regardless of whether they mean Nan or similar things.
/// As a result, Nan == Nan if and only if the two numbers have the exact same byte sequence.
pub struct Double(f64);

impl Hash for Double {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.0.to_bits().hash(state)
    }
}
impl Eq for Double {}

impl PartialEq for Double {
    fn eq(&self, other: &Self) -> bool {
        self.0.to_bits() == other.0.to_bits()
    }

    fn ne(&self, other: &Self) -> bool {
        !self.eq(other)
    }
}

impl Encode for Double {
    fn encode(&self) -> Encoded {
        Encoded::new_variable(0x82, self.0.to_be_bytes().to_vec())
    }
}


impl From<f64> for Double {
    fn from(value: f64) -> Self {
        Double(value)
    }
}

#[cfg(test)]
mod test {

    use super::*;
    #[test]
    fn construct_double() {
        let val: Double = 64.0.into();
        assert_eq!(val.encode().constructor(), 0x82);
    }
}