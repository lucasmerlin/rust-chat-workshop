use eframe::egui;
use ewebsock::{connect, connect_with_wakeup, WsEvent, WsMessage, WsReceiver, WsSender};
use ewebsock::WsMessage::Text;
use models::{Connect, Message, SendMessage};

pub struct Connection {
    sender: WsSender,
    receiver: WsReceiver,
    connect_message: Connect,
}

pub enum  ConnectionEvent {
    Opened,
    Message(Message),
    Error(String),
    Closed,
}

impl Connection {

    pub fn new(server: String, room: String, user: String, ctx: egui::Context) -> Connection {

        let (mut sender, receiver) = connect_with_wakeup(&server, move || ctx.request_repaint()).expect("Failed to create WebSocket");

        Connection {
            sender,
            receiver,
            connect_message: Connect {
                room,
                user,
            }
        }
    }

    pub fn try_recv(&mut self) -> Option<ConnectionEvent> {
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
                let connect_message = serde_json::to_string(&self.connect_message).expect("Failed to stringify!");
                self.sender.send(Text(connect_message));

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

    pub fn send(&mut self, message: SendMessage) {
        let encoded = serde_json::to_string(&message).unwrap();
        self.sender.send(Text(encoded));
    }

}