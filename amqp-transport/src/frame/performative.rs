use crate::frame::performatives::attach::Attach;
use crate::frame::performatives::begin::Begin;
use crate::frame::performatives::close::Close;
use crate::frame::performatives::detach::Detach;
use crate::frame::performatives::disposition::Disposition;
use crate::frame::performatives::end::End;
use crate::frame::performatives::flow::Flow;
use crate::frame::performatives::open::Open;
use crate::frame::performatives::transfer::Transfer;
use amqp_error::AppError;
use std::vec::IntoIter;

#[derive(Debug, Copy, Clone)]
pub enum Performative {
    Open(Open),
    Begin(Begin),
    Attach(Attach),
    Flow(Flow),
    Transfer(Transfer),
    Disposition(Disposition),
    Detach(Detach),
    End(End),
    Close(Close),
}

impl Performative {
    pub fn encode(self) -> Vec<u8> {
        todo!()
    }

    pub fn try_decode(_stream: &mut IntoIter<u8>) -> Result<Self, AppError>
    where
        Self: Sized,
    {
        todo!()
    }

    // performatives always contain the payload,
    // which is the rest of the frame body, after the performative
    pub fn payload(&self) -> Vec<u8> {
        todo!()
    }
}
