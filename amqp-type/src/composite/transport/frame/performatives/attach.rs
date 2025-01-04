use crate::error::AppError;
use crate::composite::Composite;
use crate::primitive::compound::map::Map;
use crate::primitive::variable_width::symbol::Symbol;
use crate::primitive::Primitive;
use crate::restricted::fields::Fields;
use crate::restricted::handle::Handle;
use crate::restricted::receiver_settle_mode::ReceiverSettleMode;
use crate::restricted::role::Role;
use crate::restricted::sender_settle_mode::SenderSettleMode;
use crate::restricted::sequence_no::SequenceNumber;
use std::vec::IntoIter;
use amqp_derive::AmqpComposite;
use crate::composite::transport::transport::source::Source;
use crate::composite::transport::transport::target::Target;
use crate::serde::encode::Encode;

#[derive(Debug, Clone, PartialEq, AmqpComposite)]
#[amqp(name = "amqp:attach:list", code = 0x12)]
pub struct Attach {
    name: String,
    handle: Handle,
    role: Role,
    snd_settle_mode: Option<SenderSettleMode>, // default: mixed
    rcv_settle_mode: Option<ReceiverSettleMode>, // default: first
    source: Option<Source>,
    target: Option<Target>,
    unsettled: Option<Map>,
    incomplete_unsettled: Option<bool>, // default. false
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
    use super::*;
    use crate::composite::transport::frame::performative::Performative;

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