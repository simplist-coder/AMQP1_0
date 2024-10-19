use crate::composite::performative::specifics::attach::Attach;
use crate::composite::performative::specifics::begin::Begin;
use crate::composite::performative::specifics::close::Close;
use crate::composite::performative::specifics::detach::Detach;
use crate::composite::performative::specifics::disposition::Disposition;
use crate::composite::performative::specifics::end::End;
use crate::composite::performative::specifics::flow::Flow;
use crate::composite::performative::specifics::open::Open;
use crate::composite::performative::specifics::transfer::Transfer;
use crate::error::AppError;
use std::pin::Pin;
use tokio_stream::Stream;

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
    pub async fn try_decode(
        _stream: &mut Pin<Box<impl Stream<Item = u8>>>,
    ) -> Result<Self, AppError>
    where
        Self: Sized,
    {
        todo!()
    }

    /// performatives always contain the payload,
    /// which is the rest of the frame body, after the performative
    pub fn payload(&self) -> Vec<u8> {
        todo!()
    }
}
