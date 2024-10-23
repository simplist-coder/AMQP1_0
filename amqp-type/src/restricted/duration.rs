/// # Seconds
/// A duration measured in seconds.
///
/// ##### AMQP Specification
/// ```xml
/// <type name="seconds" class="restricted" source="uint"/>
/// ```
///
/// We just use a type alias of an u32 for this, because the specification does not
/// define any specific properties
pub type Seconds = u32;

/// # Milliseconds
/// A duration measured in milliseconds.
///
/// ##### AMQP Specification
/// ```xml
/// <type name="milliseconds" class="restricted" source="uint"/>
/// ```
///
/// We just use a type alias of an u32 for this, because the specification does not
/// define any specific properties
pub type Milliseconds = u32;
