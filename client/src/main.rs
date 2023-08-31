use anyhow::Result;
use tokio::io::AsyncWriteExt;
use tokio::net::TcpStream;

#[tokio::main]
async fn main() -> Result<()> {
    let mut stream = TcpStream::connect("127.0.0.1:8080").await?;

    stream.write_all(b"hello world").await?;

    Ok(())
}
