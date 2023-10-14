use bigdecimal::BigDecimal;

use crate::serde::encode::{Encode, Encoded};

#[derive(Hash, Eq, PartialEq)]
pub struct Decimal128(BigDecimal);

impl Encode for Decimal128 {
    fn encode(&self) -> Encoded {
        0x94.into()
    }
}

impl From<f64> for Decimal128 {
    fn from(value: f64) -> Self {
        Decimal128(BigDecimal::try_from(value).unwrap())
    }
}

#[derive(thiserror::Error, Debug)]
pub enum Decimal128ConversionError {
    #[error("Coefficient is too large for Decimal128 representation.")]
    CoefficientTooLarge,
    #[error("Exponent overflowed in Decimal128 representation")]
    ExponentOverflow,
    #[error("Exponent underflowed in Decimal128 representation")]
    ExponentUnderflow,
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn construct_decimal_128() {
        let val: Decimal128 = 128.0.into();
        assert_eq!(val.encode().constructor(), 0x94);
    }
}
