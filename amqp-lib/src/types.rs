trait Encode {
    fn constructor(&self) -> Constructor;
}
pub struct Timestamp(u64);
pub struct Binary();
pub struct Symbol();
pub struct List();
pub struct Map();
pub struct Array();
pub struct Uuid();
pub struct Described();
pub struct Constructor(u8);
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
    Float(f32),
    Double(f64),
    Decimal32(f32),
    Decimal64(f64),
    Char(char),
    Timestamp(Timestamp),
    Uuid(Uuid),
    Binary(Binary),
    String(String),
    Symbol(Symbol),
    List(List),
    Map(Map),
    Array(Array),
    Described(Described), // Decimal128(), Not supported yet
}

impl Encode for AmqpType {
    fn constructor(&self) -> Constructor {
        match self {
            Self::Null => Constructor(0x40),
            Self::Boolean(b) => b.constructor(),
            Self::Ubyte(_) => Constructor(0x50),
            Self::Ushort(_) => Constructor(0x60),
            Self::Uint(val) => val.constructor(),
            Self::Ulong(val) => val.constructor(),
            Self::Byte(_) => Constructor(0x51),
            Self::Short(_) => Constructor(0x61),
            Self::Int(val) => val.constructor(),
            Self::Long(val) => val.constructor(),
            Self::Float(_) => Constructor(0x72),
            Self::Double(_) => Constructor(0x82),
            Self::Decimal32(_) => Constructor(0x74),
            Self::Decimal64(_) => Constructor(0x84),
            Self::Char(_) => Constructor(0x73),
            Self::Timestamp(_) => Constructor(0x83),
            Self::Uuid(_) => Constructor(0x98),
            Self::Binary(val) => val.constructor(),
            Self::String(val) => val.constructor(),
            Self::Symbol(val) => val.constructor(),
            Self::List(val) => val.constructor(),
            Self::Map(val) => val.constructor(),
            Self::Array(val) => val.constructor(),
            Self::Described(val) => val.constructor(),
        }
    }
}

impl Encode for bool {

    #[cfg(feature = "zero-length-bools")]
    fn constructor(&self) -> Constructor {
        match self {
            true => Constructor(0x41),
            false => Constructor(0x42)
        }
    }

    
    #[cfg(not(feature = "zero-length-bools"))]
    fn constructor(&self) -> Constructor {
        Constructor(0x56)
    }
}

impl Encode for u32 {
    fn constructor(&self) -> Constructor {
        match self {
            0 => Constructor(0x43),
            x if x > &0 && x<= &255 => Constructor(0x52),
            _ => Constructor(0x70)
        }
    }
}

impl  Encode for u64 {
    fn constructor(&self) -> Constructor {
        match self {
            0 => Constructor(0x44),
            x if x > &&0 && x <= &255 => Constructor(0x53),
            _ => Constructor(0x80)
        }
    }
}

impl Encode for i32 {
    fn constructor(&self) -> Constructor {
        match self {
            x if x >= &-128 && x <= &127 => Constructor(0x54),
            _ => Constructor(0x71)
        }
    }
}

impl Encode for i64 {
    fn constructor(&self) -> Constructor {
        match self {
            x if x >= &-128 && x <= &127 => Constructor(0x55),
            _ => Constructor(0x81)
        }
        
    }
}

impl Encode for String {
    fn constructor(&self) -> Constructor {
        match self.len() {
            x if x >= 0 as usize && x <= 255 as usize => Constructor(0xa1),
            _ => Constructor(0xb1)
        }
    }
}

impl Encode for Binary {
    fn constructor(&self) -> Constructor {
        todo!()
    }
}

impl Encode for Symbol {
    fn constructor(&self) -> Constructor {
        todo!()
    }
}

impl Encode for List {
    fn constructor(&self) -> Constructor {
        todo!()
    }
}

impl Encode for Map {
    fn constructor(&self) -> Constructor {
        todo!()
    }
}

impl Encode for Array {
    fn constructor(&self) -> Constructor {
        todo!()
    }
}

impl Encode for Described {
    fn constructor(&self) -> Constructor {
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

impl From<f32> for AmqpType {
    fn from(value: f32) -> Self {
        AmqpType::Float(value)
    }
}

impl From<f64> for AmqpType {
    fn from(value: f64) -> Self {
        AmqpType::Double(value)
    }
}

impl From<char> for AmqpType {
    fn from(value: char) -> Self {
        AmqpType::Char(value)
    }
}

impl From<Timestamp> for AmqpType {
    fn from(value: Timestamp) -> Self {
        AmqpType::Timestamp(value)
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
        assert_eq!(val.constructor().0, 0x40);
    }

    #[test]
    #[cfg(not(feature = "zero-length-bools"))]
    fn amqp_type_can_construct_bool() {
        let val = AmqpType::Boolean(true);
        assert_eq!(val.constructor().0, 0x56);
    }

    #[test]
    #[cfg(feature = "zero-length-bools")]
    fn amqp_type_constructs_bool_false_as_zero_length() {
        let val = AmqpType::Boolean(false);
        assert_eq!(val.constructor().0, 0x42);
    }

    #[test]
    #[cfg(feature = "zero-length-bools")]
    fn amqp_type_contructs_bool_true_as_zero_length() {
        let val = AmqpType::Boolean(true);
        assert_eq!(val.constructor().0, 0x41)
    }

    #[test]
    fn amqp_type_can_construct_ubyte() {
        let val = AmqpType::Ubyte(8);
        assert_eq!(val.constructor().0, 0x50);
    }

    #[test]
    fn amqp_type_can_construct_ushort() {
        let val = AmqpType::Ushort(16);
        assert_eq!(val.constructor().0, 0x60);
    }

    #[test]
    fn amqp_type_can_construct_uint() {
        let val = AmqpType::Uint(500);
        assert_eq!(val.constructor().0, 0x70);
    }

    #[test]
    fn amqp_type_encodes_uint_value_0_as_zero_length() {
        let val = AmqpType::Uint(0);
        assert_eq!(val.constructor().0, 0x43);
    }

    #[test]
    fn amqp_type_encodes_uint_values_smaller_than_256_as_smalluint() {
        let val = AmqpType::Uint(255);
        assert_eq!(val.constructor().0, 0x52);
    }
    #[test]
    fn amqp_type_can_construct_ulong() {
        let val = AmqpType::Ulong(500);
        assert_eq!(val.constructor().0, 0x80);
    }

    #[test]
    fn amqp_type_encodes_ulong_smaller_than_256_as_smallulong() {
        let val = AmqpType::Ulong(255);
        assert_eq!(val.constructor().0, 0x53);
    }

    #[test]
    fn amqp_type_encodes_ulong_value_0_as_zero_length() {
        let val = AmqpType::Ulong(0);
        assert_eq!(val.constructor().0, 0x44);
    }

    #[test]
    fn amqp_type_can_construct_byte() {
        let val = AmqpType::Byte(8);
        assert_eq!(val.constructor().0, 0x51);
    }

    #[test]
    fn amqp_type_can_construct_short() {
        let val = AmqpType::Short(8);
        assert_eq!(val.constructor().0, 0x61);
    }

    #[test]
    fn amqp_type_can_construct_int() {
        let val = AmqpType::Int(500);
        assert_eq!(val.constructor().0, 0x71);
    }

    #[test]
    fn amqp_encodes_ints_between_neg_128_and_127_as_smallint() {
        let lower = AmqpType::Int(-128);
        let higher = AmqpType::Int(127);
        assert_eq!(lower.constructor().0, 0x54);
        assert_eq!(higher.constructor().0, 0x54);
    }
    #[test]
    fn amqp_type_can_construct_long() {
        let val = AmqpType::Long(500);
        assert_eq!(val.constructor().0, 0x81);
    }

    #[test]
    fn amqp_encodes_longs_between_neg_128_and_127_as_smalllong() {
        let lower = AmqpType::Long(-128);
        let higher = AmqpType::Long(127);
        assert_eq!(lower.constructor().0, 0x55);
        assert_eq!(higher.constructor().0, 0x55);
    }

    #[test]
    fn amqp_type_can_construct_float() {
        let val = AmqpType::Float(32.0);
        assert_eq!(val.constructor().0, 0x72);
    }

    #[test]
    fn amqp_type_can_construct_double() {
        let val = AmqpType::Double(64.0);
        assert_eq!(val.constructor().0, 0x82);
    }

    #[test]
    fn amqp_type_can_construct_decimal_32() {
        let val = AmqpType::Decimal32(32.0);
        assert_eq!(val.constructor().0, 0x74);
    }

    #[test]
    fn amqp_type_can_construct_decimal_64() {
        let val = AmqpType::Decimal64(64.0);
        assert_eq!(val.constructor().0, 0x84);
    }

    #[test]
    fn amqp_type_can_construct_char() {
        let val = AmqpType::Char('a');
        assert_eq!(val.constructor().0, 0x73);
    }

    #[test]
    fn amqp_type_can_construct_timestamp() {
        let val = AmqpType::Timestamp(Timestamp(1));
        assert_eq!(val.constructor().0, 0x83);
    }

    // #[test]
    // fn amqp_type_can_construct_uuid() {
    //     let val = AmqpType::Uuid(Uuid());
    //     assert_eq!(val.constructor().0, 0x98);
    // }

    // #[test]
    // fn amqp_type_can_construct_binary() {
    //     let val = AmqpType::Binary(Binary());
    //     assert_eq!(val.constructor().0, 0xa0);
    // }

    #[test]
    fn amqp_type_encodes_strings_up_to_255_bytes_as_str8() {
        let val = AmqpType::String("hello".to_string());
        assert_eq!(val.constructor().0, 0xa1);
    }

    #[test]
    fn amqp_type_encodes_strings_longer_than_255_bytes_as_str32() {
        let val = AmqpType::String("hhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhh".to_string());
        assert_eq!(val.constructor().0, 0xb1);
    }

    // #[test]
    // fn amqp_type_can_construct_symbol() {
    //     let val = AmqpType::Symbol(Symbol());
    //     assert_eq!(val.constructor().0, 0xa3);
    // }

    // #[test]
    // fn amqp_type_can_construct_list() {
    //     let val = AmqpType::List(List());
    //     assert_eq!(val.constructor().0, 0x45);
    // }

    // #[test]
    // fn amqp_type_can_construct_map() {
    //     let val = AmqpType::Map(Map());
    //     assert_eq!(val.constructor().0, 0xc1);
    // }

    // #[test]
    // fn amqp_type_can_construct_array() {
    //     let val = AmqpType::Array(Array());
    //     assert_eq!(val.constructor().0, 0xe0);
    // }
}
