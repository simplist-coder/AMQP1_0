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
