use crate::primitive::Primitive;
use amqp_error::AppError;

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
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SenderSettleMode {
    Unsettled,
    Settled,
    Mixed,
}

impl SenderSettleMode {
    pub fn new(value: u8) -> Result<Self, AppError> {
        match value {
            0 => Ok(Self::Unsettled),
            1 => Ok(Self::Settled),
            2 => Ok(Self::Mixed),
            _ => Err(AppError::InvalidSenderSettleMode),
        }
    }
}

impl From<SenderSettleMode> for u8 {
    fn from(value: SenderSettleMode) -> Self {
        match value {
            SenderSettleMode::Unsettled => 0,
            SenderSettleMode::Settled => 1,
            SenderSettleMode::Mixed => 2,
        }
    }
}

impl From<SenderSettleMode> for Primitive {
    fn from(value: SenderSettleMode) -> Self {
        Primitive::Ubyte(u8::from(value))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sender_settle_mode() {
        assert_eq!(
            SenderSettleMode::new(0).unwrap(),
            SenderSettleMode::Unsettled
        );
        assert_eq!(SenderSettleMode::new(1).unwrap(), SenderSettleMode::Settled);
        assert_eq!(SenderSettleMode::new(2).unwrap(), SenderSettleMode::Mixed);
    }

    #[test]
    fn test_sender_settle_mode_error() {
        assert!(matches!(
            SenderSettleMode::new(10),
            Err(AppError::InvalidSenderSettleMode)
        ));
    }

    #[test]
    fn test_sender_settle_mode_into_primitive() {
        assert_eq!(
            Primitive::from(SenderSettleMode::Unsettled),
            Primitive::Ubyte(0)
        );
        assert_eq!(
            Primitive::from(SenderSettleMode::Settled),
            Primitive::Ubyte(1)
        );
        assert_eq!(
            Primitive::from(SenderSettleMode::Mixed),
            Primitive::Ubyte(2)
        );
    }
}
