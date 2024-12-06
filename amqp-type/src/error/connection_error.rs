use crate::error::{AppError, ErrorCondition};
use crate::primitive::variable_width::symbol::Symbol;
use crate::primitive::Primitive;
use crate::restricted::fields::Fields;
use indexmap::IndexMap;
use std::env;
use std::fmt::{Display, Formatter};

const AMQP_CONNECTION_FORCED: &'static str = "amqp:connection:forced";
const AMQP_CONNECTION_FRAMING_ERROR: &'static str = "amqp:connection:framing-error";
const AMQP_CONNECTION_REDIRECT: &'static str = "amqp:connection:redirect";

pub(crate) const TAGS: [&str; 3] = [
    AMQP_CONNECTION_FORCED,
    AMQP_CONNECTION_FRAMING_ERROR,
    AMQP_CONNECTION_REDIRECT,
];
#[derive(Debug)]
pub enum ConnectionError {
    ConnectionForced,
    FramingError,
    Redirect {
        host_name: Option<String>,
        network_host: Option<String>,
        port: Option<u16>,
    },
}

impl Display for ConnectionError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ConnectionError::ConnectionForced => write!(f, "{}", AMQP_CONNECTION_FORCED),
            ConnectionError::FramingError => write!(f, "{}", AMQP_CONNECTION_FRAMING_ERROR),
            ConnectionError::Redirect { .. } => write!(f, "{}", AMQP_CONNECTION_REDIRECT),
        }
    }
}

const HOST_NAME: &'static str = "hostname";
const NETWORK_HOST: &'static str = "network-host";
const PORT: &'static str = "port";

impl ErrorCondition for ConnectionError {
    fn error_condition(&self) -> Symbol {
        self.to_string()
            .try_into()
            .expect("ConnectionError to Symbol conversion must never fail.")
    }

    fn amqp_description(&self) -> Option<String> {
        let desc = match self {
            ConnectionError::ConnectionForced => "An operator intervened to close the Connection for some reason. The client may retry at some later date.",
            ConnectionError::FramingError => "A valid frame header cannot be formed from the incoming byte stream.",
            ConnectionError::Redirect { .. } => "The container is no longer available on the current connection. The peer should attempt reconnection to the container using the details provided in the info map."
        }.to_string();
        Some(desc)
    }

    ///
    /// The Library uses the following Environment Variables in order to determine the connection::redirect
    /// properties:
    /// - AMQP_CONNECTION_REDIRECT_HOST_NAME
    /// - AMQP_CONNECTION_REDIRECT_NETWORK_HOST
    /// - AMQP_CONNECTION_REDIRECT_PORT
    fn info(&self) -> Option<Fields> {
        match self {
            ConnectionError::ConnectionForced => None,
            ConnectionError::FramingError => None,
            ConnectionError::Redirect { .. } => {
                let host_name = env::var("AMQP_CONNECTION_REDIRECT_HOST_NAME").ok();
                let network_host = env::var("AMQP_CONNECTION_REDIRECT_NETWORK_HOST").ok();
                let port = env::var("AMQP_CONNECTION_REDIRECT_PORT")
                    .ok()
                    .map(|port| port.parse::<u16>().ok())
                    .expect("Port must be parsable into a u16.");
                let mut map = IndexMap::with_capacity(3);
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
                Some(Fields::new(map))
            }
        }
    }
}

impl std::error::Error for ConnectionError {}

impl TryFrom<(Option<Primitive>, Option<Primitive>, Option<Primitive>)> for ConnectionError {
    type Error = AppError;
    fn try_from(
        (condition, _, info): (Option<Primitive>, Option<Primitive>, Option<Primitive>),
    ) -> Result<Self, Self::Error> {
        if let Some(Primitive::Symbol(s)) = condition {
            match s.inner() {
                AMQP_CONNECTION_FORCED => Err(ConnectionError::ConnectionForced)?,
                AMQP_CONNECTION_FRAMING_ERROR => Err(ConnectionError::FramingError)?,
                AMQP_CONNECTION_REDIRECT => {
                    if let Some(Primitive::Map(mut info)) = info {
                        let port = info
                            .remove(Symbol::with_ascii(PORT))
                            .map(|v| v.try_into().ok())
                            .flatten();
                        let network_host = info
                            .remove(Symbol::with_ascii(NETWORK_HOST))
                            .map(|v| v.try_into().ok())
                            .flatten();
                        let host_name = info
                            .remove(Symbol::with_ascii(HOST_NAME))
                            .map(|v| v.try_into().ok())
                            .flatten();
                        Err(ConnectionError::Redirect {
                            host_name,
                            network_host,
                            port,
                        })?
                    } else {
                        Err(ConnectionError::Redirect {
                            host_name: None,
                            network_host: None,
                            port: None,
                        })?
                    }
                }
                _ => Err(AppError::SpecificationNonCompliantError),
            }
        } else {
            Err(AppError::SpecificationNonCompliantError)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_try_from_connection_error_connection_forced() {
        let error = (
            Some(Primitive::Symbol(
                Symbol::new(AMQP_CONNECTION_FORCED.into()).unwrap(),
            )),
            None,
            None,
        );
        assert!(matches!(
            ConnectionError::try_from(error),
            Err(AppError::Connection(ConnectionError::ConnectionForced))
        ));
    }

    #[test]
    fn test_try_from_connection_error_framing_error() {
        let error = (
            Some(Primitive::Symbol(
                Symbol::new(AMQP_CONNECTION_FRAMING_ERROR.into()).unwrap(),
            )),
            None,
            None,
        );
        assert!(matches!(
            ConnectionError::try_from(error),
            Err(AppError::Connection(ConnectionError::FramingError))
        ));
    }

    #[test]
    fn test_try_from_connection_error_redirect() {
        let error = (
            Some(Primitive::Symbol(
                Symbol::new(AMQP_CONNECTION_REDIRECT.into()).unwrap(),
            )),
            None,
            None,
        );
        assert!(matches!(
            ConnectionError::try_from(error),
            Err(AppError::Connection(ConnectionError::Redirect { .. }))
        ));
    }

    #[test]
    fn test_try_from_connection_error_redirect_values() {
        let redirect_args = {
            let mut args = IndexMap::new();
            args.insert(Symbol::with_ascii(HOST_NAME), "localhost".into());
            args.insert(Symbol::with_ascii(NETWORK_HOST), "127.0.0.1".into());
            args.insert(Symbol::with_ascii(PORT), 9876_u16.into());
            Some(Fields::new(args).into())
        };
        let error = (
            Some(Primitive::Symbol(
                Symbol::new(AMQP_CONNECTION_REDIRECT.into()).unwrap(),
            )),
            None,
            redirect_args,
        );
        match ConnectionError::try_from(error) {
            Err(AppError::Connection(ConnectionError::Redirect {
                host_name,
                network_host,
                port,
            })) => {
                assert_eq!(network_host, Some("127.0.0.1".into()));
                assert_eq!(host_name, Some("localhost".into()));
                assert_eq!(port, Some(9876));
            }
            result => panic!("Expected ConnectionError::Redirect. But was {:?}", result),
        }
    }

    #[test]
    fn test_connection_error_info() {
        env::set_var("AMQP_CONNECTION_REDIRECT_HOST_NAME", "localhost");
        env::set_var("AMQP_CONNECTION_REDIRECT_NETWORK_HOST", "127.0.0.1");
        env::set_var("AMQP_CONNECTION_REDIRECT_PORT", "9876");
        let expected = Fields::new(
            [
                (
                    HOST_NAME.to_string().try_into().unwrap(),
                    Primitive::from(Some("localhost".to_string())),
                ),
                (
                    NETWORK_HOST.to_string().try_into().unwrap(),
                    Some("127.0.0.1".to_string()).into(),
                ),
                (PORT.to_string().try_into().unwrap(), Some(9876_u16).into()),
            ]
            .into(),
        );

        assert_eq!(ConnectionError::ConnectionForced.info(), None);
        assert_eq!(ConnectionError::FramingError.info(), None);
        assert_eq!(
            ConnectionError::Redirect {
                host_name: None,
                network_host: None,
                port: None
            }
            .info()
            .unwrap(),
            expected
        );
    }
}
