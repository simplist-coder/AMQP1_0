use crate::error::ErrorCondition;
use crate::primitive::variable_width::symbol::Symbol;
use crate::restricted::fields::Fields;
use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub enum AmqpError {
    InternalError,
    NotFound,
    UnauthorizedAccess,
    DecodeError,
    ResourceLimitExceeded,
    NotAllowed,
    InvalidField,
    NotImplemented,
    ResourceLocked,
    PreconditionFailed,
    ResourceDeleted,
    IllegalState,
    FrameSizeTooSmall,
}

impl Display for AmqpError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            AmqpError::InternalError => write!(f, "amq:internal-error"),
            AmqpError::NotFound => write!(f, "amq:not-found"),
            AmqpError::UnauthorizedAccess => write!(f, "amq:unauthorized-access"),
            AmqpError::DecodeError => write!(f, "amq:decode-error"),
            AmqpError::ResourceLimitExceeded => write!(f, "amq:resource-limited-exceeded"),
            AmqpError::NotAllowed => write!(f, "amq:not-allowed"),
            AmqpError::InvalidField => write!(f, "amq:invalid-field"),
            AmqpError::NotImplemented => write!(f, "amq:not-implemented"),
            AmqpError::ResourceLocked => write!(f, "amq:resource-locked"),
            AmqpError::PreconditionFailed => write!(f, "amq:precondition-failed"),
            AmqpError::ResourceDeleted => write!(f, "amq:resource-deleted"),
            AmqpError::IllegalState => write!(f, "amq:illegal-state"),
            AmqpError::FrameSizeTooSmall => write!(f, "amq:frame-size-too-small"),
        }
    }
}

impl ErrorCondition for AmqpError {
    fn error_condition(&self) -> Symbol {
        self.to_string()
            .try_into()
            .expect("Error Condition Mapping must never fail.")
    }

    fn description(&self) -> Option<String> {
        let desc = match self {
            AmqpError::InternalError => "An internal error occurred. Operator intervention may be required to resume normal operation.",
            AmqpError::NotFound => "A peer attempted to work with a remote entity that does not exist.",
            AmqpError::UnauthorizedAccess => "A peer attempted to work with a remote entity to which it has no access due to security settings.",
            AmqpError::DecodeError => "Data could not be decoded.",
            AmqpError::ResourceLimitExceeded => "A peer exceeded its resource allocation.",
            AmqpError::NotAllowed => "The peer tried to use a frame in a manner that is inconsistent with the semantics defined in the specification.",
            AmqpError::InvalidField => "An invalid field was passed in a frame body, and the operation could not proceed.",
            AmqpError::NotImplemented => "The peer tried to use functionality that is not implemented in its partner.",
            AmqpError::ResourceLocked => "The client attempted to work with a server entity to which it has no access because another client is working with it.",
            AmqpError::PreconditionFailed => "The client made a request that was not allowed because some precondition failed.",
            AmqpError::ResourceDeleted => "A server entity the client is working with has been deleted.",
            AmqpError::IllegalState => "The peer sent a frame that is not permitted in the current state of the Session.",
            AmqpError::FrameSizeTooSmall => "The peer cannot send a frame because the smallest encoding of the performative
                                                        with the currently valid values would be too large to fit within a frame of the agreed
                                                        maximum frame size. When transferring a message the message data can be sent in
                                                        multiple transfer frames thereby avoiding this error. Similarly when attaching a link
                                                        with a large unsettled map the endpoint may make use of the incomplete-unsettled
                                                        flag to avoid the need for overly large frames.",
        }.to_string();
        Some(desc)
    }

    fn info(&self) -> Option<Fields> {
        None
    }
}

impl std::error::Error for AmqpError {}