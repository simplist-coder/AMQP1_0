use crate::constants::{
    PERFORMATIVE_CODE_ATTACH, PERFORMATIVE_CODE_BEGIN, PERFORMATIVE_CODE_CLOSE,
    PERFORMATIVE_CODE_DETACH, PERFORMATIVE_CODE_DISPOSITION, PERFORMATIVE_CODE_END,
    PERFORMATIVE_CODE_FLOW, PERFORMATIVE_CODE_OPEN, PERFORMATIVE_CODE_TRANSFER,
    PERFORMATIVE_SYMBOL_ATTACH, PERFORMATIVE_SYMBOL_BEGIN, PERFORMATIVE_SYMBOL_CLOSE,
    PERFORMATIVE_SYMBOL_DETACH, PERFORMATIVE_SYMBOL_DISPOSITION, PERFORMATIVE_SYMBOL_END,
    PERFORMATIVE_SYMBOL_FLOW, PERFORMATIVE_SYMBOL_OPEN, PERFORMATIVE_SYMBOL_TRANSFER,
};
use crate::frame::performatives::attach::Attach;
use crate::frame::performatives::begin::Begin;
use crate::frame::performatives::close::Close;
use crate::frame::performatives::detach::Detach;
use crate::frame::performatives::disposition::Disposition;
use crate::frame::performatives::end::End;
use crate::frame::performatives::flow::Flow;
use crate::frame::performatives::open::Open;
use crate::frame::performatives::transfer::Transfer;
use amqp_type::error::AppError;
use amqp_type::composite::{Composite, Descriptor};
use std::vec::IntoIter;
use amqp_type::error::amqp_error::AmqpError;

#[derive(Debug, Clone)]
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
        match self {
            Performative::Open(x) => x.encode(),
            Performative::Begin(x) => x.encode(),
            Performative::Attach(x) => x.encode(),
            Performative::Flow(x) => x.encode(),
            Performative::Transfer(x) => x.encode(),
            Performative::Disposition(x) => x.encode(),
            Performative::Detach(x) => x.encode(),
            Performative::End(x) => x.encode(),
            Performative::Close(x) => x.encode(),
        }
    }

    pub fn try_decode(stream: &mut IntoIter<u8>) -> Result<Self, AppError> {
        let composite = Composite::try_decode_without_constructor(stream)?;
        let descriptor = composite.descriptor().clone();
        match descriptor {
            Descriptor::Symbol(symbol) => {
                Self::try_decode_from_symbol(symbol.as_str(), composite, stream)
            }
            Descriptor::Code(code) => Self::try_decode_from_code(code, composite, stream),
        }
    }

    fn try_decode_from_code(
        code: u64,
        composite: Composite,
        stream: &mut IntoIter<u8>,
    ) -> Result<Performative, AppError> {
        match code {
            PERFORMATIVE_CODE_OPEN => Ok(Open::try_decode(composite, stream)?.into()),
            PERFORMATIVE_CODE_BEGIN => Ok(Begin::try_decode(composite, stream)?.into()),
            PERFORMATIVE_CODE_ATTACH => Ok(Attach::try_decode(composite, stream)?.into()),
            PERFORMATIVE_CODE_FLOW => Ok(Flow::try_decode(composite, stream)?.into()),
            PERFORMATIVE_CODE_TRANSFER => Ok(Transfer::try_decode(composite, stream)?.into()),
            PERFORMATIVE_CODE_DISPOSITION => Ok(Disposition::try_decode(composite, stream)?.into()),
            PERFORMATIVE_CODE_DETACH => Ok(Detach::try_decode(composite, stream)?.into()),
            PERFORMATIVE_CODE_END => Ok(End::try_decode(composite, stream)?.into()),
            PERFORMATIVE_CODE_CLOSE => Ok(Close::try_decode(composite, stream)?.into()),
            _ => Err(AmqpError::DecodeError)?
        }
    }

    fn try_decode_from_symbol(
        symbol: &str,
        composite: Composite,
        stream: &mut IntoIter<u8>,
    ) -> Result<Self, AppError> {
        match symbol {
            PERFORMATIVE_SYMBOL_OPEN => Ok(Open::try_decode(composite, stream)?.into()),
            PERFORMATIVE_SYMBOL_BEGIN => Ok(Begin::try_decode(composite, stream)?.into()),
            PERFORMATIVE_SYMBOL_ATTACH => Ok(Attach::try_decode(composite, stream)?.into()),
            PERFORMATIVE_SYMBOL_FLOW => Ok(Flow::try_decode(composite, stream)?.into()),
            PERFORMATIVE_SYMBOL_TRANSFER => Ok(Transfer::try_decode(composite, stream)?.into()),
            PERFORMATIVE_SYMBOL_DISPOSITION => {
                Ok(Disposition::try_decode(composite, stream)?.into())
            }
            PERFORMATIVE_SYMBOL_DETACH => Ok(Detach::try_decode(composite, stream)?.into()),
            PERFORMATIVE_SYMBOL_END => Ok(End::try_decode(composite, stream)?.into()),
            PERFORMATIVE_SYMBOL_CLOSE => Ok(Close::try_decode(composite, stream)?.into()),
            _ => Err(AmqpError::DecodeError)?
        }
    }

    // performatives always contain the payload,
    // which is the rest of the frame body, after the performative
    pub fn payload(&self) -> Vec<u8> {
        match self {
            Performative::Open(_) => todo!(),
            Performative::Begin(_) => todo!(),
            Performative::Attach(_) => todo!(),
            Performative::Flow(_) => todo!(),
            Performative::Transfer(_) => todo!(),
            Performative::Disposition(_) => todo!(),
            Performative::Detach(_) => todo!(),
            Performative::End(_) => todo!(),
            Performative::Close(_) => todo!(),
        }
    }
}

impl From<Open> for Performative {
    fn from(value: Open) -> Self {
        Performative::Open(value)
    }
}

impl From<Begin> for Performative {
    fn from(value: Begin) -> Self {
        Performative::Begin(value)
    }
}

impl From<Attach> for Performative {
    fn from(value: Attach) -> Self {
        Performative::Attach(value)
    }
}

impl From<Flow> for Performative {
    fn from(value: Flow) -> Self {
        Performative::Flow(value)
    }
}

impl From<Transfer> for Performative {
    fn from(value: Transfer) -> Self {
        Performative::Transfer(value)
    }
}

impl From<Disposition> for Performative {
    fn from(value: Disposition) -> Self {
        Performative::Disposition(value)
    }
}

impl From<Detach> for Performative {
    fn from(value: Detach) -> Self {
        Performative::Detach(value)
    }
}

impl From<End> for Performative {
    fn from(value: End) -> Self {
        Performative::End(value)
    }
}

impl From<Close> for Performative {
    fn from(value: Close) -> Self {
        Performative::Close(value)
    }
}

#[cfg(test)]
mod tests {}
