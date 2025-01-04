use amqp_derive::AmqpComposite;

#[derive(Debug, Clone, PartialEq, Default, AmqpComposite)]
#[amqp(name = "amqp:source:list", code = 0x28)]
pub struct Source {}
