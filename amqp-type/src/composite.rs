use crate::constants::constructors::*;
use crate::primitive::compound::list::List;
use crate::primitive::variable_width::symbol::Symbol;
use crate::serde::decode::Decode;
use crate::serde::encode::{Encode, Encoded};
use amqp_error::AppError;
use std::vec::IntoIter;

pub trait CompositeType: From<Composite> + Into<Composite> {}

#[derive(Debug, Eq, PartialEq, Clone)]
pub enum Descriptor {
    Symbol(Symbol),
    Code(u64),
}

impl Encode for Descriptor {
    fn encode(self) -> Encoded {
        match self {
            Descriptor::Symbol(x) => x.encode(),
            Descriptor::Code(x) => x.encode(),
        }
    }
}

impl Decode for Descriptor {
    fn try_decode(constructor: u8, stream: &mut IntoIter<u8>) -> Result<Self, AppError>
    where
        Self: Sized,
    {
        match constructor {
            SYMBOL | SYMBOL_SHORT => {
                Symbol::try_decode(constructor, stream).map(Descriptor::Symbol)
            }
            UNSIGNED_LONG => u64::try_decode(UNSIGNED_LONG, stream).map(Descriptor::Code),
            illegal => Err(AppError::DeserializationIllegalConstructorError(illegal)),
        }
    }
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Composite(Descriptor, List);

impl Encode for Composite {
    fn encode(self) -> Encoded {
        let descriptor = self.0.encode().into_bytes();
        let data = self.1.encode().into_bytes();
        Encoded::new_composite(DESCRIBED_TYPE, descriptor, data)
    }
}

impl Decode for Composite {
    fn try_decode(constructor: u8, stream: &mut IntoIter<u8>) -> Result<Self, AppError>
    where
        Self: Sized,
    {
        if constructor != DESCRIBED_TYPE {
            return Err(AppError::DeserializationIllegalConstructorError(
                constructor,
            ));
        }
        let descr_constr = stream
            .next()
            .ok_or(AppError::IteratorEmptyOrTooShortError)?;
        let descriptor = Descriptor::try_decode(descr_constr, stream)?;
        let list_constr = stream
            .next()
            .ok_or(AppError::IteratorEmptyOrTooShortError)?;
        let list = List::try_decode(list_constr, stream)?;
        Ok(Composite(descriptor, list))
    }
}

impl Composite {
    pub fn try_decode_without_constructor(stream: &mut IntoIter<u8>) -> Result<Self, AppError>
    where
        Self: Sized,
    {
        let constr = stream
            .next()
            .ok_or(AppError::IteratorEmptyOrTooShortError)?;
        Composite::try_decode(constr, stream)
    }
}

impl Composite {
    pub fn new(descriptor: Descriptor, list: List) -> Self {
        Composite(descriptor, list)
    }
}

impl From<Symbol> for Descriptor {
    fn from(value: Symbol) -> Self {
        Descriptor::Symbol(value)
    }
}

impl From<u64> for Descriptor {
    fn from(value: u64) -> Self {
        Descriptor::Code(value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::primitive::Primitive;

    #[test]
    fn test_encode_decode_round_trip_composite() {
        let desc = Symbol::new("Hello".to_owned()).unwrap().into();
        let list = vec![
            Primitive::String("World".to_owned()),
            Primitive::Boolean(true),
            Primitive::List(
                vec![
                    Primitive::Binary(vec![1, 2, 3, 4, 5].into()),
                    Primitive::Binary(vec![5, 5, 6, 7, 10].into()),
                ]
                .into(),
            ),
        ]
        .into();
        let original = Composite::new(desc, list);

        let encoded = original.clone().encode().into_bytes();
        let decoded = Composite::try_decode_without_constructor(&mut encoded.into_iter()).unwrap();
        assert_eq!(original, decoded);
    }
}