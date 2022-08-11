use std::io::Error;
use futures_util::{StreamExt};
use tokio::net::{TcpListener, TcpStream};

#[tokio::main]
async fn main() -> Result<(), Error> {
    let addr = "0.0.0.0:6789".to_string();

    let listener = TcpListener::bind(addr).await?;

    Ok(())
}

