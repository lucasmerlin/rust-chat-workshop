use std::collections::HashMap;
use std::convert::Infallible;
use std::net::SocketAddr;
use std::str::FromStr;
use std::sync::{Arc, Mutex};

use futures_util::TryStreamExt;
use hyper::{Body, Request, Response, Server};
use hyper_tungstenite::HyperWebsocket;

use models::ClientMessage;

use crate::room::RoomHandle;

mod room;
mod connection;

type Rooms = Arc<Mutex<HashMap<String, RoomHandle>>>;

type Error = Box<dyn std::error::Error + Send + Sync + 'static>;

#[tokio::main]
async fn main() -> Result<(), Error> {
    let addr = "0.0.0.0:6789".to_string();

    let rooms = Arc::new(Mutex::new(HashMap::new()));

    Server::bind(&SocketAddr::from_str(&addr).unwrap())
        .serve(hyper::service::make_service_fn(move |_connection| {
            let rooms = rooms.clone();
            async move {
                Ok::<_, Infallible>(hyper::service::service_fn(move |request| handle_request(request, rooms.clone())))
            }
        })).await?;

    Ok(())
}

async fn handle_request(request: Request<Body>, rooms: Rooms) -> Result<Response<Body>, Error> {
    if hyper_tungstenite::is_upgrade_request(&request) {
        let (response, websocket) = hyper_tungstenite::upgrade(request, None)?;
        tokio::spawn(async move {
            if let Err(e) = accept_connection(websocket, rooms).await {
                eprintln!("Error in websocket connection: {}", e);
            }
        });
        Ok(response)
    } else {
        Ok(Response::new(Body::from("Hello HTTP!")))
    }
}

async fn accept_connection(websocket: HyperWebsocket, rooms: Rooms) -> Result<(), tungstenite::Error> {
    let mut ws_stream = websocket.await?;

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