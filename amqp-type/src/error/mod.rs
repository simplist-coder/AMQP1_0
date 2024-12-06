use crate::primitive::composite::{Composite, Descriptor};
use crate::error::amqp_error::AmqpError;
use crate::error::connection_error::ConnectionError;
use crate::error::link_error::LinkError;
use crate::error::session_error::SessionError;
use crate::primitive::variable_width::symbol::Symbol;
use crate::primitive::Primitive;
use crate::restricted::fields::Fields;
use crate::serde::decode::Decode;
use crate::serde::encode::{Encode, Encoded};
use std::fmt::{Display, Formatter};
use std::string::FromUtf8Error;
use std::vec::IntoIter;

pub mod amqp_error;
pub mod connection_error;
pub mod link_error;
pub mod session_error;

trait ErrorCondition {
    fn error_condition(&self) -> Symbol;
    fn amqp_description(&self) -> Option<String>;
    fn info(&self) -> Option<Fields>;
}

#[derive(Debug)]
pub enum AppError {
    Amqp(AmqpError),
    Connection(ConnectionError),
    Link(LinkError),
    Session(SessionError),
    SpecificationNonCompliantError,
}

impl std::error::Error for AppError {}

impl Display for AppError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            AppError::Amqp(x) => write!(f, "AmqpError: {}", x),
            AppError::Connection(x) => write!(f, "ConnectionError: {}", x),
            AppError::Link(x) => write!(f, "LinkError: {}", x),
            AppError::Session(x) => write!(f, "SessionError: {}", x),
            AppError::SpecificationNonCompliantError => write!(f, "SpecificationNonCompliantError"),
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
            AppError::SpecificationNonCompliantError => {
                panic!("This error must never be constructed in normal operations")
            }
        }
    }

    fn amqp_description(&self) -> Option<String> {
        match self {
            AppError::Amqp(x) => x.amqp_description(),
            AppError::Connection(x) => x.amqp_description(),
            AppError::Link(x) => x.amqp_description(),
            AppError::Session(x) => x.amqp_description(),
            AppError::SpecificationNonCompliantError => {
                panic!("This error must never be constructed in normal operations")
            }
        }
    }

    fn info(&self) -> Option<Fields> {
        match self {
            AppError::Amqp(x) => x.info(),
            AppError::Connection(x) => x.info(),
            AppError::Link(x) => x.info(),
            AppError::Session(x) => x.info(),
            AppError::SpecificationNonCompliantError => {
                panic!("This error must never be constructed in normal operations")
            }
        }
    }
}

impl Encode for AppError {
    fn encode(self) -> Encoded {
        let vec: Vec<Primitive> = vec![
            self.error_condition().into(),
            self.amqp_description().into(),
            self.info().into(),
        ];
        Composite::new(Descriptor::Code(0x1d), vec.into()).encode()
    }
}

impl Decode for AppError {
    fn try_decode(constructor: u8, stream: &mut IntoIter<u8>) -> Result<Self, AppError>
    where
        Self: Sized,
    {
        AppError::try_from(Composite::try_decode(constructor, stream)?)
    }
}

impl AppError {
    pub fn try_decode_without_constructor(stream: &mut IntoIter<u8>) -> Result<Self, AppError>
    where
        Self: Sized,
    {
        match stream.next() {
            None => Err(AppError::SpecificationNonCompliantError),
            Some(constructor) => Self::try_decode(constructor, stream),
        }
    }
}

impl TryFrom<Composite> for AppError {
    type Error = AppError;

    fn try_from(value: Composite) -> Result<Self, Self::Error> {
        let (_descriptor, list) = value.into_inner();

        let mut list = list.into_inner();
        let error = match list.len() {
            1 => (Some(list.remove(0)), None, None),
            2 => (Some(list.remove(0)), Some(list.remove(0)), None),
            3 => (Some(list.remove(0)), Some(list.remove(0)), Some(list.remove(0))),
            _ => Err(AppError::SpecificationNonCompliantError)?,
        };
        if let (Some(Primitive::Symbol(condition)), _, _) = &error {
            match condition.inner() {
                x if amqp_error::TAGS.contains(&x) => {
                    Err(AmqpError::try_from(error)?)?
                }
                x if connection_error::TAGS.contains(&x) => {
                    Err(ConnectionError::try_from(error)?)?
                }
                x if link_error::TAGS.contains(&x) => {
                    Err(LinkError::try_from(error)?)?
                }
                x if session_error::TAGS.contains(&x) => {
                    Err(SessionError::try_from(error)?)?
                }
                _ => Err(AppError::SpecificationNonCompliantError)
            }
        }
        else {
            Err(AppError::SpecificationNonCompliantError)
        }
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_round_trip_amqp_error() {
        let error = AppError::Amqp(AmqpError::DecodeError);
        let encoded = error.encode().into_bytes();
        assert!(matches!(
            AppError::try_decode_without_constructor(&mut encoded.into_iter()),
            Err(AppError::Amqp(AmqpError::DecodeError))
        ))
    }

    #[test]
    fn test_round_trip_connection_error() {
        let error = AppError::Connection(ConnectionError::ConnectionForced);
        let encoded = error.encode().into_bytes();
        assert!(matches!(
            AppError::try_decode_without_constructor(&mut encoded.into_iter()),
            Err(AppError::Connection(ConnectionError::ConnectionForced))
        ))
    }

    #[test]
    fn test_round_trip_link_error() {
        let error = AppError::Link(LinkError::DetachForced);
        let encoded = error.encode().into_bytes();
        assert!(matches!(
            AppError::try_decode_without_constructor(&mut encoded.into_iter()),
            Err(AppError::Link(LinkError::DetachForced))
        ))
    }

    #[test]
    fn test_round_trip_session_error() {
        let error = AppError::Session(SessionError::HandleInUse);
        let encoded = error.encode().into_bytes();
        assert!(matches!(
            AppError::try_decode_without_constructor(&mut encoded.into_iter()),
            Err(AppError::Session(SessionError::HandleInUse))
        ))
    }
}