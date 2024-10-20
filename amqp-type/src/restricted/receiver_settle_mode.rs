/// # Receiver Settle Mode
/// Settlement policy for a Receiver
/// ##### AMQP Specification
/// ```xml
/// <type name="receiver-settle-mode" class="restricted" source="ubyte">
///     <choice name="first" value="0"/>
///     <choice name="second" value="1"/>
/// </type>
/// ```
///
/// Valid Values:
/// - 0: The Receiver will spontaneously settle all incoming transfers.
/// - 1: The Receiver will only settle after sending the disposition to the Sender and
///      receiving a disposition indicating settlement of the delivery from the sender.
#[derive(Debug, Clone, Copy)]
pub struct ReceiverSettleMode(u8);
