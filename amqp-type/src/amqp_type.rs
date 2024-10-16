use crate::compound::array::Array;
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

#[derive(Debug, Hash, Eq, PartialEq)]
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
    pub async fn try_decode(stream: &mut Pin<Box<impl Stream<Item = u8>>>) -> Result<Self, AppError>
    where
        Self: Sized,
    {
        match stream.next().await {
            None => Err(AppError::IteratorEmptyOrTooShortError),
            Some(constructor) => Self::try_decode_with_constructor(constructor, stream).await,
        }
    }

    pub async fn try_decode_with_constructor(
        constructor: u8,
        stream: &mut Pin<Box<impl Stream<Item = u8>>>,
    ) -> Result<Self, AppError>
    where
        Self: Sized,
    {
        match constructor {
            NULL => Ok(Self::Null),
            BYTE => Ok(i8::try_decode(BYTE, stream).await?.into()),
            CHAR => Ok(char::try_decode(CHAR, stream).await?.into()),
            DECIMAL_32 => Ok(Decimal32::try_decode(DECIMAL_32, stream).await?.into()),
            DECIMAL_64 => Ok(Decimal64::try_decode(DECIMAL_64, stream).await?.into()),
            DOUBLE => Ok(Double::try_decode(DOUBLE, stream).await?.into()),
            FLOAT => Ok(Float::try_decode(FLOAT, stream).await?.into()),
            SHORT => Ok(i16::try_decode(SHORT, stream).await?.into()),
            TIMESTAMP => Ok(Timestamp::try_decode(TIMESTAMP, stream).await?.into()),
            UNSIGNED_BYTE => Ok(u8::try_decode(UNSIGNED_BYTE, stream).await?.into()),
            UNSIGNED_SHORT => Ok(u16::try_decode(UNSIGNED_SHORT, stream).await?.into()),
            UUID => Ok(Uuid::try_decode(UUID, stream).await?.into()),
            x @ (BOOLEAN | BOOLEAN_TRUE | BOOLEAN_FALSE) => {
                Ok(bool::try_decode(x, stream).await?.into())
            }
            x @ (SMALL_INTEGER | INTEGER) => Ok(i32::try_decode(x, stream).await?.into()),
            x @ (SMALL_LONG | LONG) => Ok(i64::try_decode(x, stream).await?.into()),
            x @ (UNSIGNED_INTEGER | SMALL_UNSIGNED_INTEGER | UNSIGNED_INTEGER_ZERO) => {
                Ok(u32::try_decode(x, stream).await?.into())
            }
            x @ (UNSIGNED_LONG | SMALL_UNSIGNED_LONG | UNSIGNED_LONG_ZERO) => {
                Ok(u64::try_decode(x, stream).await?.into())
            }
            x @ (ARRAY_SHORT | ARRAY) => Ok(Array::try_decode(x, stream).await?.into()),
            x @ (LIST_EMPTY | LIST_SHORT | LIST) => Ok(List::try_decode(x, stream).await?.into()),
            x @ (MAP_SHORT | MAP) => Ok(Map::try_decode(x, stream).await?.into()),
            x @ (BINARY_SHORT | BINARY) => Ok(Binary::try_decode(x, stream).await?.into()),
            x @ (STRING_SHORT | STRING) => Ok(String::try_decode(x, stream).await?.into()),
            other => Err(AppError::DeserializationIllegalConstructorError(other)),
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
    use crate::common::tests::ByteVecExt;
    use bigdecimal::BigDecimal;
    use indexmap::IndexMap;

    #[test]
    fn construct_null() {
        let val = AmqpType::Null;
        assert_eq!(val.encode().constructor(), 0x40);
    }

    #[tokio::test]
    async fn test_encode_decode_round_trip_null() {
        let before = AmqpType::Null;
        let encoded: Vec<u8> = before.encode().into();
        let decoded = AmqpType::try_decode(&mut encoded.into_pinned_stream())
            .await
            .unwrap();
        assert_eq!(decoded, before);
    }
    #[tokio::test]
    async fn test_encode_decode_round_trip_boolean() {
        let before = AmqpType::Boolean(false);
        let encoded: Vec<u8> = before.encode().into();
        let decoded = AmqpType::try_decode(&mut encoded.into_pinned_stream())
            .await
            .unwrap();
        assert_eq!(decoded, before);
    }
    #[tokio::test]
    async fn test_encode_decode_round_trip_ubyte() {
        let before = AmqpType::Ubyte(10);
        let encoded: Vec<u8> = before.encode().into();
        let decoded = AmqpType::try_decode(&mut encoded.into_pinned_stream())
            .await
            .unwrap();
        assert_eq!(decoded, before);
    }
    #[tokio::test]
    async fn test_encode_decode_round_trip_ushort() {
        let before = AmqpType::Ushort(100);
        let encoded: Vec<u8> = before.encode().into();
        let decoded = AmqpType::try_decode(&mut encoded.into_pinned_stream())
            .await
            .unwrap();
        assert_eq!(decoded, before);
    }
    #[tokio::test]
    async fn test_encode_decode_round_trip_uint() {
        let before = AmqpType::Uint(100);
        let encoded: Vec<u8> = before.encode().into();
        let decoded = AmqpType::try_decode(&mut encoded.into_pinned_stream())
            .await
            .unwrap();
        assert_eq!(decoded, before);
    }
    #[tokio::test]
    async fn test_encode_decode_round_trip_ulong() {
        let before = AmqpType::Ulong(100);
        let encoded: Vec<u8> = before.encode().into();
        let decoded = AmqpType::try_decode(&mut encoded.into_pinned_stream())
            .await
            .unwrap();
        assert_eq!(decoded, before);
    }
    #[tokio::test]
    async fn test_encode_decode_round_trip_byte() {
        let before = AmqpType::Byte(100);
        let encoded: Vec<u8> = before.encode().into();
        let decoded = AmqpType::try_decode(&mut encoded.into_pinned_stream())
            .await
            .unwrap();
        assert_eq!(decoded, before);
    }
    #[tokio::test]
    async fn test_encode_decode_round_trip_short() {
        let before = AmqpType::Short(100);
        let encoded: Vec<u8> = before.encode().into();
        let decoded = AmqpType::try_decode(&mut encoded.into_pinned_stream())
            .await
            .unwrap();
        assert_eq!(decoded, before);
    }
    #[tokio::test]
    async fn test_encode_decode_round_trip_int() {
        let before = AmqpType::Int(100);
        let encoded: Vec<u8> = before.encode().into();
        let decoded = AmqpType::try_decode(&mut encoded.into_pinned_stream())
            .await
            .unwrap();
        assert_eq!(decoded, before);
    }
    #[tokio::test]
    async fn test_encode_decode_round_trip_long() {
        let before = AmqpType::Long(100);
        let encoded: Vec<u8> = before.encode().into();
        let decoded = AmqpType::try_decode(&mut encoded.into_pinned_stream())
            .await
            .unwrap();
        assert_eq!(decoded, before);
    }
    #[tokio::test]
    async fn test_encode_decode_round_trip_float() {
        let before = AmqpType::Float(1.0.into());
        let encoded: Vec<u8> = before.encode().into();
        let decoded = AmqpType::try_decode(&mut encoded.into_pinned_stream())
            .await
            .unwrap();
        assert_eq!(decoded, before);
    }
    #[tokio::test]
    async fn test_encode_decode_round_trip_double() {
        let before = AmqpType::Double(100.0.into());
        let encoded: Vec<u8> = before.encode().into();
        let decoded = AmqpType::try_decode(&mut encoded.into_pinned_stream())
            .await
            .unwrap();
        assert_eq!(decoded, before);
    }
    #[tokio::test]
    async fn test_encode_decode_round_trip_decimal32() {
        let before = AmqpType::Decimal32(100.0.into());
        let encoded: Vec<u8> = before.encode().into();
        let decoded = AmqpType::try_decode(&mut encoded.into_pinned_stream())
            .await
            .unwrap();
        assert_eq!(decoded, before);
    }
    #[tokio::test]
    async fn test_encode_decode_round_trip_decimal64() {
        let before = AmqpType::Decimal64(100.0.into());
        let encoded: Vec<u8> = before.encode().into();
        let decoded = AmqpType::try_decode(&mut encoded.into_pinned_stream())
            .await
            .unwrap();
        assert_eq!(decoded, before);
    }
    #[tokio::test]
    #[ignore]
    // Ignored because f128 is not implemented yet
    async fn test_encode_decode_round_trip_decimal128() {
        let before = AmqpType::Decimal128(BigDecimal::from(10000u32).into());
        let encoded: Vec<u8> = before.encode().into();
        let decoded = AmqpType::try_decode(&mut encoded.into_pinned_stream())
            .await
            .unwrap();
        assert_eq!(decoded, before);
    }
    #[tokio::test]
    async fn test_encode_decode_round_trip_timestamp() {
        let before = AmqpType::Timestamp(10000.into());
        let encoded: Vec<u8> = before.encode().into();
        let decoded = AmqpType::try_decode(&mut encoded.into_pinned_stream())
            .await
            .unwrap();
        assert_eq!(decoded, before);
    }
    #[tokio::test]
    async fn test_encode_decode_round_trip_uuid() {
        let before = AmqpType::Uuid(uuid::Uuid::new_v4().into());
        let encoded: Vec<u8> = before.encode().into();
        let decoded = AmqpType::try_decode(&mut encoded.into_pinned_stream())
            .await
            .unwrap();
        assert_eq!(decoded, before);
    }
    #[tokio::test]
    async fn test_encode_decode_round_trip_binary() {
        let before = AmqpType::Binary(vec![1, 2, 3, 4, 5].into());
        let encoded: Vec<u8> = before.encode().into();
        let decoded = AmqpType::try_decode(&mut encoded.into_pinned_stream())
            .await
            .unwrap();
        assert_eq!(decoded, before);
    }
    #[tokio::test]
    async fn test_encode_decode_round_trip_string() {
        let before = AmqpType::String("Hello World".to_string());
        let encoded: Vec<u8> = before.encode().into();
        let decoded = AmqpType::try_decode(&mut encoded.into_pinned_stream())
            .await
            .unwrap();
        assert_eq!(decoded, before);
    }
    #[tokio::test]
    async fn test_encode_decode_round_trip_symbol() {
        let before = AmqpType::Symbol(Symbol::new("book:seller:entry".to_string()).unwrap());
        let encoded: Vec<u8> = before.encode().into();
        let decoded = AmqpType::try_decode(&mut encoded.into_pinned_stream())
            .await
            .unwrap();
        assert_eq!(decoded, before);
    }
    #[tokio::test]
    async fn test_encode_decode_round_trip_list() {
        let before = AmqpType::List(
            vec![
                AmqpType::String("Hello world".to_string()),
                AmqpType::Char('a'),
                AmqpType::Byte(10),
            ]
            .into(),
        );
        let encoded: Vec<u8> = before.encode().into();
        let decoded = AmqpType::try_decode(&mut encoded.into_pinned_stream())
            .await
            .unwrap();
        assert_eq!(decoded, before);
    }
    #[tokio::test]
    async fn test_encode_decode_round_trip_map() {
        let map: IndexMap<AmqpType, AmqpType> = [
            (
                AmqpType::String("Hello world".to_string()),
                AmqpType::String("Hello world".to_string()),
            ),
            (
                AmqpType::Char('a'),
                AmqpType::String("Hello world aaaaaaaaaaaaa".to_string()),
            ),
            (
                AmqpType::Byte(10),
                AmqpType::String(
                    "Hello world Mega man was here and i need a long text. anyways, moving on"
                        .to_string(),
                ),
            ),
        ]
        .into();
        let before = AmqpType::Map(map.into());
        let encoded: Vec<u8> = before.encode().into();
        let decoded = AmqpType::try_decode(&mut encoded.into_pinned_stream())
            .await
            .unwrap();
        assert_eq!(decoded, before);
    }
    #[tokio::test]
    async fn test_encode_decode_round_trip_array() {
        let before = AmqpType::Array(
            vec![
                AmqpType::Int(-100),
                AmqpType::Int(100),
                AmqpType::Int(120),
            ]
            .into(),
        );
        let encoded: Vec<u8> = before.encode().into();
        let decoded = AmqpType::try_decode(&mut encoded.into_pinned_stream())
            .await
            .unwrap();
        assert_eq!(decoded, before);
    }
}
