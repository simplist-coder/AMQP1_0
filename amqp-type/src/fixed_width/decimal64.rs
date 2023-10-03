use bigdecimal::BigDecimal;

use crate::serde::encode::{Encode, Encoded};

#[derive(Hash, Eq, PartialEq)]
pub struct Decimal64(BigDecimal);

impl Encode for Decimal64 {
    fn encode(&self) -> Encoded {
        0x84.into()
    }
}

impl From<f64> for Decimal64 {
    fn from(value: f64) -> Self {
        Decimal64(BigDecimal::try_from(value).unwrap())
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn construct_decimal_64() {
        let val: Decimal64 = 64.0.into();
        assert_eq!(val.encode().constructor(), 0x84);
    }
}
