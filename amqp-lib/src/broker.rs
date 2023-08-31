use crate::{connection::Connection, error::AppError};

pub struct Broker {
    url: String,
}

impl Broker {
    pub(crate) fn new(url: &str) -> Self {
        Broker {
            url: url.to_string(),
        }
    }

    pub(crate) async fn connect(&self) -> Result<Connection, AppError> {
        Ok(Connection {})
    }
}
