use tracing::debug;

use crate::{connection::Connection, error::AppError};

pub struct Client {
    url: String,
}

impl Client {
    pub(crate) fn new(url: &str) -> Self {
        Client {
            url: url.to_string(),
        }
    }

    pub(crate) async fn connect(&self) -> Result<Connection, AppError> {
        debug!("Connecting to '{}'", self.url);
        Ok(Connection {})
    }
}
