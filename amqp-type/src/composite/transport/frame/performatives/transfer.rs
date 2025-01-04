use crate::error::AppError;
use crate::composite::Composite;
use std::vec::IntoIter;
use amqp_derive::AmqpComposite;
use crate::composite::messaging::delivery_state::DeliveryState;
use crate::primitive::Primitive;
use crate::restricted::delivery_number::DeliveryNumber;
use crate::restricted::delivery_tag::DeliveryTag;
use crate::restricted::handle::Handle;
use crate::restricted::message_format::MessageFormat;
use crate::restricted::receiver_settle_mode::ReceiverSettleMode;
use crate::serde::encode::Encode;

#[derive(Debug, Clone, PartialEq, AmqpComposite)]
#[amqp(name = "amqp:transfer:list", code = 0x14)]
pub struct Transfer {
    handle: Handle,
    delivery_id: Option<DeliveryNumber>,
    delivery_tag: Option<DeliveryTag>,
    message_format: Option<MessageFormat>,
    settled: Option<bool>,
    more: Option<bool>, // default: false
    rcv_settle_mode: Option<ReceiverSettleMode>,
    state: Option<DeliveryState>,
    resume: Option<bool>, // default: false
    aborted: Option<bool>, // default: false
    batchable: Option<bool>, // default: false


}

impl Transfer {
    pub fn encode(self) -> Vec<u8> {
        let enc: Primitive = self.into();
        enc.encode().into_bytes()
    }

    pub fn try_decode(composite: Composite, _body: &mut IntoIter<u8>) -> Result<Self, AppError> {
        Self::try_from(Primitive::from(composite))
    }
}
