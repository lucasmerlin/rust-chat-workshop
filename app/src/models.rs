use serde::Serialize;
use serde::Deserialize;

#[derive(Serialize, Deserialize, Debug)]
pub struct Message {
    pub text: String,
    pub user: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SendMessage {
    pub text: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Connect {
    pub room: String,
    pub user: String,
}