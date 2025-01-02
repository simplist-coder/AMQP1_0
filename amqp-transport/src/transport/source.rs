use amqp_derive::CompositeType;


#[derive(Debug, Clone, PartialEq, Default, CompositeType)]
#[amqp(descriptor = "amqp:source:list")]
pub struct Source {}
