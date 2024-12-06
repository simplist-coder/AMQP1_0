use crate::primitive::compound::map::Map;
use crate::primitive::variable_width::symbol::Symbol;
use crate::primitive::Primitive;
use indexmap::IndexMap;

/// # Fields
/// A mapping from field name to value.
/// ##### AMQP Spec
/// ```xml
/// <type name="fields" class="restricted" source="map"/>
/// ```
/// The fields type is a map where the keys are restricted to be of type symbol (this excludes the possibility
/// of a null key). There is no further restriction implied by the fields type on the allowed values for the
/// entries or the set of allowed keys.
#[derive(Debug, Clone, PartialEq)]
pub struct Fields(Map);

impl Fields {
    pub fn new(map: IndexMap<Symbol, Primitive>) -> Self {
        Fields(Map::from(map))
    }
}

impl From<Fields> for   Primitive {
    fn from(value: Fields) -> Self {
        Primitive::Map(value.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fields_create() {
        let mut map = IndexMap::new();
        let mut data = IndexMap::new();
        data.insert(
            Primitive::Symbol(Symbol::new("hello".to_string()).unwrap()),
            Primitive::from("world".to_string()),
        );
        map.insert(
            "hello".to_string().try_into().unwrap(),
            "world".to_string().into(),
        );

        let expected = Map::from(data);
        let fields = Fields::new(map);
        assert_eq!(fields.0, expected);
    }

    #[test]
    fn test_fields_conversion_into_primitive() {
        let map = IndexMap::new();
        assert!(matches!(
            Primitive::from(Fields::new(map)),
            Primitive::Map(_)
        ));
    }
}
