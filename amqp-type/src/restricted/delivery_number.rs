use crate::restricted::sequence_no::SequenceNumber;

/// # Delivery Number
/// ##### AMQP Spec
/// ```xml
/// <type name="delivery-number" class="restricted" source="sequence-no"/>
/// ```
pub type DeliveryNumber = SequenceNumber;
