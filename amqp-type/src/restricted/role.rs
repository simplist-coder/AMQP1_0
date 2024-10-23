/// # Role
/// Link endpoint role.
///
/// ##### AMQP Specification
/// ```xml
/// <type name="role" class="restricted" source="boolean">
///     <choice name="sender" value="false"/>
///     <choice name="receiver" value="true"/>
/// </type>
/// ```
/// Valid Values:
/// - false: Sender
/// - true: Receiver

#[derive(Debug, Clone, Copy)]
pub struct Role(bool);

impl Role {
    pub fn new(value: bool) -> Self {
        Self(value)
    }

    pub fn is_sender(&self) -> bool {
        self.0 == false
    }

    pub fn is_receiver(&self) -> bool {
        self.0 == true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_role_is_sender_is_true_for_inner_value_false() {
        assert_eq!(Role::new(false).is_sender(), true);
    }

    #[test]
    fn test_role_is_receiver_is_true_for_inner_value_true() {
        assert_eq!(Role::new(true).is_sender(), true);
    }
}
