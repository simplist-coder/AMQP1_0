use amqp_type::error::AppError;
use crate::transport::source::Source;
use crate::transport::target::Target;
use amqp_type::primitive::composite::{Composite, CompositeType, Descriptor};
use amqp_type::primitive::compound::map::Map;
use amqp_type::primitive::variable_width::symbol::Symbol;
use amqp_type::restricted::fields::Fields;
use amqp_type::restricted::handle::Handle;
use amqp_type::restricted::receiver_settle_mode::ReceiverSettleMode;
use amqp_type::restricted::role::Role;
use amqp_type::restricted::sender_settle_mode::SenderSettleMode;
use amqp_type::restricted::sequence_no::SequenceNumber;
use std::vec::IntoIter;
use amqp_derive::CompositeType;
use amqp_type::error::amqp_error::AmqpError;
use amqp_type::primitive::composite::builder::CompositeBuilder;
use amqp_type::primitive::Primitive;
use amqp_type::serde::encode::Encode;
use crate::constants::PERFORMATIVE_SYMBOL_ATTACH;

#[derive(Debug, Clone, PartialEq, CompositeType)]
#[amqp(descriptor = "amqp:attach:list")]
pub struct Attach {
    name: String,
    handle: Handle,
    role: Role,
    snd_settle_mode: Option<SenderSettleMode>,
    rcv_settle_mode: Option<ReceiverSettleMode>,
    source: Option<Source>,
    target: Option<Target>,
    unsettled: Option<Map>,
    incomplete_unsettled: Option<bool>,
    initial_delivery_count: Option<SequenceNumber>,
    max_message_size: Option<u64>,
    offered_capabilities: Vec<Symbol>,
    desired_capabilities: Vec<Symbol>,
    properties: Option<Fields>,
}

impl Attach {
    pub fn new(name: String, handle: Handle, role: Role) -> Self {
        Attach {
            name,
            handle,
            role,
            snd_settle_mode: None,
            rcv_settle_mode: None,
            source: None,
            target: None,
            unsettled: None,
            incomplete_unsettled: None,
            initial_delivery_count: None,
            max_message_size: None,
            offered_capabilities: vec![],
            desired_capabilities: vec![],
            properties: None,
        }
    }
}

impl Attach {
    pub fn encode(self) -> Vec<u8> {
        let primitive: Primitive = self.into();
        primitive.encode().into_bytes()
    }

    pub fn try_decode(composite: Composite, _body: &mut IntoIter<u8>) -> Result<Self, AppError> {
        Self::try_from(Primitive::from(composite))
    }
}


#[cfg(test)]
mod tests {
    use crate::frame::performative::Performative;
    use super::*;

    #[test]
    fn test_encode_decode_round_trip_empty() {
        let initial = Attach::new("test".to_string(), 0, Role::Receiver);
        let encoded = initial.clone().encode();
        let decoded = Performative::try_decode(&mut encoded.into_iter()).unwrap();

        assert_eq!(Performative::Attach(initial), decoded);
    }

    #[test]
    fn test_encode_decode_round_trip_full() {
        let initial = Attach {
            name: "test".to_string(),
            handle: 0,
            role: Role::Sender,
            snd_settle_mode: Some(SenderSettleMode::Unsettled),
            rcv_settle_mode: Some(ReceiverSettleMode::First),
            source: Some(Source::default()),
            target: Some(Target::default()),
            unsettled: Some(Map::from(
                vec![(Primitive::Symbol(Symbol::with_ascii("unsettled")), Primitive::String("why though?".to_string()))]
            ).try_into().unwrap()),
            incomplete_unsettled: Some(true),
            initial_delivery_count: Some(SequenceNumber::new(0)),
            max_message_size: Some(1024),
            offered_capabilities: vec![
                Symbol::with_ascii("offered1"),
                Symbol::with_ascii("offered2")
            ],
            desired_capabilities: vec![
                Symbol::with_ascii("desired1"),
                Symbol::with_ascii("desired2"),
            ],
            properties: Some(Map::from(
                vec![(Primitive::Symbol(Symbol::with_ascii("hello")), Primitive::String("world".to_string()))]
            ).try_into().unwrap()),
        };
        let encoded = initial.clone().encode();
        let decoded = Performative::try_decode(&mut encoded.into_iter()).unwrap();

        assert_eq!(Performative::Attach(initial), decoded);
    }
}