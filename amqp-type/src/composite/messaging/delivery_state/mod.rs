use amqp_derive::AmqpComposite;
use crate::composite::messaging::delivery_state::accepted::Accepted;
use crate::composite::messaging::delivery_state::modified::Modified;
use crate::composite::messaging::delivery_state::received::Received;
use crate::composite::messaging::delivery_state::rejected::Rejected;
use crate::composite::messaging::delivery_state::released::Released;

pub mod accepted;
pub mod modified;
pub mod received;
pub mod rejected;
pub mod released;

#[derive(Debug, Clone, PartialEq, AmqpComposite)]
pub enum DeliveryState {
    Received(Received),
    Accepted(Accepted),
    Rejected(Rejected),
    Released(Released),
    Modified(Modified),
}
