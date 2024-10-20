use amqp_error::AppError;
use amqp_type::composite::Composite;
use std::vec::IntoIter;

#[derive(Debug, Copy, Clone)]
pub struct Attach {}

impl Attach {
    pub fn encode(self) -> Vec<u8> {
        todo!()
    }

    pub fn try_decode(stream: Composite, stream0: &mut IntoIter<u8>) -> Result<Self, AppError> {
        todo!()
    }
}
