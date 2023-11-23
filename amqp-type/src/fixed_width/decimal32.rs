use std::hash::{Hash, Hasher};

use crate::serde::encode::{Encode, Encoded};

// 7 digits
const DEFAULT_CONSTR: u8 = 0x74;

pub struct Decimal32(f32);

impl Encode for Decimal32 {
    fn encode(&self) -> Encoded {
        Encoded::new_fixed(
            DEFAULT_CONSTR,
            encode_to_bytes(&self.0).to_be_bytes().to_vec(),
        )
    }
}

impl Hash for Decimal32 {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.0.to_bits().hash(state)
    }
}

impl PartialEq for Decimal32 {
    fn eq(&self, other: &Self) -> bool {
        self.0.to_bits().eq(&other.0.to_bits())
    }
}

impl Eq for Decimal32 {}

impl From<f32> for Decimal32 {
    fn from(value: f32) -> Self {
        Decimal32(value)
    }
}

fn encode_to_bytes(value: &f32) -> u32 {
    value.to_bits()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn construct_decimal_32() {
        let val: Decimal32 = 32f32.into();
        assert_eq!(val.encode().constructor(), 0x74);
    }

    #[test]
    fn test_positive_number() {
        let decimal = 0.15625;
        let encoded = encode_to_bytes(&decimal);
        let expected = 0b00111110001000000000000000000000;
        assert_eq!(encoded, expected);
    }

    #[test]
    fn test_negative_number() {
        let decimal = -0.15625;
        let encoded = encode_to_bytes(&decimal);
        let expected = 0b10111110001000000000000000000000;
        assert_eq!(encoded, expected);
    }

    #[test]
    fn test_large_number() {
        let decimal = 3.4028235e38; // Max value for f32
        let encoded = encode_to_bytes(&decimal);
        let expected = 0b01111111011111111111111111111111;
        assert_eq!(encoded, expected);
    }

    #[test]
    fn test_small_subnormal_number() {
        let decimal = 1E-45; // Smallest subnormal in f32
        let encoded = encode_to_bytes(&decimal);
        let expected = 0b00000000000000000000000000000001;
        assert_eq!(encoded, expected);
    }

    #[test]
    fn test_zero() {
        let decimal = 0f32;
        let encoded = encode_to_bytes(&decimal);
        let expected = 0b00000000000000000000000000000000;
        assert_eq!(encoded, expected);
    }

    #[test]
    fn test_one() {
        let decimal = 1f32;
        let encoded = encode_to_bytes(&decimal);
        let expected = 0b00111111100000000000000000000000;
        assert_eq!(encoded, expected);
    }

    #[test]
    fn test_infinity() {
        let decimal = f32::INFINITY; // A number too large for f32, should be infinity
        let encoded = encode_to_bytes(&decimal);
        let expected = 0b01111111100000000000000000000000; // Positive infinity in f32
        assert_eq!(encoded, expected);
    }

    #[test]
    fn test_negative_infinity() {
        let decimal = f32::NEG_INFINITY; // A negative number too large for f32
        let encoded = encode_to_bytes(&decimal);
        let expected = 0b11111111100000000000000000000000; // Negative infinity in f32
        assert_eq!(encoded, expected);
    }
}
