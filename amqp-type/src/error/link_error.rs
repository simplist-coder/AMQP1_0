use crate::error::ErrorCondition;
use crate::primitive::variable_width::symbol::Symbol;
use crate::primitive::Primitive;
use crate::restricted::fields::Fields;
use indexmap::IndexMap;
use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub enum LinkError {
    DetachForced,
    TransferLimitExceeded,
    MessageSizeExceeded,
    Redirect {
        host_name: String,
        network_host: String,
        port: u16,
        address: String,
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

impl ErrorCondition for LinkError {
    fn error_condition(&self) -> Symbol {
        self.to_string()
            .try_into()
            .expect("LinkError to Symbol conversion must never fail.")
    }

    fn description(&self) -> Option<String> {
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
            LinkError::Redirect {
                host_name,
                network_host,
                port,
                address,
            } => {
                let mut map = IndexMap::with_capacity(4);
                map.insert(
                    Symbol::new("hostname".to_string()).expect("Must not fail"),
                    Primitive::String(host_name.clone()),
                );
                map.insert(
                    Symbol::new("network-host".to_string()).expect("Must not fail"),
                    Primitive::String(network_host.clone()),
                );
                map.insert(
                    Symbol::new("port".to_string()).expect("Must not fail"),
                    Primitive::Ushort(port.clone()),
                );
                map.insert(
                    Symbol::new("address".to_string()).expect("Must not fail"),
                    Primitive::String(address.clone()),
                );
                Some(Fields::new(map))
            }
        }
    }
}

impl std::error::Error for LinkError {}