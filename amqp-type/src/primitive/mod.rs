pub mod composite;
pub mod compound;
pub mod fixed_width;
pub mod variable_width;

use crate::constants::{
    ARRAY, ARRAY_SHORT, BINARY, BINARY_SHORT, BOOLEAN, BOOLEAN_FALSE, BOOLEAN_TRUE, BYTE, CHAR,
    DECIMAL_32, DECIMAL_64, DESCRIBED_TYPE, DOUBLE, FLOAT, INTEGER, LIST, LIST_EMPTY, LIST_SHORT,
    LONG, MAP, MAP_SHORT, NULL, SHORT, SMALL_INTEGER, SMALL_LONG, SMALL_UNSIGNED_INTEGER,
    SMALL_UNSIGNED_LONG, STRING, STRING_SHORT, SYMBOL, SYMBOL_SHORT, TIMESTAMP, UNSIGNED_BYTE,
    UNSIGNED_INTEGER, UNSIGNED_INTEGER_ZERO, UNSIGNED_LONG, UNSIGNED_LONG_ZERO, UNSIGNED_SHORT,
    UUID,
};
use crate::error::amqp_error::AmqpError;
use crate::error::AppError;
use crate::primitive::composite::Composite;
use crate::primitive::compound::array::Array;
use crate::primitive::compound::list::List;
use crate::primitive::compound::map::Map;
use crate::primitive::fixed_width::decimal128::Decimal128;
use crate::primitive::fixed_width::decimal32::Decimal32;
use crate::primitive::fixed_width::decimal64::Decimal64;
use crate::primitive::fixed_width::double::Double;
use crate::primitive::fixed_width::float::Float;
use crate::primitive::fixed_width::timestamp::Timestamp;
use crate::primitive::fixed_width::uuid::Uuid;
use crate::primitive::variable_width::binary::Binary;
use crate::primitive::variable_width::symbol::Symbol;
use crate::serde::decode::Decode;
use crate::serde::encode::{Encode, Encoded};
use indexmap::IndexMap;
use std::hash::Hash;
use std::vec::IntoIter;

#[derive(Debug, Hash, Eq, PartialEq, Clone)]
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
    Composite(Composite),
}

impl Encode for Primitive {
    fn encode(self) -> Encoded {
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
            Self::Composite(val) => val.encode(),
        }
    }
}

impl Primitive {
    pub fn try_decode(stream: &mut IntoIter<u8>) -> Result<Self, AppError>
    where
        Self: Sized,
    {
        match stream.next() {
            None => Err(AmqpError::DecodeError)?,
            Some(constructor) => Self::try_decode_with_constructor(constructor, stream),
        }
    }

    pub fn try_decode_with_constructor(
        constructor: u8,
        stream: &mut IntoIter<u8>,
    ) -> Result<Self, AppError>
    where
        Self: Sized,
    {
        match constructor {
            NULL => Ok(Self::Null),
            BYTE => Ok(i8::try_decode(BYTE, stream)?.into()),
            CHAR => Ok(char::try_decode(CHAR, stream)?.into()),
            DECIMAL_32 => Ok(Decimal32::try_decode(DECIMAL_32, stream)?.into()),
            DECIMAL_64 => Ok(Decimal64::try_decode(DECIMAL_64, stream)?.into()),
            DOUBLE => Ok(Double::try_decode(DOUBLE, stream)?.into()),
            FLOAT => Ok(Float::try_decode(FLOAT, stream)?.into()),
            SHORT => Ok(i16::try_decode(SHORT, stream)?.into()),
            TIMESTAMP => Ok(Timestamp::try_decode(TIMESTAMP, stream)?.into()),
            UNSIGNED_BYTE => Ok(u8::try_decode(UNSIGNED_BYTE, stream)?.into()),
            UNSIGNED_SHORT => Ok(u16::try_decode(UNSIGNED_SHORT, stream)?.into()),
            UUID => Ok(Uuid::try_decode(UUID, stream)?.into()),
            x @ (BOOLEAN | BOOLEAN_TRUE | BOOLEAN_FALSE) => Ok(bool::try_decode(x, stream)?.into()),
            x @ (SMALL_INTEGER | INTEGER) => Ok(i32::try_decode(x, stream)?.into()),
            x @ (SMALL_LONG | LONG) => Ok(i64::try_decode(x, stream)?.into()),
            x @ (UNSIGNED_INTEGER | SMALL_UNSIGNED_INTEGER | UNSIGNED_INTEGER_ZERO) => {
                Ok(u32::try_decode(x, stream)?.into())
            }
            x @ (UNSIGNED_LONG | SMALL_UNSIGNED_LONG | UNSIGNED_LONG_ZERO) => {
                Ok(u64::try_decode(x, stream)?.into())
            }
            x @ (ARRAY_SHORT | ARRAY) => Ok(Array::try_decode(x, stream)?.into()),
            x @ (LIST_EMPTY | LIST_SHORT | LIST) => Ok(List::try_decode(x, stream)?.into()),
            x @ (MAP_SHORT | MAP) => Ok(Map::try_decode(x, stream)?.into()),
            x @ (BINARY_SHORT | BINARY) => Ok(Binary::try_decode(x, stream)?.into()),
            x @ (STRING_SHORT | STRING) => Ok(String::try_decode(x, stream)?.into()),
            x @ (SYMBOL | SYMBOL_SHORT) => Ok(Symbol::try_decode(x, stream)?.into()),
            DESCRIBED_TYPE => Ok(Composite::try_decode(constructor, stream)?.into()),
            _ => Err(AmqpError::DecodeError)?,
        }
    }
}

impl<T> From<Option<T>> for Primitive
where
    T: Into<Primitive>,
{
    fn from(value: Option<T>) -> Self {
        match value {
            None => Primitive::Null,
            Some(val) => val.into(),
        }
    }
}

impl From<bool> for Primitive {
    fn from(value: bool) -> Self {
        Primitive::Boolean(value)
    }
}

impl TryFrom<Primitive> for bool {
    type Error = AppError;

    fn try_from(value: Primitive) -> Result<Self, Self::Error> {
        match value {
            Primitive::Boolean(v) => Ok(v),
            _ => Err(AmqpError::DecodeError)?,
        }
    }
}

impl From<Timestamp> for Primitive {
    fn from(value: Timestamp) -> Self {
        Primitive::Timestamp(value)
    }
}

impl TryFrom<Primitive> for Timestamp {
    type Error = AppError;

    fn try_from(value: Primitive) -> Result<Self, Self::Error> {
        match value {
            Primitive::Timestamp(v) => Ok(v),
            _ => Err(AmqpError::DecodeError)?,
        }
    }
}

impl From<u8> for Primitive {
    fn from(value: u8) -> Self {
        Primitive::Ubyte(value)
    }
}

impl TryFrom<Primitive> for u8 {
    type Error = AppError;

    fn try_from(value: Primitive) -> Result<Self, Self::Error> {
        match value {
            Primitive::Ubyte(v) => Ok(v),
            _ => Err(AmqpError::DecodeError)?,
        }
    }
}

impl From<u16> for Primitive {
    fn from(value: u16) -> Self {
        Primitive::Ushort(value)
    }
}

impl TryFrom<Primitive> for u16 {
    type Error = AppError;

    fn try_from(value: Primitive) -> Result<Self, Self::Error> {
        match value {
            Primitive::Ushort(v) => Ok(v),
            _ => Err(AmqpError::DecodeError)?,
        }
    }
}

impl From<u32> for Primitive {
    fn from(value: u32) -> Self {
        Primitive::Uint(value)
    }
}

impl TryFrom<Primitive> for u32 {
    type Error = AppError;

    fn try_from(value: Primitive) -> Result<Self, Self::Error> {
        match value {
            Primitive::Uint(v) => Ok(v),
            _ => Err(AmqpError::DecodeError)?,
        }
    }
}

impl From<u64> for Primitive {
    fn from(value: u64) -> Self {
        Primitive::Ulong(value)
    }
}

impl TryFrom<Primitive> for u64 {
    type Error = AppError;

    fn try_from(value: Primitive) -> Result<Self, Self::Error> {
        match value {
            Primitive::Ulong(v) => Ok(v),
            _ => Err(AmqpError::DecodeError)?,
        }
    }
}

impl From<i8> for Primitive {
    fn from(value: i8) -> Self {
        Primitive::Byte(value)
    }
}

impl TryFrom<Primitive> for i8 {
    type Error = AppError;

    fn try_from(value: Primitive) -> Result<Self, Self::Error> {
        match value {
            Primitive::Byte(v) => Ok(v),
            _ => Err(AmqpError::DecodeError)?,
        }
    }
}

impl From<i16> for Primitive {
    fn from(value: i16) -> Self {
        Primitive::Short(value)
    }
}

impl TryFrom<Primitive> for i16 {
    type Error = AppError;

    fn try_from(value: Primitive) -> Result<Self, Self::Error> {
        match value {
            Primitive::Short(v) => Ok(v),
            _ => Err(AmqpError::DecodeError)?,
        }
    }
}

impl From<i32> for Primitive {
    fn from(value: i32) -> Self {
        Primitive::Int(value)
    }
}

impl TryFrom<Primitive> for i32 {
    type Error = AppError;

    fn try_from(value: Primitive) -> Result<Self, Self::Error> {
        match value {
            Primitive::Int(v) => Ok(v),
            _ => Err(AmqpError::DecodeError)?,
        }
    }
}

impl From<i64> for Primitive {
    fn from(value: i64) -> Self {
        Primitive::Long(value)
    }
}

impl TryFrom<Primitive> for i64 {
    type Error = AppError;

    fn try_from(value: Primitive) -> Result<Self, Self::Error> {
        match value {
            Primitive::Long(v) => Ok(v),
            _ => Err(AmqpError::DecodeError)?,
        }
    }
}

impl From<Float> for Primitive {
    fn from(value: Float) -> Self {
        Primitive::Float(value)
    }
}

impl TryFrom<Primitive> for Float {
    type Error = AppError;

    fn try_from(value: Primitive) -> Result<Self, Self::Error> {
        match value {
            Primitive::Float(v) => Ok(v),
            _ => Err(AmqpError::DecodeError)?,
        }
    }
}

impl From<Double> for Primitive {
    fn from(value: Double) -> Self {
        Primitive::Double(value)
    }
}

impl TryFrom<Primitive> for Double {
    type Error = AppError;

    fn try_from(value: Primitive) -> Result<Self, Self::Error> {
        match value {
            Primitive::Double(v) => Ok(v),
            _ => Err(AmqpError::DecodeError)?,
        }
    }
}

impl From<char> for Primitive {
    fn from(value: char) -> Self {
        Primitive::Char(value)
    }
}

impl TryFrom<Primitive> for char {
    type Error = AppError;

    fn try_from(value: Primitive) -> Result<Self, Self::Error> {
        match value {
            Primitive::Char(v) => Ok(v),
            _ => Err(AmqpError::DecodeError)?,
        }
    }
}

impl From<Uuid> for Primitive {
    fn from(value: Uuid) -> Self {
        Primitive::Uuid(value)
    }
}

impl TryFrom<Primitive> for Uuid {
    type Error = AppError;

    fn try_from(value: Primitive) -> Result<Self, Self::Error> {
        match value {
            Primitive::Uuid(v) => Ok(v),
            _ => Err(AmqpError::DecodeError)?,
        }
    }
}

impl From<Binary> for Primitive {
    fn from(value: Binary) -> Self {
        Primitive::Binary(value)
    }
}

impl TryFrom<Primitive> for Binary {
    type Error = AppError;

    fn try_from(value: Primitive) -> Result<Self, Self::Error> {
        match value {
            Primitive::Binary(v) => Ok(v),
            _ => Err(AmqpError::DecodeError)?,
        }
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

impl TryFrom<Primitive> for String {
    type Error = AppError;

    fn try_from(value: Primitive) -> Result<Self, Self::Error> {
        match value {
            Primitive::String(v) => Ok(v),
            _ => Err(AmqpError::DecodeError)?,
        }
    }
}

impl From<Symbol> for Primitive {
    fn from(value: Symbol) -> Self {
        Primitive::Symbol(value)
    }
}

impl TryFrom<Primitive> for Symbol {
    type Error = AppError;

    fn try_from(value: Primitive) -> Result<Self, Self::Error> {
        match value {
            Primitive::Symbol(v) => Ok(v),
            _ => Err(AmqpError::DecodeError)?,
        }
    }
}

impl From<List> for Primitive {
    fn from(value: List) -> Self {
        Primitive::List(value)
    }
}

impl TryFrom<Primitive> for List {
    type Error = AppError;

    fn try_from(value: Primitive) -> Result<Self, Self::Error> {
        match value {
            Primitive::List(v) => Ok(v),
            _ => Err(AmqpError::DecodeError)?,
        }
    }
}

impl From<Map> for Primitive {
    fn from(value: Map) -> Self {
        Primitive::Map(value)
    }
}

impl TryFrom<Primitive> for Map {
    type Error = AppError;

    fn try_from(value: Primitive) -> Result<Self, Self::Error> {
        match value {
            Primitive::Map(v) => Ok(v),
            _ => Err(AmqpError::DecodeError)?,
        }
    }
}

impl From<Array> for Primitive {
    fn from(value: Array) -> Self {
        Primitive::Array(value)
    }
}

impl TryFrom<Primitive> for Array {
    type Error = AppError;

    fn try_from(value: Primitive) -> Result<Self, Self::Error> {
        match value {
            Primitive::Array(v) => Ok(v),
            _ => Err(AmqpError::DecodeError)?,
        }
    }
}

impl From<Decimal32> for Primitive {
    fn from(value: Decimal32) -> Self {
        Primitive::Decimal32(value)
    }
}

impl TryFrom<Primitive> for Decimal32 {
    type Error = AppError;

    fn try_from(value: Primitive) -> Result<Self, Self::Error> {
        match value {
            Primitive::Decimal32(v) => Ok(v),
            _ => Err(AmqpError::DecodeError)?,
        }
    }
}

impl From<Decimal64> for Primitive {
    fn from(value: Decimal64) -> Self {
        Primitive::Decimal64(value)
    }
}

impl TryFrom<Primitive> for Decimal64 {
    type Error = AppError;

    fn try_from(value: Primitive) -> Result<Self, Self::Error> {
        match value {
            Primitive::Decimal64(v) => Ok(v),
            _ => Err(AmqpError::DecodeError)?,
        }
    }
}

impl From<Decimal128> for Primitive {
    fn from(value: Decimal128) -> Self {
        Primitive::Decimal128(value)
    }
}

impl TryFrom<Primitive> for Decimal128 {
    type Error = AppError;

    fn try_from(_value: Primitive) -> Result<Self, Self::Error> {
        todo!("This is not implemented yet")
    }
}

impl From<Composite> for Primitive {
    fn from(value: Composite) -> Self {
        Primitive::Composite(value)
    }
}

impl<K, V> From<IndexMap<K, V>> for Primitive
where
    K: Into<Primitive>,
    V: Into<Primitive>,
{
    fn from(value: IndexMap<K, V>) -> Self {
        Primitive::Map(Map::from(value))
    }
}

impl<K, V> TryFrom<Primitive> for IndexMap<K, V>
where
    K: TryFrom<Primitive> + Hash + Eq,
    V: TryFrom<Primitive>,
{
    type Error = AppError;

    fn try_from(value: Primitive) -> Result<Self, Self::Error> {
        match value {
            Primitive::Map(m) => m
                .into_inner()
                .into_iter()
                .map(|(k, v)| match (K::try_from(k), V::try_from(v)) {
                    (Ok(k), Ok(v)) => Ok((k, v)),
                    _ => Err(AmqpError::DecodeError)?,
                })
                .collect(),
            _ => Err(AmqpError::DecodeError)?,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use indexmap::IndexMap;

    #[test]
    fn construct_null() {
        let val = Primitive::Null;
        assert_eq!(val.encode().constructor(), 0x40);
    }

    #[test]
    fn test_encode_decode_round_trip_null() {
        let before = Primitive::Null;
        let encoded: Vec<u8> = before.clone().encode().into();
        let decoded = Primitive::try_decode(&mut encoded.into_iter()).unwrap();
        assert_eq!(decoded, before);
    }
    #[test]
    fn test_encode_decode_round_trip_boolean() {
        let before = Primitive::Boolean(false);
        let encoded: Vec<u8> = before.clone().encode().into();
        let decoded = Primitive::try_decode(&mut encoded.into_iter()).unwrap();
        assert_eq!(decoded, before);
    }
    #[test]
    fn test_encode_decode_round_trip_ubyte() {
        let before = Primitive::Ubyte(10);
        let encoded: Vec<u8> = before.clone().encode().into();
        let decoded = Primitive::try_decode(&mut encoded.into_iter()).unwrap();
        assert_eq!(decoded, before);
    }
    #[test]
    fn test_encode_decode_round_trip_ushort() {
        let before = Primitive::Ushort(100);
        let encoded: Vec<u8> = before.clone().encode().into();
        let decoded = Primitive::try_decode(&mut encoded.into_iter()).unwrap();
        assert_eq!(decoded, before);
    }
    #[test]
    fn test_encode_decode_round_trip_uint() {
        let before = Primitive::Uint(100);
        let encoded: Vec<u8> = before.clone().encode().into();
        let decoded = Primitive::try_decode(&mut encoded.into_iter()).unwrap();
        assert_eq!(decoded, before);
    }
    #[test]
    fn test_encode_decode_round_trip_ulong() {
        let before = Primitive::Ulong(100);
        let encoded: Vec<u8> = before.clone().encode().into();
        let decoded = Primitive::try_decode(&mut encoded.into_iter()).unwrap();
        assert_eq!(decoded, before);
    }
    #[test]
    fn test_encode_decode_round_trip_byte() {
        let before = Primitive::Byte(100);
        let encoded: Vec<u8> = before.clone().encode().into();
        let decoded = Primitive::try_decode(&mut encoded.into_iter()).unwrap();
        assert_eq!(decoded, before);
    }
    #[test]
    fn test_encode_decode_round_trip_short() {
        let before = Primitive::Short(100);
        let encoded: Vec<u8> = before.clone().encode().into();
        let decoded = Primitive::try_decode(&mut encoded.into_iter()).unwrap();
        assert_eq!(decoded, before);
    }
    #[test]
    fn test_encode_decode_round_trip_int() {
        let before = Primitive::Int(100);
        let encoded: Vec<u8> = before.clone().encode().into();
        let decoded = Primitive::try_decode(&mut encoded.into_iter()).unwrap();
        assert_eq!(decoded, before);
    }
    #[test]
    fn test_encode_decode_round_trip_long() {
        let before = Primitive::Long(100);
        let encoded: Vec<u8> = before.clone().encode().into();
        let decoded = Primitive::try_decode(&mut encoded.into_iter()).unwrap();
        assert_eq!(decoded, before);
    }
    #[test]
    fn test_encode_decode_round_trip_float() {
        let before = Primitive::Float(1.0.into());
        let encoded: Vec<u8> = before.clone().encode().into();
        let decoded = Primitive::try_decode(&mut encoded.into_iter()).unwrap();
        assert_eq!(decoded, before);
    }
    #[test]
    fn test_encode_decode_round_trip_double() {
        let before = Primitive::Double(100.0.into());
        let encoded: Vec<u8> = before.clone().encode().into();
        let decoded = Primitive::try_decode(&mut encoded.into_iter()).unwrap();
        assert_eq!(decoded, before);
    }
    #[test]
    fn test_encode_decode_round_trip_decimal32() {
        let before = Primitive::Decimal32(100.0.into());
        let encoded: Vec<u8> = before.clone().encode().into();
        let decoded = Primitive::try_decode(&mut encoded.into_iter()).unwrap();
        assert_eq!(decoded, before);
    }
    #[test]
    fn test_encode_decode_round_trip_decimal64() {
        let before = Primitive::Decimal64(100.0.into());
        let encoded: Vec<u8> = before.clone().encode().into();
        let decoded = Primitive::try_decode(&mut encoded.into_iter()).unwrap();
        assert_eq!(decoded, before);
    }
    #[test]
    #[ignore]
    // Ignored because f128 is not implemented yet
    fn test_encode_decode_round_trip_decimal128() {
        let before = Primitive::Decimal128(Decimal128());
        let encoded: Vec<u8> = before.clone().encode().into();
        let decoded = Primitive::try_decode(&mut encoded.into_iter()).unwrap();
        assert_eq!(decoded, before);
    }
    #[test]
    fn test_encode_decode_round_trip_timestamp() {
        let before = Primitive::Timestamp(10000.into());
        let encoded: Vec<u8> = before.clone().encode().into();
        let decoded = Primitive::try_decode(&mut encoded.into_iter()).unwrap();
        assert_eq!(decoded, before);
    }
    #[test]
    fn test_encode_decode_round_trip_uuid() {
        let before = Primitive::Uuid(uuid::Uuid::new_v4().into());
        let encoded: Vec<u8> = before.clone().encode().into();
        let decoded = Primitive::try_decode(&mut encoded.into_iter()).unwrap();
        assert_eq!(decoded, before);
    }
    #[test]
    fn test_encode_decode_round_trip_binary() {
        let before = Primitive::Binary(vec![1, 2, 3, 4, 5].into());
        let encoded: Vec<u8> = before.clone().encode().into();
        let decoded = Primitive::try_decode(&mut encoded.into_iter()).unwrap();
        assert_eq!(decoded, before);
    }
    #[test]
    fn test_encode_decode_round_trip_string() {
        let before = Primitive::String("Hello World".to_string());
        let encoded: Vec<u8> = before.clone().encode().into();
        let decoded = Primitive::try_decode(&mut encoded.into_iter()).unwrap();
        assert_eq!(decoded, before);
    }
    #[test]
    fn test_encode_decode_round_trip_symbol() {
        let before = Primitive::Symbol(Symbol::new("book:seller:entry".to_string()).unwrap());
        let encoded: Vec<u8> = before.clone().encode().into();
        let decoded = Primitive::try_decode(&mut encoded.into_iter()).unwrap();
        assert_eq!(decoded, before);
    }
    #[test]
    fn test_encode_decode_round_trip_list() {
        let before = Primitive::List(
            vec![
                Primitive::String("Hello world".to_string()),
                Primitive::Char('a'),
                Primitive::Byte(10),
            ]
            .into(),
        );
        let encoded: Vec<u8> = before.clone().encode().into();
        let decoded = Primitive::try_decode(&mut encoded.into_iter()).unwrap();
        assert_eq!(decoded, before);
    }
    #[test]
    fn test_encode_decode_round_trip_map() {
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
        let encoded: Vec<u8> = before.clone().encode().into();
        let decoded = Primitive::try_decode(&mut encoded.into_iter()).unwrap();
        assert_eq!(decoded, before);
    }
    #[test]
    fn test_encode_decode_round_trip_array() {
        let before = Primitive::Array(
            vec![
                Primitive::Int(-100),
                Primitive::Int(100),
                Primitive::Int(120),
            ]
            .into(),
        );
        let encoded: Vec<u8> = before.clone().encode().into();
        let decoded = Primitive::try_decode(&mut encoded.into_iter()).unwrap();
        assert_eq!(decoded, before);
    }
}
