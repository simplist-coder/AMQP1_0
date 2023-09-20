use std::hash::Hash;

use bigdecimal::num_traits::ToBytes;
use indexmap::IndexMap;

use crate::types::binary::Binary;
use crate::types::collection::*;
use crate::types::decimal::*;
use crate::types::floating_point::*;

pub trait Hashable: Hash {}

pub trait Encode {
    fn encode(&self) -> Encoded;
}

pub trait Decode<'a>: From<&'a [u8]> + Encode {}

pub enum Encoded {
    Empty(u8),
    // Constructor
    Fixed(u8, Vec<u8>),
    // Constructor, Data
    Variable(u8, Vec<u8>),
    // Constructor, Data, size is computed from data
    Compound(u8, u32, Vec<u8>),
    // Constructor, count, data
    Array(u8, u32, u8, Vec<u8>), // Constructor, count, element constructor, data
}

impl Encoded {
    pub fn new_empty(constructor: u8) -> Self {
        Encoded::Empty(constructor)
    }

    pub fn new_fixed(constructor: u8, data: Vec<u8>) -> Self {
        Encoded::Fixed(constructor, data)
    }

    pub fn new_variable(constructor: u8, data: Vec<u8>) -> Self {
        Encoded::Variable(constructor, data)
    }

    pub fn new_compound(constructor: u8, count: u32, data: Vec<u8>) -> Self {
        Encoded::Compound(constructor, count, data)
    }

    pub fn new_array(constructor: u8, count: u32, element_constructor: u8, data: Vec<u8>) -> Self {
        Encoded::Array(constructor, count, element_constructor, data)
    }

    pub fn constructor(&self) -> u8 {
        match self {
            Self::Empty(c) => c.to_owned(),
            Self::Fixed(c, _) => c.to_owned(),
            Self::Variable(c, _) => c.to_owned(),
            Self::Compound(c, _, _) => c.to_owned(),
            Self::Array(c, _, _, _) => c.to_owned(),
        }
    }

    pub fn data_len(&self) -> usize {
        match self {
            Self::Empty(_) => 0,
            Self::Fixed(_, data) => data.len(),
            Self::Variable(_, data) => data.len(),
            Self::Compound(_, _, data) => data.len(),
            Self::Array(_, _, _, data) => data.len(),
        }
    }
}

impl From<Encoded> for Vec<u8> {
    fn from(value: Encoded) -> Self {
        let mut res = Vec::new();
        match value {
            Encoded::Empty(c) => res.push(c),
            Encoded::Fixed(c, mut data) => {
                res.push(c);
                res.append(&mut data);
            }
            Encoded::Variable(c, mut data) => {
                res.push(c);
                let mut size: Vec<u8> = match c {
                    0xA => vec![data.len() as u8],
                    _ => (data.len() as u32).to_be_bytes().to_vec()
                };
                res.append(&mut size);
                res.append(&mut data);
            }
            Encoded::Compound(c, count, mut data) => {
                res.push(c);
                res.append(&mut count.to_be_bytes().to_vec());
                res.append(&mut data);
            }
            Encoded::Array(_, _, _, _) => {
                todo!("Implement Array encode to bytes")
            }
        }
        res
    }
}

impl From<u8> for Encoded {
    fn from(value: u8) -> Self {
        Encoded::Empty(value)
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
        match self {
            true => Encoded::new_fixed(0x56, vec![0x01]),
            false => Encoded::new_fixed(0x56, vec![0x00]),
        }
    }
}

impl Encode for u8 {
    fn encode(&self) -> Encoded {
        Encoded::new_fixed(0x50, self.to_be_bytes().to_vec())
    }
}

impl Encode for u16 {
    fn encode(&self) -> Encoded {
        Encoded::new_fixed(0x60, self.to_be_bytes().to_vec())
    }
}

impl Encode for i8 {
    fn encode(&self) -> Encoded {
        Encoded::new_fixed(0x51, self.to_be_bytes().to_vec())
    }
}

impl Encode for i16 {
    fn encode(&self) -> Encoded {
        Encoded::new_fixed(0x61, self.to_be_bytes().to_vec())
    }
}

impl Encode for char {
    fn encode(&self) -> Encoded {
        Encoded::new_fixed(0x73, self.to_string().into_bytes())
    }
}

impl Encode for Uuid {
    fn encode(&self) -> Encoded {
        Encoded::new_fixed(0x98, self.0.into_bytes().to_vec())
    }
}

impl Encode for u32 {
    fn encode(&self) -> Encoded {
        match self {
            0 => Encoded::new_empty(0x43),
            x if x > &0 && x <= &255 => Encoded::new_fixed(0x52, x.to_be_bytes().to_vec()),
            _ => Encoded::new_fixed(0x70, self.to_be_bytes().to_vec()),
        }
    }
}

impl Encode for u64 {
    fn encode(&self) -> Encoded {
        match self {
            0 => Encoded::new_empty(0x44),
            x if x > &&0 && x <= &255 => Encoded::new_fixed(0x53, x.to_be_bytes().to_vec()),
            _ => Encoded::new_fixed(0x80, self.to_be_bytes().to_vec()),
        }
    }
}

impl Encode for i32 {
    fn encode(&self) -> Encoded {
        match self {
            x if x >= &-128 && x <= &127 => Encoded::new_fixed(0x54, x.to_be_bytes().to_vec()),
            _ => Encoded::new_fixed(0x71, self.to_be_bytes().to_vec()),
        }
    }
}

impl Encode for i64 {
    fn encode(&self) -> Encoded {
        match self {
            x if x >= &-128 && x <= &127 => Encoded::new_fixed(0x55, x.to_be_bytes().to_vec()),
            _ => Encoded::new_fixed(0x81, self.to_be_bytes().to_vec()),
        }
    }
}

impl Encode for String {
    fn encode(&self) -> Encoded {
        match self.len() {
            x if x >= 0 as usize && x <= 255 as usize => {
                Encoded::new_variable(0xa1, self.as_bytes().to_vec())
            }
            _ => Encoded::new_variable(0xb1, self.as_bytes().to_vec()),
        }
    }
}

impl Encode for Symbol {
    fn encode(&self) -> Encoded {
        match self.0.len() {
            x if x <= 255 => Encoded::new_variable(0xa3, self.0.as_bytes().to_vec()),
            _ => Encoded::new_variable(0xb1, self.0.as_bytes().to_vec()),
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn construct_null() {
        let val = AmqpType::Null;
        assert_eq!(val.encode().constructor(), 0x40);
    }

    #[test]
    #[cfg(not(feature = "zero-length-bools"))]
    fn construct_bool() {
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
    fn construct_ubyte() {
        let val = AmqpType::Ubyte(8);
        assert_eq!(val.encode().constructor(), 0x50);
    }

    #[test]
    fn construct_ushort() {
        let val = AmqpType::Ushort(16);
        assert_eq!(val.encode().constructor(), 0x60);
    }

    #[test]
    fn construct_uint() {
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
    fn construct_ulong() {
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
    fn construct_byte() {
        let val = AmqpType::Byte(8);
        assert_eq!(val.encode().constructor(), 0x51);
    }

    #[test]
    fn construct_short() {
        let val = AmqpType::Short(8);
        assert_eq!(val.encode().constructor(), 0x61);
    }

    #[test]
    fn construct_int() {
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
    fn construct_long() {
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
    fn construct_float() {
        let val = AmqpType::Float(32.0.into());
        assert_eq!(val.encode().constructor(), 0x72);
    }

    #[test]
    fn construct_double() {
        let val = AmqpType::Double(64.0.into());
        assert_eq!(val.encode().constructor(), 0x82);
    }

    #[test]
    fn construct_decimal_32() {
        let val = AmqpType::Decimal32(32.0.into());
        assert_eq!(val.encode().constructor(), 0x74);
    }

    #[test]
    fn construct_decimal_64() {
        let val = AmqpType::Decimal64(64.0.into());
        assert_eq!(val.encode().constructor(), 0x84);
    }

    #[test]
    fn construct_decimal_128() {
        let val = AmqpType::Decimal128(128.0.into());
        assert_eq!(val.encode().constructor(), 0x94);
    }

    #[test]
    fn construct_char() {
        let val = AmqpType::Char('a');
        assert_eq!(val.encode().constructor(), 0x73);
    }

    #[test]
    fn construct_timestamp() {
        let val = AmqpType::Timestamp(Timestamp(1));
        assert_eq!(val.encode().constructor(), 0x83);
    }

    #[test]
    fn construct_uuid() {
        let val = AmqpType::Uuid(Uuid(uuid::Uuid::new_v4()));
        assert_eq!(val.encode().constructor(), 0x98);
    }

    #[test]
    fn construct_binary() {
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
    fn construct_symbol() {
        let val = AmqpType::Symbol(Symbol("".to_string()));
        assert_eq!(val.encode().constructor(), 0xa3);
    }

    #[test]
    fn construct_empty_list() {
        let val = AmqpType::List(vec![].into());
        assert_eq!(val.encode().constructor(), 0x45);
    }

    #[test]
    fn construct_list_with_less_than_255_elements() {
        let val = AmqpType::List(vec![1.into()].into());
        assert_eq!(val.encode().constructor(), 0xc0);
    }

    #[test]
    fn construct_list_with_more_than_255_elements() {
        let mut arr = vec![];
        for i in 0..500 {
            arr.push(i.into())
        }
        let val = AmqpType::List(arr.into());
        assert_eq!(val.encode().constructor(), 0xd0);
    }

    #[test]
    fn construct_list_with_less_than_255_elements_and_larger_than_255_bytes() {
        let mut arr = vec![];
        for i in 0..100 {
            arr.push("aaaaaaaaaaaaaaaaaaaa".into());
        }
        let val = AmqpType::List(arr.into());
        assert_eq!(val.encode().constructor(), 0xd0);
    }

    #[test]
    fn construct_map_with_less_than_255_elements() {
        let val = AmqpType::Map(IndexMap::new().into());
        assert_eq!(val.encode().constructor(), 0xc1);
    }

    #[test]
    fn construct_map_with_less_more_255_elements() {
        let mut map = IndexMap::new();
        for i in 1..500 {
            map.insert(i.into(), i.into());
        }
        let val = AmqpType::Map(map.into());
        assert_eq!(val.encode().constructor(), 0xd1);
    }

    #[test]
    fn construct_array_with_less_than_255_elements() {
        let val = AmqpType::Array(vec![].into());
        assert_eq!(val.encode().constructor(), 0xe0);
    }

    #[test]
    fn construct_array_with_more_than_255_elements() {
        let mut arr = vec![];
        for i in 0..500 {
            arr.push(i.into())
        }
        let val = AmqpType::Array(arr.into());
        assert_eq!(val.encode().constructor(), 0xf0);
    }

    #[test]
    fn construct_array_with_less_than_255_elements_and_larger_than_255_bytes() {
        let mut arr = vec![];
        for i in 0..100 {
            arr.push("aaaaaaaaaaaaaaaaaaaa".into());
        }
        let val = AmqpType::Array(arr.into());
        assert_eq!(val.encode().constructor(), 0xf0);
    }
}
