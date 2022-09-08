use std::collections::HashMap;
use tokio::net::TcpStream;
use tokio::sync::{mpsc, oneshot};
use tokio_tungstenite::WebSocketStream;
use models::ClientMessage;
use crate::connection::Connection;

#[derive(Debug)]
enum RoomMessage {
    Join(Connection, oneshot::Sender<u64>),
    ClientMessage(ClientMessage, String),
    Leave(u64),
}

struct RoomActor {
    rx: mpsc::Receiver<RoomMessage>,
    next_connection_id: u64,
    users: HashMap<u64, Connection>,
}

impl RoomActor {
    pub async fn run(&mut self) {
        while let Some(msg) = self.rx.recv().await {
            match msg {
                RoomMessage::Join(connection, send_uid) => {
                    println!("User {} joined", connection.user);

                    self.users.insert(self.next_connection_id, connection);
                    send_uid.send(self.next_connection_id).unwrap();
                    self.next_connection_id += 1;
                }
                _ => {}
            }
        }
    }
}

#[derive(Clone)]
pub struct RoomHandle {
    tx: mpsc::Sender<RoomMessage>,
}

impl RoomHandle {
    pub fn new() -> Self {
        let (tx, rx) = mpsc::channel(100);

        let mut actor = RoomActor {
            rx,
            users: HashMap::new(),
            next_connection_id: 0,
        };

        tokio::spawn(async move {
            actor.run().await
        });

        RoomHandle {
            tx,
        }
    }


    pub async fn join(&self, user: String, stream: WebSocketStream<TcpStream>) {
        let (conn_id_tx, conn_id_rx) = oneshot::channel();
        let (sender_tx, mut sender_rx) = mpsc::channel(100);

        self.tx.send(RoomMessage::Join(Connection {
            sender: sender_tx,
            user: user.clone(),
        }, conn_id_tx)).await.unwrap();

        let conn_id = conn_id_rx.await.unwrap();
        println!("{user} successfully joined room with user id {conn_id}")
    }
}