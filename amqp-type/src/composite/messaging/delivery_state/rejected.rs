use amqp_derive::AmqpComposite;

#[derive(Debug, Clone, PartialEq, AmqpComposite)]
#[amqp(name = "amqp:rejected:list", code = 0x25)]
pub struct Rejected {}