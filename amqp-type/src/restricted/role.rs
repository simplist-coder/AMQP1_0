use crate::primitive::Primitive;

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
///
/// ```
///# use amqp_type::restricted::role::Role;
/// assert_eq!(Role::new(true), Role::Receiver);
/// assert_eq!(Role::new(false), Role::Sender);
/// ```

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Role {
    Sender,
    Receiver,
}

impl Role {
    pub fn new(value: bool) -> Self {
        match value {
            true => Self::Receiver,
            false => Self::Sender,
        }
    }
}

impl From<Role> for Primitive {
    fn from(value: Role) -> Self {
        Primitive::Boolean(value.into())
    }
}

impl From<Role> for bool {
    fn from(value: Role) -> Self {
        match value {
            Role::Sender => false,
            Role::Receiver => true,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_role_true_is_receiver() {
        assert_eq!(Role::new(true), Role::Receiver);
    }

    #[test]
    fn test_role_false_is_sender() {
        assert_eq!(Role::new(false), Role::Sender);
    }
}
