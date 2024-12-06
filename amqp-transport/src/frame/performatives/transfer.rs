use amqp_type::error::AppError;
use amqp_type::primitive::composite::Composite;
use std::vec::IntoIter;

#[derive(Debug, Copy, Clone)]
pub struct Transfer {}

impl Transfer {
    pub fn encode(self) -> Vec<u8> {
        todo!()
    }

    pub fn try_decode(_composite: Composite, _body: &mut IntoIter<u8>) -> Result<Self, AppError> {
        todo!()
    }
}
