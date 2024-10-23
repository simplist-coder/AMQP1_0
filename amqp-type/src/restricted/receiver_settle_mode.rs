use crate::primitive::Primitive;
use amqp_error::AppError;

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
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ReceiverSettleMode {
    First,
    Second,
}

impl ReceiverSettleMode {
    pub fn new(value: u8) -> Result<Self, AppError> {
        match value {
            0 => Ok(ReceiverSettleMode::First),
            1 => Ok(ReceiverSettleMode::Second),
            _ => Err(AppError::InvalidSenderSettleMode),
        }
    }
}

impl From<ReceiverSettleMode> for u8 {
    fn from(value: ReceiverSettleMode) -> Self {
        match value {
            ReceiverSettleMode::First => 0,
            ReceiverSettleMode::Second => 1,
        }
    }
}

impl From<ReceiverSettleMode> for Primitive {
    fn from(value: ReceiverSettleMode) -> Self {
        Primitive::Ubyte(u8::from(value))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_receiver_settle_mode() {
        assert_eq!(
            ReceiverSettleMode::new(0).unwrap(),
            ReceiverSettleMode::First
        );
        assert_eq!(
            ReceiverSettleMode::new(1).unwrap(),
            ReceiverSettleMode::Second
        );
    }

    #[test]
    fn test_receiver_settle_mode_error() {
        assert!(matches!(
            ReceiverSettleMode::new(5),
            Err(AppError::InvalidReceiverSettleMode)
        ))
    }

    #[test]
    fn test_receiver_settle_mode_into_primitive() {
        assert_eq!(
            Primitive::from(ReceiverSettleMode::First),
            Primitive::Ubyte(0)
        );
        assert_eq!(
            Primitive::from(ReceiverSettleMode::Second),
            Primitive::Ubyte(1)
        );
    }
}
