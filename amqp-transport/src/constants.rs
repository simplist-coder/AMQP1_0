// Specification Constants

/// The IANA assigned port number for AMQP.
/// The standard AMQP port number that has been assigned
/// by IANA for TCP, UDP, and SCTP.
/// There are currently no UDP or SCTP mappings defined for
/// AMQP. The port number is reserved for future transport
/// mappings to these protocols.
/// ```
///# use amqp_transport::constants::PORT;
/// assert_eq!(PORT, 5672);
/// ```
pub const PORT: u16 = 5672;

/// The IANA assigned port number for secure AMQP (amqps).
/// The standard AMQP port number that has been assigned
/// by IANA for secure TCP using TLS.
/// Implementations listening on this port should NOT expect
/// a protocol handshake before TLS is negotiated.
/// ```
///# use amqp_transport::constants::SECURE_PORT;
/// assert_eq!(SECURE_PORT, 5671);
/// ```
pub const SECURE_PORT: u16 = 5671;

/// AMQP major protocol version.
/// ```
///# use amqp_transport::constants::MAJOR;
/// assert_eq!(MAJOR, 1)
/// ```
pub const MAJOR: u8 = 1;

/// AMQP minor protocol version.
/// ```
///# use amqp_transport::constants::MINOR;
/// assert_eq!(MINOR, 0)
/// ```
pub const MINOR: u8 = 0;

/// AMQP protocol revision.
/// ```
///# use amqp_transport::constants::REVISION;
/// assert_eq!(REVISION, 0)
/// ```
pub const REVISION: u8 = 0;

// Utility Constants
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

pub const PERFORMATIVE_SYMBOL_OPEN: &str = "amqp:open:list";
pub const PERFORMATIVE_SYMBOL_BEGIN: &str = "amqp:begin:list";
pub const PERFORMATIVE_SYMBOL_ATTACH: &str = "amqp:attach:list";
pub const PERFORMATIVE_SYMBOL_FLOW: &str = "amqp:flow:list";
pub const PERFORMATIVE_SYMBOL_TRANSFER: &str = "amqp:transfer:list";
pub const PERFORMATIVE_SYMBOL_DISPOSITION: &str = "amqp:disposition:list";
pub const PERFORMATIVE_SYMBOL_DETACH: &str = "amqp:detach:list";
pub const PERFORMATIVE_SYMBOL_END: &str = "amqp:end:list";
pub const PERFORMATIVE_SYMBOL_CLOSE: &str = "amqp:close:list";
