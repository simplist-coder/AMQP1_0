use crate::error::AppError;
use crate::composite::Composite;
use crate::primitive::variable_width::symbol::Symbol;
use crate::primitive::Primitive;
use crate::restricted::duration::Milliseconds;
use crate::restricted::fields::Fields;
use crate::restricted::ietf_language_tag::IetfLanguageTag;
use crate::serde::encode::Encode;
use std::vec::IntoIter;
use amqp_derive::AmqpComposite;

#[derive(Debug, Clone, PartialEq, AmqpComposite)]
#[amqp(name = "amqp:open:list", code = 0x10)]
pub struct Open {
    container_id: String,
    host_name: Option<String>,
    max_frame_size: Option<u32>,
    channel_max: Option<u16>,
    idle_timeout: Option<Milliseconds>,
    outgoing_locales: Vec<IetfLanguageTag>,
    incoming_locales: Vec<IetfLanguageTag>,
    offered_capabilities: Vec<Symbol>,
    desired_capabilities: Vec<Symbol>,
    properties: Option<Fields>,
}

impl Open {
    pub fn new(container_id: String) -> Open {
        Open {
            container_id,
            host_name: None,
            max_frame_size: None,
            channel_max: None,
            idle_timeout: None,
            outgoing_locales: vec![],
            incoming_locales: vec![],
            offered_capabilities: vec![],
            desired_capabilities: vec![],
            properties: None,
        }
    }
}

impl Open {
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
    use crate::primitive::compound::map::Map;

    #[test]
    fn test_encode_decode_round_trip_empty() {
        let initial = Open::new("foo".to_string());
        let encoded = initial.clone().encode();
        let decoded = Performative::try_decode(&mut encoded.into_iter()).unwrap();

        assert_eq!(Performative::Open(initial), decoded);
    }

    #[test]
    fn test_encode_decode_round_trip_all_values() {
        let initial = Open {
            container_id: "test".to_string(),
            host_name: Some(String::from("host_name")),
            max_frame_size: Some(5000),
            channel_max: Some(1000),
            idle_timeout: Some(1000),
            outgoing_locales: vec![
                IetfLanguageTag::new("de-at".to_string()),
                IetfLanguageTag::new("en-us".to_string()),
            ],
            incoming_locales: vec![
                IetfLanguageTag::new("en-us".to_string()),
                IetfLanguageTag::new("de-at".to_string()),
            ],
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

        assert_eq!(Performative::Open(initial), decoded);
    }
}