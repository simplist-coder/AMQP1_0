use amqp_derive::AmqpComposite;

#[derive(Debug, Clone, PartialEq, Default, AmqpComposite)]
#[amqp(name = "amqp:target:list", code = 0x29)]
pub struct Target {}