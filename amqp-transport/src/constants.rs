#![allow(dead_code)]
pub const AMQP_FRAME: u8 = 0x00;
pub const SASL_FRAME: u8 = 0x01;
pub const PERFORMATIVE_CODE_OPEN: u64 = 0x10;
pub const PERFORMATIVE_CODE_BEGIN: u64 = 0x11;
pub const PERFORMATIVE_CODE_ATTACH: u64 = 0x12;
pub const PERFORMATIVE_CODE_FLOW: u64 = 0x13;
pub const PERFORMATIVE_CODE_TRANSFER: u64 = 0x14;
pub const PERFORMATIVE_CODE_DISPOSITION: u64 = 0x15;
pub const PERFORMATIVE_CODE_DETACH: u64 = 0x16;
pub const PERFORMATIVE_CODE_END: u64 = 0x17;
pub const PERFORMATIVE_CODE_CLOSE: u64 = 0x18;

pub const PERFORMATIVE_SYMBOL_OPEN: &'static str = "amqp:open:list";
pub const PERFORMATIVE_SYMBOL_BEGIN: &'static str = "amqp:begin:list";
pub const PERFORMATIVE_SYMBOL_ATTACH: &'static str = "amqp:attach:list";
pub const PERFORMATIVE_SYMBOL_FLOW: &'static str = "amqp:flow:list";
pub const PERFORMATIVE_SYMBOL_TRANSFER: &'static str = "amqp:transfer:list";
pub const PERFORMATIVE_SYMBOL_DISPOSITION: &'static str = "amqp:disposition:list";
pub const PERFORMATIVE_SYMBOL_DETACH: &'static str = "amqp:detach:list";
pub const PERFORMATIVE_SYMBOL_END: &'static str = "amqp:end:list";
pub const PERFORMATIVE_SYMBOL_CLOSE: &'static str = "amqp:close:list";
