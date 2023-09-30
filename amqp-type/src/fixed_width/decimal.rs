use bigdecimal::BigDecimal;

use crate::serde::encode::{Encode, Encoded};

#[derive(Hash, Eq, PartialEq)]
pub struct Decimal32(BigDecimal);
#[derive(Hash, Eq, PartialEq)]
pub struct Decimal64(BigDecimal);
#[derive(Hash, Eq, PartialEq)]
pub struct Decimal128(BigDecimal);

impl Encode for Decimal32 {
    fn encode(&self) -> Encoded {
        0x74.into()
    }
}

impl Encode for Decimal64 {
    fn encode(&self) -> Encoded {
        0x84.into()
    }
}

impl Encode for Decimal128 {
    fn encode(&self) -> Encoded {
        0x94.into()
    }
}

impl From<f32> for Decimal32 {
    fn from(value: f32) -> Self {
        Decimal32(BigDecimal::try_from(value).unwrap())
    }
}

impl From<f64> for Decimal64 {
    fn from(value: f64) -> Self {
        Decimal64(BigDecimal::try_from(value).unwrap())
    }
}

impl From<f64> for Decimal128 {
    fn from(value: f64) -> Self {
        Decimal128(BigDecimal::try_from(value).unwrap())
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn construct_decimal_32() {
        let val: Decimal32 = 32.0.into();
        assert_eq!(val.encode().constructor(), 0x74);
    }

    #[test]
    fn construct_decimal_64() {
        let val: Decimal64 = 64.0.into();
        assert_eq!(val.encode().constructor(), 0x84);
    }

    #[test]
    fn construct_decimal_128() {
        let val: Decimal128 = 128.0.into();
        assert_eq!(val.encode().constructor(), 0x94);
    }
}
