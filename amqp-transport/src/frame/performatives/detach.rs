use amqp_error::AppError;
use amqp_type::composite::Composite;
use std::vec::IntoIter;

#[derive(Debug, Copy, Clone)]
pub struct Detach {}

impl Detach {
    pub fn encode(self) -> Vec<u8> {
        todo!()
    }

    pub fn try_decode(composite: Composite, body: &mut IntoIter<u8>) -> Result<Self, AppError> {
        todo!()
    }
}
