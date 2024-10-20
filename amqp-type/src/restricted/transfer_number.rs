use crate::restricted::sequence_no::SequenceNumber;

/// # Transfer Number
/// ##### AMQP Spec
/// ```xml
/// <type name="transfer-number" class="restricted" source="sequence-no"/>
/// ```
pub type TransferNumber = SequenceNumber;
