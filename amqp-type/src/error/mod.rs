use crate::error::amqp_error::AmqpError;
use crate::error::connection_error::ConnectionError;
use crate::error::link_error::LinkError;
use crate::error::session_error::SessionError;
use crate::primitive::variable_width::symbol::Symbol;
use crate::restricted::fields::Fields;
use crate::serde::decode::Decode;
use crate::serde::encode::{Encode, Encoded};
use std::fmt::{Display, Formatter};
use std::string::FromUtf8Error;
use std::vec::IntoIter;

pub mod amqp_error;
pub mod connection_error;
pub mod session_error;
pub mod link_error;

trait ErrorCondition {
    fn error_condition(&self) -> Symbol;
    fn description(&self) -> Option<String>;
    fn info(&self) -> Option<Fields>;
}

#[derive(Debug)]
pub enum AppError {
    Amqp(AmqpError),
    Connection(ConnectionError),
    Link(LinkError),
    Session(SessionError),
}

impl std::error::Error for AppError {}

impl Display for AppError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            AppError::Amqp(x) => write!(f, "AmqpError: {}", x),
            AppError::Connection(x) => write!(f, "ConnectionError: {}", x),
            AppError::Link(x) => write!(f, "LinkError: {}", x),
            AppError::Session(x) => write!(f, "SessionError: {}", x),
        }
    }
}

impl ErrorCondition for AppError {
    fn error_condition(&self) -> Symbol {
        match self {
            AppError::Amqp(x) => x.error_condition(),
            AppError::Connection(x) => x.error_condition(),
            AppError::Link(x) => x.error_condition(),
            AppError::Session(x) => x.error_condition(),
        }
    }

    fn description(&self) -> Option<String> {
        match self {
            AppError::Amqp(x) => x.description(),
            AppError::Connection(x) => x.description(),
            AppError::Link(x) => x.description(),
            AppError::Session(x) => x.description(),
        }
    }

    fn info(&self) -> Option<Fields> {
        match self {
            AppError::Amqp(x) => x.info(),
            AppError::Connection(x) => x.info(),
            AppError::Link(x) => x.info(),
            AppError::Session(x) => x.info(),
        }
    }
}

impl Encode for AppError {
    fn encode(self) -> Encoded {
        todo!()
    }
}

impl Decode for AppError {
    fn try_decode(_constructor: u8, _stream: &mut IntoIter<u8>) -> Result<Self, AppError>
    where
        Self: Sized
    {
        todo!()
    }
}

impl From<AmqpError> for AppError {
    fn from(error: AmqpError) -> Self {
        AppError::Amqp(error)
    }
}

impl From<ConnectionError> for AppError {
    fn from(error: ConnectionError) -> Self {
        AppError::Connection(error)
    }
}

impl From<LinkError> for AppError {
    fn from(error: LinkError) -> Self {
        AppError::Link(error)
    }
}

impl From<SessionError> for AppError {
    fn from(error: SessionError) -> Self {
        AppError::Session(error)
    }
}

impl From<FromUtf8Error> for AppError {
    fn from(_: FromUtf8Error) -> Self {
        AppError::Amqp(AmqpError::InvalidField)
    }
}
