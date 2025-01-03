use amqp_derive::AmqpComposite;

#[derive(Debug, Clone, PartialEq, Default, AmqpComposite)]
#[amqp(descriptor = "amqp:source:list")]
pub struct Source {}
