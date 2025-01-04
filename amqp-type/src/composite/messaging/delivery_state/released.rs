use amqp_derive::AmqpComposite;

#[derive(Debug, Clone, PartialEq, AmqpComposite)]
#[amqp(name = "amqp:released:list", code = 0x26)]
pub struct Released {}