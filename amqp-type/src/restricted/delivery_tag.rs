use crate::primitive::variable_width::binary::Binary;

/// # Delivery Tag
///
/// ##### AMQP Specification
/// ```xml
/// <type name="delivery-tag" class="restricted" source="binary"/>
/// ```
/// A delivery-tag may be up to 32 octets of binary data.
pub struct DeliveryTag(Binary);
