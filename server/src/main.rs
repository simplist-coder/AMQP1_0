use anyhow::Result;
use tokio::{io::AsyncReadExt, net::TcpListener};

#[tokio::main]
async fn main() -> Result<()> {
    let stream = TcpListener::bind("127.0.0.1:8080").await?;

    loop {}
    Ok(())
}
