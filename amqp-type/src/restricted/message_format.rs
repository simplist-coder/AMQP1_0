/// # Message Format
/// 32-bit message format code.
/// ##### AMQP Spec
/// ```xml
/// <type name="message-format" class="restricted" source="uint"/>
/// ```
/// The upper three octets of a message format code identify a particular message format. The lowest octet
/// indicates the version of said message format. Any given version of a format is forwards compatible
/// with all higher versions.
pub type MessageFormat = u32;
