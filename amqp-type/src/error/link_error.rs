use crate::error::{AppError, ErrorCondition};
use crate::primitive::variable_width::symbol::Symbol;
use crate::primitive::Primitive;
use crate::restricted::fields::Fields;
use indexmap::IndexMap;
use std::env;
use std::fmt::{Display, Formatter};

const AMQP_LINK_DETACH_FORCED: &str = "amqp:link:detach-forced";
const AMQP_LINK_TRANSFER_LIMIT_EXCEEDED: &str = "amqp:link:transfer-limit-exceeded";
const AMQP_LINK_MESSAGE_SIZE_EXCEEDED: &str = "amqp:link:message-size-exceeded";
const AMQP_LINK_REDIRECT: &str = "amqp:link:redirect";
const AMQP_LINK_STOLEN: &str = "amqp:link:stolen";

pub(crate) const TAGS: [&str; 5] = [
    AMQP_LINK_DETACH_FORCED,
    AMQP_LINK_TRANSFER_LIMIT_EXCEEDED,
    AMQP_LINK_MESSAGE_SIZE_EXCEEDED,
    AMQP_LINK_REDIRECT,
    AMQP_LINK_STOLEN,
];

#[derive(Debug)]
pub enum LinkError {
    DetachForced,
    TransferLimitExceeded,
    MessageSizeExceeded,
    Redirect {
        host_name: Option<String>,
        network_host: Option<String>,
        port: Option<u16>,
        address: Option<String>,
    },
    Stolen,
}

impl Display for LinkError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            LinkError::DetachForced => write!(f, "amqp:link:detach-forced"),
            LinkError::TransferLimitExceeded => write!(f, "amqp:link:transfer-limit-exceeded"),
            LinkError::MessageSizeExceeded => write!(f, "amqp:link:message-size-exceeded"),
            LinkError::Redirect { .. } => write!(f, "amqp:link:redirect"),
            LinkError::Stolen => write!(f, "amqp:link:stolen"),
        }
    }
}

const HOST_NAME: &str = "hostname";
const NETWORK_HOST: &str = "network-host";
const PORT: &str = "port";
const ADDRESS: &str = "address";

impl ErrorCondition for LinkError {
    fn error_condition(&self) -> Symbol {
        self.to_string()
            .try_into()
            .expect("LinkError to Symbol conversion must never fail.")
    }

    fn amqp_description(&self) -> Option<String> {
        let desc = match self {
            LinkError::DetachForced => "An operator intervened to detach for some reason.",
            LinkError::TransferLimitExceeded => "The peer sent more Message transfers than currently allowed on the link.",
            LinkError::MessageSizeExceeded => "The peer sent a larger message than is supported on the link.",
            LinkError::Redirect { .. } => "The address provided cannot be resolved to a terminus at the current container. \
                                            The info map may contain the following information to allow the client to locate the attach to the terminus.",
            LinkError::Stolen => "The link has been attached elsewhere, causing the existing attachment to be forcibly closed."
        }.to_string();
        Some(desc)
    }

    fn info(&self) -> Option<Fields> {
        match self {
            LinkError::DetachForced => None,
            LinkError::TransferLimitExceeded => None,
            LinkError::MessageSizeExceeded => None,
            LinkError::Stolen => None,
            LinkError::Redirect {..} => {
                let host_name = env::var("AMQP_LINK_REDIRECT_HOST_NAME").ok();
                let network_host = env::var("AMQP_LINK_REDIRECT_NETWORK_HOST").ok();
                let address = env::var("AMQP_LINK_REDIRECT_ADDRESS").ok();
                let port = env::var("AMQP_LINK_REDIRECT_PORT")
                    .ok()
                    .map(|port| port.parse::<u16>().ok())
                    .expect("Port must be parsable into a u16.");
                let mut map = IndexMap::with_capacity(4);
                map.insert(
                    Symbol::new(HOST_NAME.to_string()).expect("Must not fail"),
                    host_name.into(),
                );
                map.insert(
                    Symbol::new(NETWORK_HOST.to_string()).expect("Must not fail"),
                    network_host.into(),
                );
                map.insert(
                    Symbol::new(PORT.to_string()).expect("Must not fail"),
                    port.into(),
                );
                map.insert(
                    Symbol::new(ADDRESS.to_string()).expect("Must not fail"),
                    address.into(),
                );
                Some(Fields::new(map))
            }
        }
    }
}

impl TryFrom<(Option<Primitive>, Option<Primitive>, Option<Primitive>)> for LinkError {
    type Error = AppError;

    fn try_from((condition, _, info): (Option<Primitive>, Option<Primitive>, Option<Primitive>)) -> Result<Self, Self::Error> {
        if let Some(Primitive::Symbol(s)) = condition {
            match s.inner() {
                AMQP_LINK_DETACH_FORCED => Err(LinkError::DetachForced)?,
                AMQP_LINK_TRANSFER_LIMIT_EXCEEDED => Err(LinkError::TransferLimitExceeded)?,
                AMQP_LINK_MESSAGE_SIZE_EXCEEDED => Err(LinkError::MessageSizeExceeded)?,
                AMQP_LINK_STOLEN => Err(LinkError::Stolen)?,
                AMQP_LINK_REDIRECT => {
                    if let Some(Primitive::Map(info)) = info {
                        let mut values = info.into_inner();
                        let address = values
                            .remove(&Primitive::Symbol(Symbol::with_ascii(ADDRESS)))
                            .and_then(|v| v.try_into().ok());
                        let port = values
                            .remove(&Primitive::Symbol(Symbol::with_ascii(PORT)))
                            .and_then(|v| v.try_into().ok());
                        let network_host = values
                            .remove(&Primitive::Symbol(Symbol::with_ascii(NETWORK_HOST)))
                            .and_then(|v| v.try_into().ok());
                        let host_name = values
                            .remove(&Primitive::Symbol(Symbol::with_ascii(HOST_NAME)))
                            .and_then(|v| v.try_into().ok());
                        Err(LinkError::Redirect {
                            host_name,
                            network_host,
                            port,
                            address
                        })?
                    } else {
                        Err(LinkError::Redirect {
                            host_name: None,
                            network_host: None,
                            port: None,
                            address: None
                        })?
                    }
                },
                _ => Err(AppError::SpecificationNonCompliantError),
            }
        } else {
            Err(AppError::SpecificationNonCompliantError)
        }
    }
}

impl std::error::Error for LinkError {}


#[cfg(test)]
mod tests {
    use super::*;


    #[test]
    fn test_try_from_link_error_detach_forced() {
        let error = (
            Some(Primitive::Symbol(
                Symbol::with_ascii(AMQP_LINK_DETACH_FORCED),
            )),
            None,
            None,
        );
        assert!(matches!(
            LinkError::try_from(error),
            Err(AppError::Link(LinkError::DetachForced))
        ));
    }
    #[test]
    fn test_try_from_link_error_transfer_limit_exceeded() {
        let error = (
            Some(Primitive::Symbol(
                Symbol::with_ascii(AMQP_LINK_TRANSFER_LIMIT_EXCEEDED),
            )),
            None,
            None,
        );
        assert!(matches!(
            LinkError::try_from(error),
            Err(AppError::Link(LinkError::TransferLimitExceeded))
        ));
    }
    #[test]
    fn test_try_from_link_error_message_size_exceeded() {
        let error = (
            Some(Primitive::Symbol(
                Symbol::with_ascii(AMQP_LINK_MESSAGE_SIZE_EXCEEDED),
            )),
            None,
            None,
        );
        assert!(matches!(
            LinkError::try_from(error),
            Err(AppError::Link(LinkError::MessageSizeExceeded))
        ));
    }
    #[test]
    fn test_try_from_link_error_stolen() {
        let error = (
            Some(Primitive::Symbol(
                Symbol::with_ascii(AMQP_LINK_STOLEN),
            )),
            None,
            None,
        );
        assert!(matches!(
            LinkError::try_from(error),
            Err(AppError::Link(LinkError::Stolen))
        ));
    }
    #[test]
    fn test_try_from_link_error_redirect() {
        let redirect_args = {
            let mut args = IndexMap::new();
            args.insert(Symbol::with_ascii(HOST_NAME), "localhost".into());
            args.insert(Symbol::with_ascii(NETWORK_HOST), "127.0.0.1".into());
            args.insert(Symbol::with_ascii(PORT), 9876_u16.into());
            args.insert(Symbol::with_ascii(ADDRESS), "15".into());
            Some(Fields::new(args).into())
        };
        let error = (
            Some(Primitive::Symbol(
                Symbol::with_ascii(AMQP_LINK_REDIRECT),
            )),
            None,
            redirect_args
        );
        match LinkError::try_from(error) {
            Err(AppError::Link(LinkError::Redirect { host_name, network_host, port, address })) => {
                assert_eq!(host_name, Some("localhost".to_string()));
                assert_eq!(port, Some(9876_u16));
                assert_eq!(address, Some("15".into()));
                assert_eq!(network_host, Some("127.0.0.1".to_string()));
            }
            result => panic!("Expected LinkError::Redirect but was: {:?}", result),
        }
    }

    #[test]
    fn test_link_error_info() {
        env::set_var("AMQP_LINK_REDIRECT_HOST_NAME", "localhost");
        env::set_var("AMQP_LINK_REDIRECT_NETWORK_HOST", "127.0.0.1");
        env::set_var("AMQP_LINK_REDIRECT_PORT", "9876");
        env::set_var("AMQP_LINK_REDIRECT_ADDRESS", "15");
        let expected = Fields::new([
            (Symbol::with_ascii(HOST_NAME), Primitive::from(Some("localhost".to_string()))),
            (Symbol::with_ascii(NETWORK_HOST), Some("127.0.0.1".to_string()).into()),
            (Symbol::with_ascii(PORT), Some(9876_u16).into()),
            (Symbol::with_ascii(ADDRESS), Some("15".to_string()).into()),
        ].into());

        assert_eq!(LinkError::DetachForced.info(), None);
        assert_eq!(LinkError::TransferLimitExceeded.info(), None);
        assert_eq!(LinkError::MessageSizeExceeded.info(), None);
        assert_eq!(LinkError::Redirect { host_name: None, network_host: None, port: None, address: None }.info().unwrap(), expected);
    }
}