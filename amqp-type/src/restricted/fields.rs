use crate::primitive::compound::map::Map;

/// # Fields
/// A mapping from field name to value.
/// ##### AMQP Spec
/// ```xml
/// <type name="fields" class="restricted" source="map"/>
/// ```
/// The fields type is a map where the keys are restricted to be of type symbol (this excludes the possibility
/// of a null key). There is no further restriction implied by the fields type on the allowed values for the
/// entries or the set of allowed keys.
pub type Fields = Map;
