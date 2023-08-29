use crate::client::Client;
use crate::broker::Broker;
use crate::connection::Connection;

pub enum Peer {
	Client(Client),
	Broker(Broker)
}

impl Peer {

	pub fn new_client(url: &str) -> Self {
		let client = Client::new(url);
		Peer::Client(client)
	}

	pub fn new_broker(url: &str) -> Self {
		let broker = Broker::new(url);
		Peer::Broker(broker)
	}


	pub async fn connect(&self) -> Connection {
		match self {
            Self::Client(client) => client.connect(),
			Self::Broker(broker) => broker.connect(),
        }
	}
}
