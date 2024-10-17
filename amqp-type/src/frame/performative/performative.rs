use crate::error::AppError;
use crate::frame::performative::specifics::attach::Attach;
use crate::frame::performative::specifics::begin::Begin;
use crate::frame::performative::specifics::close::Close;
use crate::frame::performative::specifics::detach::Detach;
use crate::frame::performative::specifics::disposition::Disposition;
use crate::frame::performative::specifics::end::End;
use crate::frame::performative::specifics::flow::Flow;
use crate::frame::performative::specifics::open::Open;
use crate::frame::performative::specifics::transfer::Transfer;
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
    pub(crate) async fn try_decode(
        _stream: &mut Pin<Box<impl Stream<Item = u8>>>,
    ) -> Result<Self, AppError>
    where
        Self: Sized,
    {
        todo!()
    }
}
