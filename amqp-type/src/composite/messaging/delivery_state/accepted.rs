use amqp_derive::AmqpComposite;

#[derive(Debug, Clone, PartialEq, AmqpComposite)]
#[amqp(name = "amqp:accepted:list", code = 0x24)]
pub struct Accepted {}