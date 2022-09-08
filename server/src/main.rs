mod room;
mod connection;

use std::io::Error;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use futures_util::TryStreamExt;
use tokio::net::{TcpListener, TcpStream};

use models::ClientMessage;
use crate::room::RoomHandle;

type Rooms = Arc<Mutex<HashMap<String, RoomHandle>>>;

#[tokio::main]
async fn main() -> Result<(), Error> {
    let addr = "0.0.0.0:6789".to_string();

    let listener = TcpListener::bind(addr).await?;

    let rooms = Arc::new(Mutex::new(HashMap::new()));

    while let Ok((stream, _)) = listener.accept().await {
        tokio::spawn(accept_connection(stream, rooms.clone()));
    }

    Ok(())
}

async fn accept_connection(stream: TcpStream, rooms: Rooms) -> Result<(), tungstenite::Error> {
    let mut ws_stream = tokio_tungstenite::accept_async(stream).await?;

    if let Some(tungstenite::Message::Text(text)) = ws_stream.try_next().await? {
        if let Ok(ClientMessage::Connect { room, user }) = serde_json::from_str::<ClientMessage>(&text) {
            let handle = {
                let mut rooms = rooms.lock().unwrap();
                rooms
                    .entry(room)
                    .or_insert_with(|| RoomHandle::new()).clone()
            };

            handle.join(user, ws_stream).await;
        }
    }

    Ok(())
}