pub const BOOLEAN: u8 = 0x56;
pub const BOOLEAN_TRUE: u8 = 0x41;
pub const BOOLEAN_FALSE: u8 = 0x42;
pub const BYTE: u8 = 0x51;
pub const CHAR: u8 = 0x73;
pub const DECIMAL_32: u8 = 0x74;
pub const DECIMAL_64: u8 = 0x84;
pub const DOUBLE: u8 = 0x82;
pub const FLOAT: u8 = 0x72;
pub const INTEGER: u8 = 0x71;
pub const SMALL_INTEGER: u8 = 0x54;
pub const LONG: u8 = 0x81;
pub const SMALL_LONG: u8 = 0x55;
pub const SHORT: u8 = 0x61;
pub const TIMESTAMP: u8 = 0x83;
pub const UNSIGNED_BYTE: u8 = 0x50;
pub const UNSIGNED_INTEGER: u8 = 0x70;
pub const SMALL_UNSIGNED_INTEGER: u8 = 0x52;
pub const UNSIGNED_INTEGER_ZERO: u8 = 0x43;
pub const UNSIGNED_LONG: u8 = 0x80;
pub const SMALL_UNSIGNED_LONG: u8 = 0x53;
pub const UNSIGNED_LONG_ZERO: u8 = 0x44;
pub const UNSIGNED_SHORT: u8 = 0x60;
pub const UUID: u8 = 0x98;
pub const NULL: u8 = 0x40;
pub const ARRAY_SHORT: u8 = 0xe0;
pub const ARRAY: u8 = 0xf0;
pub const LIST_EMPTY: u8 = 0x45;
pub const LIST_SHORT: u8 = 0xc0;
pub const LIST: u8 = 0xd0;
pub const MAP_SHORT: u8 = 0xc1;
pub const MAP: u8 = 0xd1;
pub const BINARY_SHORT: u8 = 0xa0;
pub const BINARY: u8 = 0xb0;
pub const STRING_SHORT: u8 = 0xa1;
pub const STRING: u8 = 0xb1;
pub const SYMBOL_SHORT: u8 = 0xa3;
pub const SYMBOL: u8 = 0xb3;
pub const DESCRIBED_TYPE: u8 = 0x00;






pub const PERFORMATIVE_CODE_OPEN: u64 = 0x10;
pub const PERFORMATIVE_CODE_BEGIN: u64 = 0x11;
pub const PERFORMATIVE_CODE_ATTACH: u64 = 0x12;
pub const PERFORMATIVE_CODE_FLOW: u64 = 0x13;
pub const PERFORMATIVE_CODE_TRANSFER: u64 = 0x14;
pub const PERFORMATIVE_CODE_DISPOSITION: u64 = 0x15;
pub const PERFORMATIVE_CODE_DETACH: u64 = 0x16;
pub const PERFORMATIVE_CODE_END: u64 = 0x17;
pub const PERFORMATIVE_CODE_CLOSE: u64 = 0x18;

pub const PERFORMATIVE_SYMBOL_OPEN: &str = "amqp:open:list";
pub const PERFORMATIVE_SYMBOL_BEGIN: &str = "amqp:begin:list";
pub const PERFORMATIVE_SYMBOL_ATTACH: &str = "amqp:attach:list";
pub const PERFORMATIVE_SYMBOL_FLOW: &str = "amqp:flow:list";
pub const PERFORMATIVE_SYMBOL_TRANSFER: &str = "amqp:transfer:list";
pub const PERFORMATIVE_SYMBOL_DISPOSITION: &str = "amqp:disposition:list";
pub const PERFORMATIVE_SYMBOL_DETACH: &str = "amqp:detach:list";
pub const PERFORMATIVE_SYMBOL_END: &str = "amqp:end:list";
pub const PERFORMATIVE_SYMBOL_CLOSE: &str = "amqp:close:list";