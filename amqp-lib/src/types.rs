

pub trait Constructable {
    fn constructor(&self) -> Constructor;
}
pub struct Timestamp(u64);
pub struct Binary();
pub struct Symbol();
pub struct List();
pub struct Map();
pub struct Array();
pub struct Uuid();
pub struct Constructor(u8);
pub enum PrimitiveType {
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

    // Decimal128(), Not supported yet
}

impl Constructable for PrimitiveType {

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
        }
    }
}


impl From<bool> for PrimitiveType {
    fn from(value: bool) -> Self {
        PrimitiveType::Boolean(value)
    }
}

impl From<u8> for PrimitiveType {
    fn from(value: u8) -> Self {
        PrimitiveType::Ubyte(value)
    }
}

impl From<u16> for PrimitiveType {
    fn from(value: u16) -> Self {
        PrimitiveType::Ushort(value)
    }
}

impl From<u32> for PrimitiveType {
    fn from(value: u32) -> Self {
        PrimitiveType::Uint(value)
    }
}

impl From<u64> for PrimitiveType {
    fn from(value: u64) -> Self {
        PrimitiveType::Ulong(value)
    }
}

impl From<i8> for PrimitiveType {
    fn from(value: i8) -> Self {
        PrimitiveType::Byte(value)
    }
}

impl From<i16> for PrimitiveType {
    fn from(value: i16) -> Self {
        PrimitiveType::Short(value)
    }
}

impl From<i32> for PrimitiveType {
    fn from(value: i32) -> Self {
        PrimitiveType::Int(value)
    }
}

impl From<i64> for PrimitiveType {
    fn from(value: i64) -> Self {
        PrimitiveType::Long(value)
    }
}

impl From<f32> for PrimitiveType {
    fn from(value: f32) -> Self {
        PrimitiveType::Float(value)
    }
}

impl From<f64> for PrimitiveType {
    fn from(value: f64) -> Self {
        PrimitiveType::Double(value)
    }
}

impl From<char> for PrimitiveType {
    fn from(value: char) -> Self {
        PrimitiveType::Char(value)
    }
}

impl From<Timestamp> for PrimitiveType {
    fn from(value: Timestamp) -> Self {
        PrimitiveType::Timestamp(value)
    }
}

impl From<Uuid> for PrimitiveType {
    fn from(value: Uuid) -> Self {
        PrimitiveType::Uuid(value)
    }
}

impl From<Binary> for PrimitiveType {
    fn from(value: Binary) -> Self {
        PrimitiveType::Binary(value)
    }
}

impl From<String> for PrimitiveType {
    fn from(value: String) -> Self {
        PrimitiveType::String(value)
    }
}

impl From<Symbol> for PrimitiveType {
    fn from(value: Symbol) -> Self {
        PrimitiveType::Symbol(value)
    }
}

impl From<List> for PrimitiveType {
    fn from(value: List) -> Self {
        PrimitiveType::List(value)
    }
}

impl From<Map> for PrimitiveType {
    fn from(value: Map) -> Self {
        PrimitiveType::Map(value)
    }
}

impl From<Array> for PrimitiveType {
    fn from(value: Array) -> Self {
        PrimitiveType::Array(value)
    }
}