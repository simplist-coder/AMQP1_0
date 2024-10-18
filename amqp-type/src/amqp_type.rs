use crate::composite::composite::Composite;
use crate::primitive::primitive::Primitive;

pub enum AmqpType {
    Primitive(Primitive),
    Composite(Composite),
}
