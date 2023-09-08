use bigdecimal::BigDecimal;
use crate::types::amqp_type::{Encode, Constructor};


#[derive(Hash, Eq, PartialEq)]
pub struct Decimal32(BigDecimal);
#[derive(Hash, Eq, PartialEq)]
pub struct Decimal64(BigDecimal);
#[derive(Hash, Eq, PartialEq)]
pub struct Decimal128(BigDecimal);



impl Encode for Decimal32 {
    fn constructor(&self) -> Constructor {
        0x74.into()
    }

    fn encode(&self) -> Vec<u8> {
        todo!()
    }
}

impl Encode for Decimal64 {
    fn constructor(&self) -> Constructor {
        0x84.into()
    }

    fn encode(&self) -> Vec<u8> {
        todo!()
    }
}

impl Encode for Decimal128 {
    fn constructor(&self) -> Constructor {
        0x94.into()
    }

    fn encode(&self) -> Vec<u8> {
        todo!()
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
