
#[cfg(not(feature = "zero-length-bools"))]
pub const BOOLEAN: u8 = 0x56;
#[cfg(feature = "zero-length-bools")]
pub const BOOLEAN_TRUE: u8 = 0x41;
#[cfg(feature = "zero-length-bools")]
pub const BOOLEAN_FALSE: u8 = 0x42;

pub const BYTE: u8 = 0x51;