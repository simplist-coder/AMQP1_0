use crate::error::{AppError, ErrorCondition};
use crate::primitive::variable_width::symbol::Symbol;
use crate::primitive::Primitive;
use crate::restricted::fields::Fields;
use std::fmt::{Display, Formatter};
use crate::config::Config;

const AMQP_INTERNAL_ERROR: &'static str = "amqp:internal-error";
const AMQP_NOT_FOUND: &'static str = "amqp:not-found";
const AMQP_UNAUTHORIZED_ACCESS: &'static str = "amqp:unauthorized-access";
const AMQP_DECODE_ERROR: &'static str = "amqp:decode-error";
const AMQP_RESOURCE_LIMITED_EXCEEDED: &'static str = "amqp:resource-limited-exceeded";
const AMQP_NOT_ALLOWED: &'static str = "amqp:not-allowed";
const AMQP_INVALID_FIELD: &'static str = "amqp:invalid-field";
const AMQP_NOT_IMPLEMENTED: &'static str = "amqp:not-implemented";
const AMQP_RESOURCE_LOCKED: &'static str = "amqp:resource-locked";
const AMQP_PRECONDITION_FAILED: &'static str = "amqp:precondition-failed";
const AMQP_RESOURCE_DELETED: &'static str = "amqp:resource-deleted";
const AMQP_ILLEGAL_STATE: &'static str = "amqp:illegal-state";
const AMQP_FRAME_SIZE_TOO_SMALL: &'static str = "amqp:frame-size-too-small";

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

pub(crate) const TAGS: [&'static str; 13] = [
    AMQP_INTERNAL_ERROR,
    AMQP_NOT_FOUND,
    AMQP_UNAUTHORIZED_ACCESS,
    AMQP_DECODE_ERROR,
    AMQP_RESOURCE_LIMITED_EXCEEDED,
    AMQP_NOT_ALLOWED,
    AMQP_INVALID_FIELD,
    AMQP_NOT_IMPLEMENTED,
    AMQP_RESOURCE_LOCKED,
    AMQP_PRECONDITION_FAILED,
    AMQP_RESOURCE_DELETED,
    AMQP_ILLEGAL_STATE,
    AMQP_FRAME_SIZE_TOO_SMALL,
];

impl Display for AmqpError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            AmqpError::InternalError => write!(f, "{}", AMQP_INTERNAL_ERROR),
            AmqpError::NotFound => write!(f, "{}", AMQP_NOT_FOUND),
            AmqpError::UnauthorizedAccess => write!(f, "{}", AMQP_UNAUTHORIZED_ACCESS),
            AmqpError::DecodeError => write!(f, "{}", AMQP_DECODE_ERROR),
            AmqpError::ResourceLimitExceeded => write!(f, "{}", AMQP_RESOURCE_LIMITED_EXCEEDED),
            AmqpError::NotAllowed => write!(f, "{}", AMQP_NOT_ALLOWED),
            AmqpError::InvalidField => write!(f, "{}", AMQP_INVALID_FIELD),
            AmqpError::NotImplemented => write!(f, "{}", AMQP_NOT_IMPLEMENTED),
            AmqpError::ResourceLocked => write!(f, "{}", AMQP_RESOURCE_LOCKED),
            AmqpError::PreconditionFailed => write!(f, "{}", AMQP_PRECONDITION_FAILED),
            AmqpError::ResourceDeleted => write!(f, "{}", AMQP_RESOURCE_DELETED),
            AmqpError::IllegalState => write!(f, "{}", AMQP_ILLEGAL_STATE),
            AmqpError::FrameSizeTooSmall => write!(f, "{}", AMQP_FRAME_SIZE_TOO_SMALL),
        }
    }
}

impl ErrorCondition for AmqpError {
    fn error_condition(&self) -> Symbol {
        self.to_string()
            .try_into()
            .expect("Error Condition Mapping must never fail.")
    }

    fn amqp_description(&self) -> Option<String> {
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

impl From<&str> for AmqpError {
    fn from(value: &str) -> Self {
        match value {
            AMQP_INTERNAL_ERROR => AmqpError::InternalError,
            AMQP_NOT_FOUND => AmqpError::NotFound,
            AMQP_UNAUTHORIZED_ACCESS => AmqpError::UnauthorizedAccess,
            AMQP_DECODE_ERROR => AmqpError::DecodeError,
            AMQP_RESOURCE_LIMITED_EXCEEDED => AmqpError::ResourceLimitExceeded,
            AMQP_NOT_ALLOWED => AmqpError::NotAllowed,
            AMQP_INVALID_FIELD => AmqpError::InvalidField,
            AMQP_NOT_IMPLEMENTED => AmqpError::NotImplemented,
            AMQP_RESOURCE_LOCKED => AmqpError::ResourceLocked,
            AMQP_PRECONDITION_FAILED => AmqpError::PreconditionFailed,
            AMQP_RESOURCE_DELETED => AmqpError::ResourceDeleted,
            AMQP_ILLEGAL_STATE => AmqpError::IllegalState,
            AMQP_FRAME_SIZE_TOO_SMALL => AmqpError::FrameSizeTooSmall,
            _ => panic!("Invalid AmqpError Symbol. This operation must never fail."),
        }
    }
}

impl TryFrom<(Option<Primitive>, Option<Primitive>, Option<Primitive>)> for AmqpError {
    type Error = AppError;
    fn try_from(
        (condition, _, _): (Option<Primitive>, Option<Primitive>, Option<Primitive>),
    ) -> Result<Self, Self::Error> {
        if let Some(Primitive::Symbol(s)) = condition {
            Err(AmqpError::from(s.inner()))?
        } else {
            Err(AppError::SpecificationNonCompliantError)
        }
    }
}

impl std::error::Error for AmqpError {}

#[cfg(test)]
mod tests {
    use super::*;

    fn tag_to_error_pattern() {}

    #[test]
    fn test_try_from_triple_for_all_amqp_errors() {
        assert!(matches!(
            AmqpError::try_from((
                Some(Primitive::Symbol(
                    Symbol::new(AMQP_INTERNAL_ERROR.into()).unwrap()
                )),
                None,
                None
            )),
            Err(AppError::Amqp(AmqpError::InternalError))
        ));
        assert!(matches!(
            AmqpError::try_from((
                Some(Primitive::Symbol(
                    Symbol::new(AMQP_NOT_FOUND.into()).unwrap()
                )),
                None,
                None
            )),
            Err(AppError::Amqp(AmqpError::NotFound))
        ));
        assert!(matches!(
            AmqpError::try_from((
                Some(Primitive::Symbol(
                    Symbol::new(AMQP_UNAUTHORIZED_ACCESS.into()).unwrap()
                )),
                None,
                None
            )),
            Err(AppError::Amqp(AmqpError::UnauthorizedAccess))
        ));
        assert!(matches!(
            AmqpError::try_from((
                Some(Primitive::Symbol(
                    Symbol::new(AMQP_DECODE_ERROR.into()).unwrap()
                )),
                None,
                None
            )),
            Err(AppError::Amqp(AmqpError::DecodeError))
        ));
        assert!(matches!(
            AmqpError::try_from((
                Some(Primitive::Symbol(
                    Symbol::new(AMQP_RESOURCE_LIMITED_EXCEEDED.into()).unwrap()
                )),
                None,
                None
            )),
            Err(AppError::Amqp(AmqpError::ResourceLimitExceeded))
        ));
        assert!(matches!(
            AmqpError::try_from((
                Some(Primitive::Symbol(
                    Symbol::new(AMQP_NOT_ALLOWED.into()).unwrap()
                )),
                None,
                None
            )),
            Err(AppError::Amqp(AmqpError::NotAllowed))
        ));
        assert!(matches!(
            AmqpError::try_from((
                Some(Primitive::Symbol(
                    Symbol::new(AMQP_INVALID_FIELD.into()).unwrap()
                )),
                None,
                None
            )),
            Err(AppError::Amqp(AmqpError::InvalidField))
        ));
        assert!(matches!(
            AmqpError::try_from((
                Some(Primitive::Symbol(
                    Symbol::new(AMQP_NOT_IMPLEMENTED.into()).unwrap()
                )),
                None,
                None
            )),
            Err(AppError::Amqp(AmqpError::NotImplemented))
        ));
        assert!(matches!(
            AmqpError::try_from((
                Some(Primitive::Symbol(
                    Symbol::new(AMQP_RESOURCE_LOCKED.into()).unwrap()
                )),
                None,
                None
            )),
            Err(AppError::Amqp(AmqpError::ResourceLocked))
        ));
        assert!(matches!(
            AmqpError::try_from((
                Some(Primitive::Symbol(
                    Symbol::new(AMQP_PRECONDITION_FAILED.into()).unwrap()
                )),
                None,
                None
            )),
            Err(AppError::Amqp(AmqpError::PreconditionFailed))
        ));
        assert!(matches!(
            AmqpError::try_from((
                Some(Primitive::Symbol(
                    Symbol::new(AMQP_RESOURCE_DELETED.into()).unwrap()
                )),
                None,
                None
            )),
            Err(AppError::Amqp(AmqpError::ResourceDeleted))
        ));
        assert!(matches!(
            AmqpError::try_from((
                Some(Primitive::Symbol(
                    Symbol::new(AMQP_ILLEGAL_STATE.into()).unwrap()
                )),
                None,
                None
            )),
            Err(AppError::Amqp(AmqpError::IllegalState))
        ));
        assert!(matches!(
            AmqpError::try_from((
                Some(Primitive::Symbol(
                    Symbol::new(AMQP_FRAME_SIZE_TOO_SMALL.into()).unwrap()
                )),
                None,
                None
            )),
            Err(AppError::Amqp(AmqpError::FrameSizeTooSmall))
        ));
    }


}
