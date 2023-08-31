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
    Timestamp(u64),
    Uuid(String),
    Binary,
    String(String),
    Symbol,
    List,
    Map,
    Array,
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

