use crate::constants::constructors::*;
use crate::error::AppError;
use crate::primitive::compound::array::Array;
use crate::primitive::compound::list::List;
use crate::primitive::compound::map::Map;
use crate::primitive::fixed_width::decimal128::Decimal128;
use crate::primitive::fixed_width::decimal32::Decimal32;
use crate::primitive::fixed_width::decimal64::Decimal64;
use crate::primitive::fixed_width::double::*;
use crate::primitive::fixed_width::float::Float;
use crate::primitive::fixed_width::timestamp::Timestamp;
use crate::primitive::fixed_width::uuid::Uuid;
use crate::primitive::variable_width::binary::Binary;
use crate::primitive::variable_width::symbol::Symbol;
use crate::serde::decode::Decode;
use crate::serde::encode::{Encode, Encoded};
use std::hash::Hash;
use std::pin::Pin;
use tokio_stream::{Stream, StreamExt};

#[derive(Debug, Hash, Eq, PartialEq)]
pub enum Primitive {
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

impl Encode for Primitive {
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

impl Primitive {
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
            x @ (SYMBOL | SYMBOL_SHORT) => Ok(Symbol::try_decode(x, stream).await?.into()),
            other => Err(AppError::DeserializationIllegalConstructorError(other)),
        }
    }
}

impl From<Option<Primitive>> for Primitive {
    fn from(value: Option<Primitive>) -> Self {
        value.unwrap_or_else(|| Primitive::Null)
    }
}

impl From<bool> for Primitive {
    fn from(value: bool) -> Self {
        Primitive::Boolean(value)
    }
}

impl From<Timestamp> for Primitive {
    fn from(value: Timestamp) -> Self {
        Primitive::Timestamp(value)
    }
}

impl From<u8> for Primitive {
    fn from(value: u8) -> Self {
        Primitive::Ubyte(value)
    }
}

impl From<u16> for Primitive {
    fn from(value: u16) -> Self {
        Primitive::Ushort(value)
    }
}

impl From<u32> for Primitive {
    fn from(value: u32) -> Self {
        Primitive::Uint(value)
    }
}

impl From<u64> for Primitive {
    fn from(value: u64) -> Self {
        Primitive::Ulong(value)
    }
}

impl From<i8> for Primitive {
    fn from(value: i8) -> Self {
        Primitive::Byte(value)
    }
}

impl From<i16> for Primitive {
    fn from(value: i16) -> Self {
        Primitive::Short(value)
    }
}

impl From<i32> for Primitive {
    fn from(value: i32) -> Self {
        Primitive::Int(value)
    }
}

impl From<i64> for Primitive {
    fn from(value: i64) -> Self {
        Primitive::Long(value)
    }
}

impl From<Float> for Primitive {
    fn from(value: Float) -> Self {
        Primitive::Float(value.into())
    }
}

impl From<Double> for Primitive {
    fn from(value: Double) -> Self {
        Primitive::Double(value.into())
    }
}

impl From<char> for Primitive {
    fn from(value: char) -> Self {
        Primitive::Char(value)
    }
}

impl From<Uuid> for Primitive {
    fn from(value: Uuid) -> Self {
        Primitive::Uuid(value)
    }
}

impl From<Binary> for Primitive {
    fn from(value: Binary) -> Self {
        Primitive::Binary(value)
    }
}

impl From<&str> for Primitive {
    fn from(value: &str) -> Self {
        value.to_string().into()
    }
}

impl From<String> for Primitive {
    fn from(value: String) -> Self {
        Primitive::String(value)
    }
}

impl From<Symbol> for Primitive {
    fn from(value: Symbol) -> Self {
        Primitive::Symbol(value)
    }
}

impl From<List> for Primitive {
    fn from(value: List) -> Self {
        Primitive::List(value)
    }
}

impl From<Map> for Primitive {
    fn from(value: Map) -> Self {
        Primitive::Map(value)
    }
}

impl From<Array> for Primitive {
    fn from(value: Array) -> Self {
        Primitive::Array(value)
    }
}

impl From<Decimal32> for Primitive {
    fn from(value: Decimal32) -> Self {
        Primitive::Decimal32(value)
    }
}

impl From<Decimal64> for Primitive {
    fn from(value: Decimal64) -> Self {
        Primitive::Decimal64(value)
    }
}

impl From<Decimal128> for Primitive {
    fn from(value: Decimal128) -> Self {
        Primitive::Decimal128(value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::tests::ByteVecExt;
    use indexmap::IndexMap;

    #[test]
    fn construct_null() {
        let val = Primitive::Null;
        assert_eq!(val.encode().constructor(), 0x40);
    }

    #[tokio::test]
    async fn test_encode_decode_round_trip_null() {
        let before = Primitive::Null;
        let encoded: Vec<u8> = before.encode().into();
        let decoded = Primitive::try_decode(&mut encoded.into_pinned_stream())
            .await
            .unwrap();
        assert_eq!(decoded, before);
    }
    #[tokio::test]
    async fn test_encode_decode_round_trip_boolean() {
        let before = Primitive::Boolean(false);
        let encoded: Vec<u8> = before.encode().into();
        let decoded = Primitive::try_decode(&mut encoded.into_pinned_stream())
            .await
            .unwrap();
        assert_eq!(decoded, before);
    }
    #[tokio::test]
    async fn test_encode_decode_round_trip_ubyte() {
        let before = Primitive::Ubyte(10);
        let encoded: Vec<u8> = before.encode().into();
        let decoded = Primitive::try_decode(&mut encoded.into_pinned_stream())
            .await
            .unwrap();
        assert_eq!(decoded, before);
    }
    #[tokio::test]
    async fn test_encode_decode_round_trip_ushort() {
        let before = Primitive::Ushort(100);
        let encoded: Vec<u8> = before.encode().into();
        let decoded = Primitive::try_decode(&mut encoded.into_pinned_stream())
            .await
            .unwrap();
        assert_eq!(decoded, before);
    }
    #[tokio::test]
    async fn test_encode_decode_round_trip_uint() {
        let before = Primitive::Uint(100);
        let encoded: Vec<u8> = before.encode().into();
        let decoded = Primitive::try_decode(&mut encoded.into_pinned_stream())
            .await
            .unwrap();
        assert_eq!(decoded, before);
    }
    #[tokio::test]
    async fn test_encode_decode_round_trip_ulong() {
        let before = Primitive::Ulong(100);
        let encoded: Vec<u8> = before.encode().into();
        let decoded = Primitive::try_decode(&mut encoded.into_pinned_stream())
            .await
            .unwrap();
        assert_eq!(decoded, before);
    }
    #[tokio::test]
    async fn test_encode_decode_round_trip_byte() {
        let before = Primitive::Byte(100);
        let encoded: Vec<u8> = before.encode().into();
        let decoded = Primitive::try_decode(&mut encoded.into_pinned_stream())
            .await
            .unwrap();
        assert_eq!(decoded, before);
    }
    #[tokio::test]
    async fn test_encode_decode_round_trip_short() {
        let before = Primitive::Short(100);
        let encoded: Vec<u8> = before.encode().into();
        let decoded = Primitive::try_decode(&mut encoded.into_pinned_stream())
            .await
            .unwrap();
        assert_eq!(decoded, before);
    }
    #[tokio::test]
    async fn test_encode_decode_round_trip_int() {
        let before = Primitive::Int(100);
        let encoded: Vec<u8> = before.encode().into();
        let decoded = Primitive::try_decode(&mut encoded.into_pinned_stream())
            .await
            .unwrap();
        assert_eq!(decoded, before);
    }
    #[tokio::test]
    async fn test_encode_decode_round_trip_long() {
        let before = Primitive::Long(100);
        let encoded: Vec<u8> = before.encode().into();
        let decoded = Primitive::try_decode(&mut encoded.into_pinned_stream())
            .await
            .unwrap();
        assert_eq!(decoded, before);
    }
    #[tokio::test]
    async fn test_encode_decode_round_trip_float() {
        let before = Primitive::Float(1.0.into());
        let encoded: Vec<u8> = before.encode().into();
        let decoded = Primitive::try_decode(&mut encoded.into_pinned_stream())
            .await
            .unwrap();
        assert_eq!(decoded, before);
    }
    #[tokio::test]
    async fn test_encode_decode_round_trip_double() {
        let before = Primitive::Double(100.0.into());
        let encoded: Vec<u8> = before.encode().into();
        let decoded = Primitive::try_decode(&mut encoded.into_pinned_stream())
            .await
            .unwrap();
        assert_eq!(decoded, before);
    }
    #[tokio::test]
    async fn test_encode_decode_round_trip_decimal32() {
        let before = Primitive::Decimal32(100.0.into());
        let encoded: Vec<u8> = before.encode().into();
        let decoded = Primitive::try_decode(&mut encoded.into_pinned_stream())
            .await
            .unwrap();
        assert_eq!(decoded, before);
    }
    #[tokio::test]
    async fn test_encode_decode_round_trip_decimal64() {
        let before = Primitive::Decimal64(100.0.into());
        let encoded: Vec<u8> = before.encode().into();
        let decoded = Primitive::try_decode(&mut encoded.into_pinned_stream())
            .await
            .unwrap();
        assert_eq!(decoded, before);
    }
    #[tokio::test]
    #[ignore]
    // Ignored because f128 is not implemented yet
    async fn test_encode_decode_round_trip_decimal128() {
        let before = Primitive::Decimal128(Decimal128());
        let encoded: Vec<u8> = before.encode().into();
        let decoded = Primitive::try_decode(&mut encoded.into_pinned_stream())
            .await
            .unwrap();
        assert_eq!(decoded, before);
    }
    #[tokio::test]
    async fn test_encode_decode_round_trip_timestamp() {
        let before = Primitive::Timestamp(10000.into());
        let encoded: Vec<u8> = before.encode().into();
        let decoded = Primitive::try_decode(&mut encoded.into_pinned_stream())
            .await
            .unwrap();
        assert_eq!(decoded, before);
    }
    #[tokio::test]
    async fn test_encode_decode_round_trip_uuid() {
        let before = Primitive::Uuid(uuid::Uuid::new_v4().into());
        let encoded: Vec<u8> = before.encode().into();
        let decoded = Primitive::try_decode(&mut encoded.into_pinned_stream())
            .await
            .unwrap();
        assert_eq!(decoded, before);
    }
    #[tokio::test]
    async fn test_encode_decode_round_trip_binary() {
        let before = Primitive::Binary(vec![1, 2, 3, 4, 5].into());
        let encoded: Vec<u8> = before.encode().into();
        let decoded = Primitive::try_decode(&mut encoded.into_pinned_stream())
            .await
            .unwrap();
        assert_eq!(decoded, before);
    }
    #[tokio::test]
    async fn test_encode_decode_round_trip_string() {
        let before = Primitive::String("Hello World".to_string());
        let encoded: Vec<u8> = before.encode().into();
        let decoded = Primitive::try_decode(&mut encoded.into_pinned_stream())
            .await
            .unwrap();
        assert_eq!(decoded, before);
    }
    #[tokio::test]
    async fn test_encode_decode_round_trip_symbol() {
        let before = Primitive::Symbol(Symbol::new("book:seller:entry".to_string()).unwrap());
        let encoded: Vec<u8> = before.encode().into();
        let decoded = Primitive::try_decode(&mut encoded.into_pinned_stream())
            .await
            .unwrap();
        assert_eq!(decoded, before);
    }
    #[tokio::test]
    async fn test_encode_decode_round_trip_list() {
        let before = Primitive::List(
            vec![
                Primitive::String("Hello world".to_string()),
                Primitive::Char('a'),
                Primitive::Byte(10),
            ]
            .into(),
        );
        let encoded: Vec<u8> = before.encode().into();
        let decoded = Primitive::try_decode(&mut encoded.into_pinned_stream())
            .await
            .unwrap();
        assert_eq!(decoded, before);
    }
    #[tokio::test]
    async fn test_encode_decode_round_trip_map() {
        let map: IndexMap<Primitive, Primitive> = [
            (
                Primitive::String("Hello world".to_string()),
                Primitive::String("Hello world".to_string()),
            ),
            (
                Primitive::Char('a'),
                Primitive::String("Hello world aaaaaaaaaaaaa".to_string()),
            ),
            (
                Primitive::Byte(10),
                Primitive::String(
                    "Hello world Mega man was here and i need a long text. anyways, moving on"
                        .to_string(),
                ),
            ),
        ]
        .into();
        let before = Primitive::Map(map.into());
        let encoded: Vec<u8> = before.encode().into();
        let decoded = Primitive::try_decode(&mut encoded.into_pinned_stream())
            .await
            .unwrap();
        assert_eq!(decoded, before);
    }
    #[tokio::test]
    async fn test_encode_decode_round_trip_array() {
        let before = Primitive::Array(
            vec![
                Primitive::Int(-100),
                Primitive::Int(100),
                Primitive::Int(120),
            ]
            .into(),
        );
        let encoded: Vec<u8> = before.encode().into();
        let decoded = Primitive::try_decode(&mut encoded.into_pinned_stream())
            .await
            .unwrap();
        assert_eq!(decoded, before);
    }
}
