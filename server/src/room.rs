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

#[derive(Clone)]
pub struct RoomHandle {
    tx: mpsc::Sender<RoomMessage>,
}