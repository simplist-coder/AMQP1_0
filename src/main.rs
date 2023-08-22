use anyhow::Result;
use std::net::SocketAddr;
use tokio::net::{TcpListener, TcpStream};

mod connection;
mod error;
mod frame;
mod link;
mod performative;
mod session;
mod types;
mod terminus;
mod client;
mod broker;

#[tokio::main]
async fn main() -> Result<()> {
    let addr = "127.0.0.1:8080".parse::<SocketAddr>()?;
    let stream = TcpListener::bind(&addr).await?;

    loop {
        let (socket, _) = stream.accept().await?;
    }
    Ok(())
}
