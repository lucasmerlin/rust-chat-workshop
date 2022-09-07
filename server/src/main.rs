use std::io::Error;

use futures_util::TryStreamExt;
use tokio::net::{TcpListener, TcpStream};

use models::ClientMessage;

#[tokio::main]
async fn main() -> Result<(), Error> {
    let addr = "0.0.0.0:6789".to_string();

    let listener = TcpListener::bind(addr).await?;

    while let Ok((stream, _)) = listener.accept().await {
        tokio::spawn(accept_connection(stream));
    }

    Ok(())
}

async fn accept_connection(stream: TcpStream) -> Result<(), tungstenite::Error> {
    let mut ws_stream = tokio_tungstenite::accept_async(stream).await?;

    if let Some(tungstenite::Message::Text(text)) = ws_stream.try_next().await? {
        if let Ok(ClientMessage::Connect { room, user }) = serde_json::from_str::<ClientMessage>(&text) {
            println!("{user} is trying to join {room}");
        }
    }

    Ok(())
}