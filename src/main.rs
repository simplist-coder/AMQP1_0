use anyhow::Result;
use std::net::SocketAddr;
use tokio::net::{TcpListener, TcpStream};

mod broker;
mod client;
mod connection;
mod error;
mod frame;
mod link;
mod performative;
mod session;
mod terminus;
mod types;

#[tokio::main]
async fn main() -> Result<()> {
    let addr = "127.0.0.1:8080".parse::<SocketAddr>()?;
    let stream = TcpListener::bind(&addr).await?;

    loop {
        let (socket, _) = stream.accept().await?;
    }
    Ok(())
}
