use crate::error::AppError;
use crate::composite::Composite;
use std::vec::IntoIter;

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Disposition {}

impl Disposition {
    pub fn encode(self) -> Vec<u8> {
        todo!()
    }

    pub fn try_decode(_composite: Composite, _body: &mut IntoIter<u8>) -> Result<Self, AppError> {
        todo!()
    }
}
