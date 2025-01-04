use crate::error::AppError;
use crate::composite::Composite;
use std::vec::IntoIter;
use amqp_derive::AmqpComposite;
use crate::primitive::Primitive;
use crate::restricted::fields::Fields;
use crate::restricted::handle::Handle;
use crate::restricted::sequence_no::SequenceNumber;
use crate::restricted::transfer_number::TransferNumber;
use crate::serde::encode::Encode;

#[derive(Debug, Clone, PartialEq, AmqpComposite)]
#[amqp(name = "amqp:flow:list", code = 0x13)]
pub struct Flow {
    next_incoming_id: TransferNumber,
    incoming_window: u32,
    next_outgoing_id: TransferNumber,
    outgoing_window: u32,
    handle: Option<Handle>,
    delivery_count: Option<SequenceNumber>,
    link_credit: Option<u32>,
    available: Option<u32>,
    drain: Option<bool>, // default false
    echo: Option<bool>, // default false
    properties: Option<Fields>
}

impl Flow {
    pub fn encode(self) -> Vec<u8> {
        let enc: Primitive = self.into();
        enc.encode().into_bytes()
    }

    pub fn try_decode(composite: Composite, _body: &mut IntoIter<u8>) -> Result<Self, AppError> {
        Self::try_from(Primitive::from(composite))
    }
}
