use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(tag = "type")]
pub enum ServerMessage {
    Message {
        text: String,
        user: String,
    },
    Joined {
        user: String,
    },
    Left {
        user: String,
    },
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(tag = "type")]
pub enum ClientMessage {
    SendMessage {
        text: String,
    },
    Connect {
        room: String,
        user: String,
    },
}
