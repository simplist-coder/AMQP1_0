use crate::array::array::Array;
use crate::compound::list::List;
use crate::compound::map::Map;
use crate::constants::constructors::*;
use crate::error::AppError;
use crate::fixed_width::decimal128::Decimal128;
use crate::fixed_width::decimal32::Decimal32;
use crate::fixed_width::decimal64::Decimal64;
use crate::fixed_width::double::*;
use crate::fixed_width::float::Float;
use crate::fixed_width::timestamp::Timestamp;
use crate::fixed_width::uuid::Uuid;
use crate::serde::decode::Decode;
use crate::serde::encode::{Encode, Encoded};
use crate::variable_width::binary::Binary;
use crate::variable_width::symbol::Symbol;
use std::hash::Hash;
use std::pin::Pin;
use tokio_stream::{Stream, StreamExt};

#[derive(Hash, Eq, PartialEq)]
pub enum AmqpType {
    Null,
    Boolean(bool),
    Ubyte(u8),
    Ushort(u16),
    Uint(u32),
    Ulong(u64),
    Byte(i8),
    Short(i16),
    Int(i32),
    Long(i64),
    Float(Float),
    Double(Double),
    Decimal32(Decimal32),
    Decimal64(Decimal64),
    Decimal128(Decimal128),
    Char(char),
    Timestamp(Timestamp),
    Uuid(Uuid),
    Binary(Binary),
    String(String),
    Symbol(Symbol),
    List(List),
    Map(Map),
    Array(Array),
}

impl Encode for AmqpType {
    fn encode(&self) -> Encoded {
        match self {
            Self::Null => NULL.into(),
            Self::Boolean(val) => val.encode(),
            Self::Ubyte(val) => val.encode(),
            Self::Ushort(val) => val.encode(),
            Self::Uint(val) => val.encode(),
            Self::Ulong(val) => val.encode(),
            Self::Byte(val) => val.encode(),
            Self::Short(val) => val.encode(),
            Self::Int(val) => val.encode(),
            Self::Long(val) => val.encode(),
            Self::Float(val) => val.encode(),
            Self::Double(val) => val.encode(),
            Self::Decimal32(val) => val.encode(),
            Self::Decimal64(val) => val.encode(),
            Self::Decimal128(val) => val.encode(),
            Self::Char(val) => val.encode(),
            Self::Timestamp(val) => val.encode(),
            Self::Uuid(val) => val.encode(),
            Self::Binary(val) => val.encode(),
            Self::String(val) => val.encode(),
            Self::Symbol(val) => val.encode(),
            Self::List(val) => val.encode(),
            Self::Map(val) => val.encode(),
            Self::Array(val) => val.encode(),
        }
    }
}

impl AmqpType {
    #[allow(dead_code)]
    pub async fn try_decode(mut stream: Pin<Box<impl Stream<Item=u8>>>) -> Result<Self, AppError>
    where
        Self: Sized,
    {
        match stream.next().await {
            None => Err(AppError::IteratorEmptyOrTooShortError),
            Some(NULL) => Ok(Self::Null),
            Some(BOOLEAN) => Ok(bool::try_decode(BOOLEAN, stream).await?.into()),
            Some(BYTE) => Ok(i8::try_decode(BYTE, stream).await?.into()),
            Some(CHAR) => Ok(char::try_decode(CHAR, stream).await?.into()),
            Some(DECIMAL_32) => Ok(Decimal32::try_decode(DECIMAL_32, stream).await?.into()),
            Some(DECIMAL_64) => Ok(Decimal64::try_decode(DECIMAL_64, stream).await?.into()),
            Some(DOUBLE) => Ok(Double::try_decode(DOUBLE, stream).await?.into()),
            Some(FLOAT) => Ok(Float::try_decode(FLOAT, stream).await?.into()),
            Some(INTEGER) => Ok(i32::try_decode(INTEGER, stream).await?.into()),
            Some(SMALL_INTEGER) => Ok(i32::try_decode(SMALL_INTEGER, stream).await?.into()),
            Some(LONG) => Ok(i64::try_decode(LONG, stream).await?.into()),
            Some(SMALL_LONG) => Ok(i64::try_decode(SMALL_LONG, stream).await?.into()),
            Some(SHORT) => Ok(i16::try_decode(SHORT, stream).await?.into()),
            Some(TIMESTAMP) => Ok(Timestamp::try_decode(TIMESTAMP, stream).await?.into()),
            Some(UNSIGNED_BYTE) => Ok(u8::try_decode(UNSIGNED_BYTE, stream).await?.into()),
            Some(UNSIGNED_INTEGER) => Ok(u32::try_decode(UNSIGNED_INTEGER, stream).await?.into()),
            Some(SMALL_UNSIGNED_INTEGER) => Ok(u32::try_decode(SMALL_UNSIGNED_INTEGER, stream).await?.into()),
            Some(UNSIGNED_INTEGER_ZERO) => Ok(u32::try_decode(UNSIGNED_INTEGER_ZERO, stream).await?.into()),
            Some(UNSIGNED_LONG) => Ok(u64::try_decode(UNSIGNED_LONG, stream).await?.into()),
            Some(SMALL_UNSIGNED_LONG) => Ok(u64::try_decode(SMALL_UNSIGNED_LONG, stream).await?.into()),
            Some(UNSIGNED_LONG_ZERO) => Ok(u64::try_decode(UNSIGNED_LONG_ZERO, stream).await?.into()),
            Some(UNSIGNED_SHORT) => Ok(u16::try_decode(UNSIGNED_SHORT, stream).await?.into()),
            Some(UUID) => Ok(Uuid::try_decode(UUID, stream).await?.into()),
            Some(ARRAY_SHORT) => Ok(Array::try_decode(ARRAY_SHORT, stream).await?.into()),
            Some(ARRAY) => Ok(Array::try_decode(ARRAY, stream).await?.into()),
            Some(LIST_SHORT) => Ok(List::try_decode(LIST_SHORT, stream).await?.into()),
            Some(LIST) => Ok(List::try_decode(LIST, stream).await?.into()),
            Some(MAP_SHORT) => Ok(Map::try_decode(MAP_SHORT, stream).await?.into()),
            Some(MAP) => Ok(Map::try_decode(MAP, stream).await?.into()),
            Some(BINARY_SHORT) => Ok(Binary::try_decode(BINARY_SHORT, stream).await?.into()),
            Some(BINARY) => Ok(Binary::try_decode(BINARY, stream).await?.into()),
            Some(STRING_SHORT) => Ok(String::try_decode(STRING_SHORT, stream).await?.into()),
            Some(STRING) => Ok(String::try_decode(STRING, stream).await?.into()),
            #[cfg(feature = "zero-length-encoding")]
            Some(&BOOLEAN_TRUE) => Ok(Self::Boolean(true)),
            #[cfg(feature = "zero-length-encoding")]
            Some(&BOOLEAN_FALSE) => Ok(Self::Boolean(false)),
            Some(other) => Err(AppError::DeserializationIllegalConstructorError(other))
        }
    }
}

impl From<Option<AmqpType>> for AmqpType {
    fn from(value: Option<AmqpType>) -> Self {
        value.unwrap_or_else(|| AmqpType::Null)
    }
}

impl From<bool> for AmqpType {
    fn from(value: bool) -> Self {
        AmqpType::Boolean(value)
    }
}

impl From<Timestamp> for AmqpType {
    fn from(value: Timestamp) -> Self {
        AmqpType::Timestamp(value)
    }
}

impl From<u8> for AmqpType {
    fn from(value: u8) -> Self {
        AmqpType::Ubyte(value)
    }
}

impl From<u16> for AmqpType {
    fn from(value: u16) -> Self {
        AmqpType::Ushort(value)
    }
}

impl From<u32> for AmqpType {
    fn from(value: u32) -> Self {
        AmqpType::Uint(value)
    }
}

impl From<u64> for AmqpType {
    fn from(value: u64) -> Self {
        AmqpType::Ulong(value)
    }
}

impl From<i8> for AmqpType {
    fn from(value: i8) -> Self {
        AmqpType::Byte(value)
    }
}

impl From<i16> for AmqpType {
    fn from(value: i16) -> Self {
        AmqpType::Short(value)
    }
}

impl From<i32> for AmqpType {
    fn from(value: i32) -> Self {
        AmqpType::Int(value)
    }
}

impl From<i64> for AmqpType {
    fn from(value: i64) -> Self {
        AmqpType::Long(value)
    }
}

impl From<Float> for AmqpType {
    fn from(value: Float) -> Self {
        AmqpType::Float(value.into())
    }
}

impl From<Double> for AmqpType {
    fn from(value: Double) -> Self {
        AmqpType::Double(value.into())
    }
}

impl From<char> for AmqpType {
    fn from(value: char) -> Self {
        AmqpType::Char(value)
    }
}

impl From<Uuid> for AmqpType {
    fn from(value: Uuid) -> Self {
        AmqpType::Uuid(value)
    }
}

impl From<Binary> for AmqpType {
    fn from(value: Binary) -> Self {
        AmqpType::Binary(value)
    }
}

impl From<&str> for AmqpType {
    fn from(value: &str) -> Self {
        value.to_string().into()
    }
}

impl From<String> for AmqpType {
    fn from(value: String) -> Self {
        AmqpType::String(value)
    }
}

impl From<Symbol> for AmqpType {
    fn from(value: Symbol) -> Self {
        AmqpType::Symbol(value)
    }
}

impl From<List> for AmqpType {
    fn from(value: List) -> Self {
        AmqpType::List(value)
    }
}

impl From<Map> for AmqpType {
    fn from(value: Map) -> Self {
        AmqpType::Map(value)
    }
}

impl From<Array> for AmqpType {
    fn from(value: Array) -> Self {
        AmqpType::Array(value)
    }
}

impl From<Decimal32> for AmqpType {
    fn from(value: Decimal32) -> Self {
        AmqpType::Decimal32(value)
    }
}

impl From<Decimal64> for AmqpType {
    fn from(value: Decimal64) -> Self {
        AmqpType::Decimal64(value)
    }
}

impl From<Decimal128> for AmqpType {
    fn from(value: Decimal128) -> Self {
        AmqpType::Decimal128(value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn construct_null() {
        let val = AmqpType::Null;
        assert_eq!(val.encode().constructor(), 0x40);
    }
}
