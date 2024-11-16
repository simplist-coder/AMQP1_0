use crate::error::ErrorCondition;
use crate::primitive::variable_width::symbol::Symbol;
use crate::primitive::Primitive;
use crate::restricted::fields::Fields;
use indexmap::IndexMap;
use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub enum ConnectionError {
    ConnectionForced,
    FramingError,
    Redirect {
        host_name: String,
        network_host: String,
        port: u16,
    },
}

impl Display for ConnectionError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ConnectionError::ConnectionForced => write!(f, "amqp:connection:forced"),
            ConnectionError::FramingError => write!(f, "amqp:connection:framing-error"),
            ConnectionError::Redirect{..} => write!(f, "amqp:connection:redirect"),
        }
    }
}

impl ErrorCondition for ConnectionError {
    fn error_condition(&self) -> Symbol {
        self.to_string()
            .try_into()
            .expect("ConnectionError to Symbol conversion must never fail.")
    }

    fn description(&self) -> Option<String> {
        let desc = match self {
            ConnectionError::ConnectionForced => "An operator intervened to close the Connection for some reason. The client may retry at some later date.",
            ConnectionError::FramingError => "A valid frame header cannot be formed from the incoming byte stream.",
            ConnectionError::Redirect{..} => "The container is no longer available on the current connection. The peer should attempt reconnection to the container using the details provided in the info map."
        }.to_string();
        Some(desc)
    }

    fn info(&self) -> Option<Fields> {
        match self {
            ConnectionError::ConnectionForced => None,
            ConnectionError::FramingError => None,
            ConnectionError::Redirect{host_name, network_host, port} => {
                let mut map = IndexMap::with_capacity(3);
                map.insert(Symbol::new("hostname".to_string()).expect("Must not fail"), Primitive::String(host_name.clone()));
                map.insert(Symbol::new("network-host".to_string()).expect("Must not fail"), Primitive::String(network_host.clone()));
                map.insert(Symbol::new("port".to_string()).expect("Must not fail"), Primitive::Ushort(port.clone()));
                Some(Fields::new(map))
            }
        }
    }
}

impl std::error::Error for ConnectionError {}