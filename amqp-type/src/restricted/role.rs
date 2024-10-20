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
