use amqp_error::AppError;
use amqp_type::alias::Fields;
use amqp_type::composite::Composite;
use amqp_type::primitive::variable_width::symbol::Symbol;
use std::vec::IntoIter;

type TransferNumber = u32;
type Handle = u32;

#[derive(Debug, Clone)]
pub struct Begin {
    remote_channel: Option<u16>,
    next_outgoing_id: TransferNumber,
    incoming_window: u32,
    outgoing_window: u32,
    handle_max: Option<Handle>,
    offered_capabilities: Vec<Symbol>,
    desired_capabilities: Vec<Symbol>,
    properties: Fields,
}

impl Begin {
    pub fn encode(self) -> Vec<u8> {
        todo!()
    }

    pub fn try_decode(composite: Composite, body: &mut IntoIter<u8>) -> Result<Self, AppError> {
        todo!()
    }
}
