/// # Sequence Number
/// A 32-bit RFC-1982 serial number.
/// ```xml
/// <type name="sequence-no" class="restricted" source="uint"/>
/// ```
/// A sequence-no encodes a serial number as defined in RFC-1982. The arithmetic, and operators for
/// these numbers are defined by RFC-1982.
#[derive(Debug, Clone, Copy)]
pub struct SequenceNumber(u32);
