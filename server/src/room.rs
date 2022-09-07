use std::collections::HashMap;
use tokio::sync::{mpsc, oneshot};
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