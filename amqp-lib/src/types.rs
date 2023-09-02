

pub trait Encode {
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
    Described(Described)

    // Decimal128(), Not supported yet
}

impl Encode for AmqpType {

    fn constructor(&self) -> Constructor {
        match self {
            Self::Null=> Constructor(0x40),
            Self::Boolean(b)=> Constructor(0x56),
            Self::Ubyte(_)=> Constructor(0x50),
            Self::Ushort(_)=> Constructor(0x40),
            Self::Uint(_)=> Constructor(0x40),
            Self::Ulong(_)=> Constructor(0x40),
            Self::Byte(_)=> Constructor(0x40),
            Self::Short(_)=> Constructor(0x40),
            Self::Int(_)=> Constructor(0x40),
            Self::Long(_)=> Constructor(0x40),
            Self::Float(_)=> Constructor(0x40),
            Self::Double(_)=> Constructor(0x40),
            Self::Decimal32(_)=> Constructor(0x40),
            Self::Decimal64(_)=> Constructor(0x40),
            Self::Char(_)=> Constructor(0x40),
            Self::Timestamp(_)=> Constructor(0x40),
            Self::Uuid(_)=> Constructor(0x40),
            Self::Binary(_)=> Constructor(0x40),
            Self::String(_)=> Constructor(0x40),
            Self::Symbol(_)=> Constructor(0x40),
            Self::List(_)=> Constructor(0x40),
            Self::Map(_)=> Constructor(0x40),
            Self::Array(_)=> Constructor(0x40),
            Self::Described(_) => Constructor(0x40)
        }
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



