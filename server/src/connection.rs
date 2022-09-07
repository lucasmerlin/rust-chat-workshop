use tokio::sync::mpsc;

#[derive(Debug)]
pub struct Connection {
    pub user: String,
    pub sender: mpsc::Sender<tungstenite::Message>,
}