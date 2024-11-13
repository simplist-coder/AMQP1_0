use amqp_error::AppError;

const BOOLEAN: u8 = 0x56;
const BOOLEAN_TRUE: u8 = 0x41;
const BOOLEAN_FALSE: u8 = 0x42;
const BYTE: u8 = 0x51;
const CHAR: u8 = 0x73;
const DECIMAL_32: u8 = 0x74;
const DECIMAL_64: u8 = 0x84;
const DOUBLE: u8 = 0x82;
const FLOAT: u8 = 0x72;
const INTEGER: u8 = 0x71;
const SMALL_INTEGER: u8 = 0x54;
const LONG: u8 = 0x81;
const SMALL_LONG: u8 = 0x55;
const SHORT: u8 = 0x61;
const TIMESTAMP: u8 = 0x83;
const UNSIGNED_BYTE: u8 = 0x50;
const UNSIGNED_INTEGER: u8 = 0x70;
const SMALL_UNSIGNED_INTEGER: u8 = 0x52;
const UNSIGNED_INTEGER_ZERO: u8 = 0x43;
const UNSIGNED_LONG: u8 = 0x80;
const SMALL_UNSIGNED_LONG: u8 = 0x53;
const UNSIGNED_LONG_ZERO: u8 = 0x44;
const UNSIGNED_SHORT: u8 = 0x60;
const UUID: u8 = 0x98;
const NULL: u8 = 0x40;
const ARRAY_SHORT: u8 = 0xe0;
const ARRAY: u8 = 0xf0;
const LIST_EMPTY: u8 = 0x45;
const LIST_SHORT: u8 = 0xc0;
const LIST: u8 = 0xd0;
const MAP_SHORT: u8 = 0xc1;
const MAP: u8 = 0xd1;
const BINARY_SHORT: u8 = 0xa0;
const BINARY: u8 = 0xb0;
const STRING_SHORT: u8 = 0xa1;
const STRING: u8 = 0xb1;
const SYMBOL_SHORT: u8 = 0xa3;
const SYMBOL: u8 = 0xb3;
const DESCRIBED_TYPE: u8 = 0x00;

pub enum Constructor {
    Boolean,
    BooleanTrue,
    BooleanFalse,
    Byte,
    Char,
    Decimal32,
    Decimal64,
    Double,
    Float,
    Integer,
    SmallInteger,
    Long,
    SmallLong,
    Short,
    Timestamp,
    UnsignedByte,
    UnsignedInteger,
    SmallUnsignedInteger,
    UnsignedIntegerZero,
    UnsignedLong,
    SmallUnsignedLong,
    UnsignedLongZero,
    UnsignedShort,
    Uuid,
    Null,
    ArrayShort,
    Array,
    ListEmpty,
    ListShort,
    List,
    MapShort,
    Map,
    BinaryShort,
    Binary,
    StringShort,
    String,
    SymbolShort,
    Symbol,
    DescribedType,
}

impl TryFrom<u8> for Constructor {
    type Error = AppError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            BOOLEAN => Ok(Self::Boolean),
            BOOLEAN_TRUE => Ok(Self::BooleanTrue),
            BOOLEAN_FALSE => Ok(Self::BooleanFalse),
            BYTE => Ok(Self::Byte),
            CHAR => Ok(Self::Char),
            DECIMAL_32 => Ok(Self::Decimal32),
            DECIMAL_64 => Ok(Self::Decimal64),
            DOUBLE => Ok(Self::Double),
            FLOAT => Ok(Self::Float),
            INTEGER => Ok(Self::Integer),
            SMALL_INTEGER => Ok(Self::SmallInteger),
            LONG => Ok(Self::Long),
            SMALL_LONG => Ok(Self::SmallLong),
            SHORT => Ok(Self::Short),
            TIMESTAMP => Ok(Self::Timestamp),
            UNSIGNED_BYTE => Ok(Self::UnsignedByte),
            UNSIGNED_INTEGER => Ok(Self::UnsignedInteger),
            SMALL_UNSIGNED_INTEGER => Ok(Self::SmallUnsignedInteger),
            UNSIGNED_INTEGER_ZERO => Ok(Self::UnsignedIntegerZero),
            UNSIGNED_LONG => Ok(Self::UnsignedLong),
            SMALL_UNSIGNED_LONG => Ok(Self::SmallUnsignedLong),
            UNSIGNED_LONG_ZERO => Ok(Self::UnsignedLongZero),
            UNSIGNED_SHORT => Ok(Self::UnsignedShort),
            UUID => Ok(Self::Uuid),
            NULL => Ok(Self::Null),
            ARRAY_SHORT => Ok(Self::ArrayShort),
            ARRAY => Ok(Self::Array),
            LIST_EMPTY => Ok(Self::ListEmpty),
            LIST_SHORT => Ok(Self::ListShort),
            LIST => Ok(Self::List),
            MAP_SHORT => Ok(Self::MapShort),
            MAP => Ok(Self::Map),
            BINARY_SHORT => Ok(Self::BinaryShort),
            BINARY => Ok(Self::Binary),
            STRING_SHORT => Ok(Self::StringShort),
            STRING => Ok(Self::String),
            SYMBOL_SHORT => Ok(Self::SymbolShort),
            SYMBOL => Ok(Self::Symbol),
            DESCRIBED_TYPE => Ok(Self::DescribedType),
            other => Err(AppError::DeserializationIllegalConstructorError(other))
        }
    }
}

impl From<Constructor> for u8 {
    fn from(value: Constructor) -> Self {
        match value {
            Constructor::Boolean => BOOLEAN,
            Constructor::BooleanTrue => BOOLEAN_TRUE,
            Constructor::BooleanFalse => BOOLEAN_FALSE,
            Constructor::Byte => BYTE,
            Constructor::Char => CHAR,
            Constructor::Decimal32 => DECIMAL_32,
            Constructor::Decimal64 => DECIMAL_64,
            Constructor::Double => DOUBLE,
            Constructor::Float => FLOAT,
            Constructor::Integer => INTEGER,
            Constructor::SmallInteger => SMALL_INTEGER,
            Constructor::Long => LONG,
            Constructor::SmallLong => SMALL_LONG,
            Constructor::Short => SHORT,
            Constructor::Timestamp => TIMESTAMP,
            Constructor::UnsignedByte => UNSIGNED_BYTE,
            Constructor::UnsignedInteger => UNSIGNED_INTEGER,
            Constructor::SmallUnsignedInteger => SMALL_UNSIGNED_INTEGER,
            Constructor::UnsignedIntegerZero => UNSIGNED_INTEGER_ZERO,
            Constructor::UnsignedLong => UNSIGNED_LONG,
            Constructor::SmallUnsignedLong => SMALL_UNSIGNED_LONG,
            Constructor::UnsignedLongZero => UNSIGNED_LONG_ZERO,
            Constructor::UnsignedShort => UNSIGNED_SHORT,
            Constructor::Uuid => UUID,
            Constructor::Null => NULL,
            Constructor::ArrayShort => ARRAY_SHORT,
            Constructor::Array => ARRAY,
            Constructor::ListEmpty => LIST_EMPTY,
            Constructor::ListShort => LIST_SHORT,
            Constructor::List => LIST,
            Constructor::MapShort => MAP_SHORT,
            Constructor::Map => MAP,
            Constructor::BinaryShort => BINARY_SHORT,
            Constructor::Binary => BINARY,
            Constructor::StringShort => STRING_SHORT,
            Constructor::String => STRING,
            Constructor::SymbolShort => SYMBOL_SHORT,
            Constructor::Symbol => SYMBOL,
            Constructor::DescribedType => DESCRIBED_TYPE,
        }
    }
}
