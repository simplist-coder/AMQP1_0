use crate::error::{AppError, ErrorCondition};
use crate::primitive::variable_width::symbol::Symbol;
use crate::restricted::fields::Fields;
use std::fmt::{Display, Formatter};
use crate::config::Config;
use crate::primitive::Primitive;

const AMQP_SESSION_WINDOW_VIOLATION: &'static str = "amqp:session:window-violation";
const AMQP_SESSION_ERRANT_LINK: &'static str = "amqp:session:errant-link";
const AMQP_SESSION_HANDLE_IN_USE: &'static str = "amqp:session:handle-in-use";
const AMQP_SESSION_UNATTACHED_HANDLE: &'static str = "amqp:session:unattached-handle";

pub(crate) const TAGS: [&'static str; 4] = [
    AMQP_SESSION_WINDOW_VIOLATION,
    AMQP_SESSION_ERRANT_LINK,
    AMQP_SESSION_HANDLE_IN_USE,
    AMQP_SESSION_UNATTACHED_HANDLE,
];

#[derive(Debug)]
pub enum SessionError {
    WindowViolation,
    ErrantLink,
    HandleInUse,
    UnattachedHandle,
}

impl Display for SessionError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            SessionError::WindowViolation => write!(f, "amqp:session:window-violation"),
            SessionError::ErrantLink => write!(f, "amqp:session:errant-link"),
            SessionError::HandleInUse => write!(f, "amqp:session:handle-in-use"),
            SessionError::UnattachedHandle => write!(f, "amqp:session:unattached-handle"),
        }
    }
}

impl ErrorCondition for SessionError {
    fn error_condition(&self) -> Symbol {
        self.to_string()
            .try_into()
            .expect("SessionError to Symbol conversion must never fail.")
    }

    fn amqp_description(&self) -> Option<String> {
        let desc = match self {
            SessionError::WindowViolation => "The peer violated incoming window for the session.",
            SessionError::ErrantLink => "Input was received for a link that was detached with an error",
            SessionError::HandleInUse => "An attach was received using a handle that is already in use for an attached Link.",
            SessionError::UnattachedHandle => "A frame (other than attach) was received referencing a handle which is not currently in use of an attached Link.",
        }.to_string();
        Some(desc)
    }

    fn info(&self) -> Option<Fields> {
        None
    }
}

impl TryFrom<(Option<Primitive>, Option<Primitive>, Option<Primitive>)> for SessionError {
    type Error = AppError;

    fn try_from(value: (Option<Primitive>, Option<Primitive>, Option<Primitive>)) -> Result<Self, Self::Error> {
        todo!()
    }
}
impl std::error::Error for SessionError {}
