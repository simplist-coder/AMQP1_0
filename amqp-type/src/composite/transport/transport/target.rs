use amqp_derive::AmqpComposite;

#[derive(Debug, Clone, PartialEq, Default, AmqpComposite)]
#[amqp(descriptor = "amqp:target:list")]
pub struct Target {}