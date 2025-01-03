use crate::error::AppError;
use crate::composite::Composite;
use crate::primitive::variable_width::symbol::Symbol;
use crate::restricted::fields::Fields;
use crate::restricted::handle::Handle;
use crate::restricted::transfer_number::TransferNumber;
use std::vec::IntoIter;

#[derive(Debug, Clone, PartialEq)]
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

    pub fn try_decode(_composite: Composite, _body: &mut IntoIter<u8>) -> Result<Self, AppError> {
        todo!()
    }
}
