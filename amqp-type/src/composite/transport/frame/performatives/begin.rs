use crate::error::AppError;
use crate::composite::Composite;
use crate::primitive::variable_width::symbol::Symbol;
use crate::restricted::fields::Fields;
use crate::restricted::handle::Handle;
use crate::restricted::transfer_number::TransferNumber;
use std::vec::IntoIter;
use amqp_derive::AmqpComposite;
use crate::primitive::Primitive;
use crate::serde::encode::Encode;

#[derive(Debug, Clone, PartialEq, AmqpComposite)]
#[amqp(name = "amqp:begin:list", code = 0x11)]
pub struct Begin {
    remote_channel: Option<u16>,
    next_outgoing_id: TransferNumber,
    incoming_window: u32,
    outgoing_window: u32,
    handle_max: Option<Handle>, // default: 4294967295
    offered_capabilities: Vec<Symbol>,
    desired_capabilities: Vec<Symbol>,
    properties: Fields,
}

impl Begin {
    pub fn encode(self) -> Vec<u8> {
        let enc: Primitive = self.into();
        enc.encode().into_bytes()
    }

    pub fn try_decode(composite: Composite, _body: &mut IntoIter<u8>) -> Result<Self, AppError> {
        Self::try_from(Primitive::from(composite))
    }
}
