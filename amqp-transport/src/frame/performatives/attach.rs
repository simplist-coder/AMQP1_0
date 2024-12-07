use amqp_type::error::AppError;
use amqp_type::primitive::composite::transport::source::Source;
use amqp_type::primitive::composite::transport::target::Target;
use amqp_type::primitive::composite::Composite;
use amqp_type::primitive::compound::map::Map;
use amqp_type::primitive::variable_width::symbol::Symbol;
use amqp_type::restricted::fields::Fields;
use amqp_type::restricted::handle::Handle;
use amqp_type::restricted::receiver_settle_mode::ReceiverSettleMode;
use amqp_type::restricted::role::Role;
use amqp_type::restricted::sender_settle_mode::SenderSettleMode;
use amqp_type::restricted::sequence_no::SequenceNumber;
use std::vec::IntoIter;

#[derive(Debug, Clone, PartialEq)]
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
    pub fn encode(self) -> Vec<u8> {
        todo!()
    }

    pub fn try_decode(_composite: Composite, _body: &mut IntoIter<u8>) -> Result<Self, AppError> {
        todo!()
    }
}
