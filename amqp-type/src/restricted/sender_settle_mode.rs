/// # Sender Settle Mode
/// Settlement policy for a Sender.
///
/// ##### AMQP Specification
/// ```xml
/// <type name="sender-settle-mode" class="restricted" source="ubyte">
///     <choice name="unsettled" value="0"/>
///     <choice name="settled" value="1"/>
///     <choice name="mixed" value="2"/>
/// </type>
/// ```
///
/// Valid Values:
/// - 0: The Sender will send all deliveries initially unsettled to the Receiver.
/// - 1: The Sender will send all deliveries settled to the Receiver.
/// - 2: The Sender may send a mixture of settled and unsettled deliveries to the Receiver.
#[derive(Debug, Clone, Copy)]
pub struct SenderSettleMode(u8);
