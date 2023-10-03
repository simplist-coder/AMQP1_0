use bigdecimal::BigDecimal;

use crate::serde::encode::{Encode, Encoded};

#[derive(Hash, Eq, PartialEq)]
pub struct Decimal32(BigDecimal);

impl Encode for Decimal32 {
    fn encode(&self) -> Encoded {
        0x74.into()
    }
}
impl From<f32> for Decimal32 {
    fn from(value: f32) -> Self {
        Decimal32(BigDecimal::try_from(value).unwrap())
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
}
