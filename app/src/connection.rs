use ewebsock::{connect, WsEvent, WsMessage, WsReceiver, WsSender};
use ewebsock::WsMessage::Text;
use crate::models::Message;

pub struct Connection {
    sender: WsSender,
    receiver: WsReceiver,
}

pub enum  ConnectionEvent {
    Opened,
    Message(Message),
    Error(String),
    Closed,
}

impl Connection {

    pub fn new() -> Connection {

        let (sender, receiver) = connect("ws://localhost:6789").unwrap();

        Connection {
            sender,
            receiver,
        }
    }

    pub fn try_recv(&self) -> Option<ConnectionEvent> {
        let value = self.receiver.try_recv()?;

        Some(match value {
            WsEvent::Message(message) => {
                if let Text(text) = message {
                    serde_json::from_str(&text)
                        .map(|message| ConnectionEvent::Message(message))
                        .unwrap_or_else(|error| ConnectionEvent::Error(error.to_string()))
                } else {
                    ConnectionEvent::Error("Invalid Message Type".to_string())
                }
            }
            WsEvent::Opened => {
                ConnectionEvent::Opened
            }
            WsEvent::Error(error) => {
                ConnectionEvent::Error(error)
            }
            WsEvent::Closed => {
                ConnectionEvent::Closed
            }
        })
    }

    pub fn send(&mut self, message: Message) {
        let encoded = serde_json::to_string(&message).unwrap();
        self.sender.send(Text(encoded));
    }

}