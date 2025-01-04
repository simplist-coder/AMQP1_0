use amqp_derive::AmqpComposite;

#[derive(Debug, Clone, PartialEq, AmqpComposite)]
#[amqp(name = "amqp:modified:list", code = 0x27)]
pub struct Modified {}
