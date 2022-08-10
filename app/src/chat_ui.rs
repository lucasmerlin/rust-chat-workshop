use eframe::egui::TextStyle;
use crate::connection::{Connection, ConnectionEvent};
use crate::egui;
use crate::egui::ScrollArea;
use crate::models::Message;

enum Status {
    Connecting,
    Connected,
    Error(String),
    Closed,
}

pub struct ChatUi {
    input_message: String,

    connection: Connection,
    messages: Vec<Message>,

    status: Status,
}


impl ChatUi {
    pub fn new(connection: Connection) -> ChatUi {
        ChatUi {
            input_message: "".to_string(),
            connection,
            messages: vec![],

            status: Status::Connecting,
        }
    }

    pub fn ui(&mut self, ctx: &egui::Context) {
        while let Some(event) = self.connection.try_recv() {
            match event {
                ConnectionEvent::Opened => {
                    self.status = Status::Connected
                }
                ConnectionEvent::Message(message) => {
                    self.messages.push(message);
                }
                ConnectionEvent::Error(error) => {
                    self.status = Status::Error(error)
                }
                ConnectionEvent::Closed => {
                    self.status = Status::Closed
                }
            }
        }

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.label("Chat App");

                match &self.status {
                    Status::Connecting => {
                        ui.label("Connecting");
                        ui.spinner();
                    }
                    Status::Connected => {
                        ui.label("Connected");
                    }
                    Status::Error(error) => {
                        ui.label("Error: ");
                        ui.label(error);
                    }
                    Status::Closed => {
                        ui.label("Disconnected");
                    }
                }
            });
            let text_height = ui.text_style_height(&TextStyle::Body);
            ScrollArea::vertical().stick_to_bottom().show_rows(
                ui,
                text_height,
                self.messages.len(),
                |ui, row_range| {
                    for row in row_range {
                        ui.horizontal(|ui| {
                           ui.label(&self.messages[row].text);
                        });
                    }
                }
            );
            ui.horizontal(|ui| {
                ui.text_edit_singleline(&mut self.input_message);
                if ui.button("Send").clicked() {
                    self.connection.send(Message {
                        text: std::mem::take(&mut self.input_message)
                    });
                    self.input_message = "".to_string();
                }
            });
        });
    }
}