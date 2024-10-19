use crate::composite::Composite;
use crate::primitive::Primitive;

pub enum AmqpType {
    Primitive(Primitive),
    Composite(Composite),
}
