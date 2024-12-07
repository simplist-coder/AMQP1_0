use crate::primitive::compound::map::Map;
use crate::primitive::variable_width::symbol::Symbol;
use crate::primitive::Primitive;
use indexmap::IndexMap;
use crate::error::amqp_error::AmqpError;
use crate::error::AppError;

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

    fn verify_has_symbol_keys(map: &Map) -> Result<(), <Fields as TryFrom<Primitive>>::Error> {
        if let Some((k, _)) = map.inner().first() {
            // Field only allows Symbols as keys in its Map, so we need to
            // return an error if that is not the case, in order to remain compliant to the protocol
            if !matches!(k, Primitive::Symbol(_)) {
                Err(AmqpError::DecodeError)?
            }
        }
        Ok(())
    }
}

impl From<Fields> for Primitive {
    fn from(value: Fields) -> Self {
        Primitive::Map(value.0)
    }
}

impl TryFrom<Primitive> for Fields {
    type Error = AppError;

    fn try_from(value: Primitive) -> Result<Self, Self::Error> {
        let map: Map = value.try_into()?;
        Self::verify_has_symbol_keys(&map)?;
        Ok(Fields(map))
    }
}

impl TryFrom<Primitive> for Option<Fields> {
    type Error = AppError;

    fn try_from(value: Primitive) -> Result<Self, Self::Error> {
        let opt: Option<Map> = value.try_into()?;
        match opt {
            None => Ok(None),
            Some(map) => {Ok(Some(Fields(map)))}
        }
    }
}

impl TryFrom<Map> for Fields {
    type Error = AppError;

    fn try_from(value: Map) -> Result<Self, Self::Error> {
        Primitive::Map(value).try_into()
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
