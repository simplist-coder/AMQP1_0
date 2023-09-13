use std::hash::Hash;

use bigdecimal::BigDecimal;
use indexmap::IndexMap;
use crate::types::binary::Binary;
use crate::types::collection::*;
use crate::types::decimal::*;
use crate::error::AppError;


pub trait Hashable: std::hash::Hash {}
pub trait Encode {
    fn encode(&self) -> Encoded;
}

 pub trait Decode<'a>: From<&'a [u8]> + Encode {}


pub struct Encoded {
    constructor: u8,
    data: Option<Vec<u8>>
}

impl Encoded {
    pub fn constructor(&self) -> u8 {
        self.constructor
    }
}


impl From<u8> for Encoded {
    fn from(value: u8) -> Self {
        Encoded { constructor: value, data: None }
    }
}


#[derive(Hash, Eq, PartialEq)]
pub struct Symbol(String);
#[derive(Hash, Eq, PartialEq)]
pub struct Uuid(uuid::Uuid);
#[derive(Hash, Eq, PartialEq)]
pub struct Described();
#[derive(Hash, Eq, PartialEq)]
pub struct Constructor(u8);
#[derive(Hash, Eq, PartialEq)]
pub struct Timestamp(u64);
pub struct Float(f32);
pub struct Double(f64);

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
            Self::Null => 0x40.into(),
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

impl Hash for Float {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.0.to_bits().hash(state)
    }
}

impl Hash for Double {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.0.to_bits().hash(state)
    }
}


impl PartialEq for Float {
    fn eq(&self, other: &Self) -> bool {
        self.0.to_bits() == other.0.to_bits()
    }

    fn ne(&self, other: &Self) -> bool {
        !self.eq(other)
    }
}

impl Eq for Float {}


impl PartialEq for Double {
    fn eq(&self, other: &Self) -> bool {
        self.0.to_bits() == self.0.to_bits()
    }

    fn ne(&self, other: &Self) -> bool {
        !self.eq(other)
    }
}


impl Eq for Double {}
impl From<u8> for Constructor {
    fn from(value: u8) -> Self {
        Constructor(value)
    }
}

impl Encode for Timestamp {
    fn encode(&self) -> Encoded {
        0x83.into()
    }

}
impl From<Timestamp> for AmqpType {
    fn from(value: Timestamp) -> Self {
        AmqpType::Timestamp(value)
    }
}
impl Encode for bool {
    #[cfg(feature = "zero-length-bools")]
    fn encode(&self) -> Encoded {
        match self {
            true => 0x41.into(),
            false => 0x42.into(),
        }
    }

    #[cfg(not(feature = "zero-length-bools"))]
    fn encode(&self) -> Encoded {
        0x56.into()
    }
}

impl Encode for u8 {
    fn encode(&self) -> Encoded {
        0x50.into()
    }

}

impl Encode for u16 {
    fn encode(&self) -> Encoded {
        0x60.into()
    }

}

impl Encode for i8 {
    fn encode(&self) -> Encoded {
        0x51.into()
    }

}

impl Encode for i16 {
    fn encode(&self) -> Encoded {
        0x61.into()
    }

}

impl Encode for Float {
    fn encode(&self) -> Encoded {
        0x72.into()
    }

}

impl Encode for Double {
    fn encode(&self) -> Encoded {
        0x82.into()
    }

}

impl Encode for char {
    fn encode(&self) -> Encoded {
        0x73.into()
    }

}


impl Encode for Uuid {
    fn encode(&self) -> Encoded {
        0x98.into()
    }

}
impl Encode for u32 {
    fn encode(&self) -> Encoded {
        match self {
            0 => 0x43.into(),
            x if x > &0 && x <= &255 => 0x52.into(),
            _ => 0x70.into(),
        }
    }

}

impl Encode for u64 {
    fn encode(&self) -> Encoded {
        match self {
            0 => 0x44.into(),
            x if x > &&0 && x <= &255 => 0x53.into(),
            _ => 0x80.into(),
        }
    }

}

impl Encode for i32 {
    fn encode(&self) -> Encoded {
        match self {
            x if x >= &-128 && x <= &127 => 0x54.into(),
            _ => 0x71.into(),
        }
    }

}

impl Encode for i64 {
    fn encode(&self) -> Encoded {
        match self {
            x if x >= &-128 && x <= &127 => 0x55.into(),
            _ => 0x81.into(),
        }
    }

}

impl Encode for String {
    fn encode(&self) -> Encoded {
        match self.len() {
            x if x >= 0 as usize && x <= 255 as usize => 0xa1.into(),
            _ => 0xb1.into(),
        }
    }

}

impl Encode for Symbol {
    fn encode(&self) -> Encoded {
        match self.0.len() {
            x if x <= 255 => 0xa3.into(),
            _ => 0xb1.into()
        }
    }

}

impl Encode for Described {
    fn encode(&self) -> Encoded {
        todo!()
    }

}


impl From<bool> for AmqpType {
    fn from(value: bool) -> Self {
        AmqpType::Boolean(value)
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

impl From<f32> for Float {
    fn from(value: f32) -> Self {
        Float(value)
    }
}

impl From<Double> for AmqpType {
    fn from(value: Double) -> Self {
        AmqpType::Double(value.into())
    }
}

impl From<f64> for Double {
    fn from(value: f64) -> Self {
        Double(value)
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

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn amqp_type_can_construct_null() {
        let val = AmqpType::Null;
        assert_eq!(val.encode().constructor(), 0x40);
    }

    #[test]
    #[cfg(not(feature = "zero-length-bools"))]
    fn amqp_type_can_construct_bool() {
        let val = AmqpType::Boolean(true);
        assert_eq!(val.encode().constructor(), 0x56);
    }

    #[test]
    #[cfg(feature = "zero-length-bools")]
    fn amqp_type_constructs_bool_false_as_zero_length() {
        let val = AmqpType::Boolean(false);
        assert_eq!(val.encode().constructor(), 0x42);
    }

    #[test]
    #[cfg(feature = "zero-length-bools")]
    fn amqp_type_contructs_bool_true_as_zero_length() {
        let val = AmqpType::Boolean(true);
        assert_eq!(val.encode().constructor(), 0x41)
    }

    #[test]
    fn amqp_type_can_construct_ubyte() {
        let val = AmqpType::Ubyte(8);
        assert_eq!(val.encode().constructor(), 0x50);
    }

    #[test]
    fn amqp_type_can_construct_ushort() {
        let val = AmqpType::Ushort(16);
        assert_eq!(val.encode().constructor(), 0x60);
    }

    #[test]
    fn amqp_type_can_construct_uint() {
        let val = AmqpType::Uint(500);
        assert_eq!(val.encode().constructor(), 0x70);
    }

    #[test]
    fn amqp_type_encodes_uint_value_0_as_zero_length() {
        let val = AmqpType::Uint(0);
        assert_eq!(val.encode().constructor(), 0x43);
    }

    #[test]
    fn amqp_type_encodes_uint_values_smaller_than_256_as_smalluint() {
        let val = AmqpType::Uint(255);
        assert_eq!(val.encode().constructor(), 0x52);
    }
    #[test]
    fn amqp_type_can_construct_ulong() {
        let val = AmqpType::Ulong(500);
        assert_eq!(val.encode().constructor(), 0x80);
    }

    #[test]
    fn amqp_type_encodes_ulong_smaller_than_256_as_smallulong() {
        let val = AmqpType::Ulong(255);
        assert_eq!(val.encode().constructor(), 0x53);
    }

    #[test]
    fn amqp_type_encodes_ulong_value_0_as_zero_length() {
        let val = AmqpType::Ulong(0);
        assert_eq!(val.encode().constructor(), 0x44);
    }

    #[test]
    fn amqp_type_can_construct_byte() {
        let val = AmqpType::Byte(8);
        assert_eq!(val.encode().constructor(), 0x51);
    }

    #[test]
    fn amqp_type_can_construct_short() {
        let val = AmqpType::Short(8);
        assert_eq!(val.encode().constructor(), 0x61);
    }

    #[test]
    fn amqp_type_can_construct_int() {
        let val = AmqpType::Int(500);
        assert_eq!(val.encode().constructor(), 0x71);
    }

    #[test]
    fn amqp_encodes_ints_between_neg_128_and_127_as_smallint() {
        let lower = AmqpType::Int(-128);
        let higher = AmqpType::Int(127);
        assert_eq!(lower.encode().constructor(), 0x54);
        assert_eq!(higher.encode().constructor(), 0x54);
    }
    #[test]
    fn amqp_type_can_construct_long() {
        let val = AmqpType::Long(500);
        assert_eq!(val.encode().constructor(), 0x81);
    }

    #[test]
    fn amqp_encodes_longs_between_neg_128_and_127_as_smalllong() {
        let lower = AmqpType::Long(-128);
        let higher = AmqpType::Long(127);
        assert_eq!(lower.encode().constructor(), 0x55);
        assert_eq!(higher.encode().constructor(), 0x55);
    }

    #[test]
    fn amqp_type_can_construct_float() {
        let val = AmqpType::Float(32.0.into());
        assert_eq!(val.encode().constructor(), 0x72);
    }

    #[test]
    fn amqp_type_can_construct_double() {
        let val = AmqpType::Double(64.0.into());
        assert_eq!(val.encode().constructor(), 0x82);
    }

    #[test]
    fn amqp_type_can_construct_decimal_32() {
        let val = AmqpType::Decimal32(32.0.into());
        assert_eq!(val.encode().constructor(), 0x74);
    }

    #[test]
    fn amqp_type_can_construct_decimal_64() {
        let val = AmqpType::Decimal64(64.0.into());
        assert_eq!(val.encode().constructor(), 0x84);
    }

    #[test]
    fn amqp_type_can_construct_decimal_128() {
        let val = AmqpType::Decimal128(128.0.into());
        assert_eq!(val.encode().constructor(), 0x94);
    }

    #[test]
    fn amqp_type_can_construct_char() {
        let val = AmqpType::Char('a');
        assert_eq!(val.encode().constructor(), 0x73);
    }

    #[test]
    fn amqp_type_can_construct_timestamp() {
        let val = AmqpType::Timestamp(Timestamp(1));
        assert_eq!(val.encode().constructor(), 0x83);
    }

    #[test]
    fn amqp_type_can_construct_uuid() {
        let val = AmqpType::Uuid(Uuid(uuid::Uuid::new_v4()));
        assert_eq!(val.encode().constructor(), 0x98);
    }

    #[test]
    fn amqp_type_can_construct_binary() {
        let val = AmqpType::Binary(Vec::new().into());
        assert_eq!(val.encode().constructor(), 0xa0);
    }

    #[test]
    fn amqp_type_encodes_strings_up_to_255_bytes_as_str8() {
        let val = AmqpType::String("hello".to_string());
        assert_eq!(val.encode().constructor(), 0xa1);
    }

    #[test]
    fn amqp_type_encodes_strings_longer_than_255_bytes_as_str32() {
        let val = AmqpType::String("hhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhh".to_string());
        assert_eq!(val.encode().constructor(), 0xb1);
    }

    #[test]
    fn amqp_type_can_construct_symbol() {
        let val = AmqpType::Symbol(Symbol("".to_string()));
        assert_eq!(val.encode().constructor(), 0xa3);
    }

    #[test]
    fn amqp_type_can_construct_empty_list() {
        let val = AmqpType::List(vec![].into());
        assert_eq!(val.encode().constructor(), 0x45);
    }

    #[test]
    fn amqp_type_can_construct_list_with_less_than_255_elements() {
        let val = AmqpType::List(vec![1.into()].into());
        assert_eq!(val.encode().constructor(), 0xc0);
    }

    #[test]
    fn amqp_type_can_construct_list_with_more_than_255_elements() {
        let mut arr = vec![];
        for i in 0 .. 500 {
            arr.push(i.into())
        }
        let val = AmqpType::List(arr.into());
        assert_eq!(val.encode().constructor(), 0xd0);
    }

    #[test]
    fn amqp_type_can_construct_list_with_less_than_255_elements_and_larger_than_255_bytes() {
        let mut arr = vec![];
        for i in 0 .. 100 {
            arr.push("aaaaaaaaaaaaaaaaaaaa".into());
        }
        let val = AmqpType::List(arr.into());
        assert_eq!(val.encode().constructor(), 0xd0);
        
    }

    #[test]
    fn amqp_type_can_construct_map_with_less_than_255_elements() {
        let val = AmqpType::Map(IndexMap::new().into());
        assert_eq!(val.encode().constructor(), 0xc1);
    }

    #[test]
    fn amqp_type_can_construct_map_with_less_more_255_elements() {
        let mut map = IndexMap::new();
        for i in 1 .. 500 {
            map.insert(i.into(), i.into());
        }
        let val = AmqpType::Map(map.into());
        assert_eq!(val.encode().constructor(), 0xd1);
    }

    #[test]
    fn amqp_type_can_construct_array_with_less_than_255_elements() {
        let val = AmqpType::Array(vec![].into());
        assert_eq!(val.encode().constructor(), 0xe0);
    }

    #[test]
    fn amqp_type_can_construct_array_with_more_than_255_elements() {

        let mut arr = vec![];
        for i in 0 .. 500 {
            arr.push(i.into())
        }
        let val = AmqpType::Array(arr.into());
        assert_eq!(val.encode().constructor(), 0xf0);
        
    }
}
