use std::hash::Hash;

use crate::types::amqp_type::{Encode, Encoded};

pub struct Float(f32);
pub struct Double(f64);

/// Crate assumes nothing about the values being passed to it.
/// Any kind of f32 value is handled as is.
/// This means that the hash function considers only the bytes of a float.
/// Two f32 or f64 values are considered Equal if and only if, the entirety of their by sequences match.
/// This ensures that no information is lost.
/// regardless of whether they mean Nan or similar things.
/// As a result, Nan == Nan if and only if the two numbers have the exact same byte sequence.
impl Hash for Float {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.0.to_bits().hash(state)
    }
}

impl Hash for Double {
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

impl PartialEq for Double {
    fn eq(&self, other: &Self) -> bool {
        self.0.to_bits() == other.0.to_bits()
    }

    fn ne(&self, other: &Self) -> bool {
        !self.eq(other)
    }
}

impl Encode for Float {
    fn encode(&self) -> Encoded {
        Encoded::new_fixed(0x72, self.0.to_be_bytes().to_vec())
    }
}

impl Encode for Double {
    fn encode(&self) -> Encoded {
        Encoded::new_variable(0x82, self.0.to_be_bytes().to_vec())
    }
}

impl From<f32> for Float {
    fn from(value: f32) -> Self {
        Float(value)
    }
}
impl From<f64> for Double {
    fn from(value: f64) -> Self {
        Double(value)
    }
}
