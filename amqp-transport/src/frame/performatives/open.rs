use amqp_error::AppError;
use amqp_type::composite::Composite;
use amqp_type::primitive::variable_width::symbol::Symbol;
use amqp_type::restricted::duration::Milliseconds;
use amqp_type::restricted::fields::Fields;
use amqp_type::restricted::ietf_language_tag::IetfLanguageTag;
use std::vec::IntoIter;

#[derive(Debug, Clone)]
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
    pub fn encode(self) -> Vec<u8> {
        todo!()
    }

    pub fn try_decode(composite: Composite, body: &mut IntoIter<u8>) -> Result<Self, AppError> {
        todo!()
    }
}
