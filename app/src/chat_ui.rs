use eframe::egui::TextStyle;

use models::{ClientMessage, ServerMessage};

use crate::connection::{Connection, ConnectionEvent};
use crate::egui;
use crate::egui::{Key, RichText, ScrollArea};

enum Status {
    Connecting,
    Connected,
    Error(String),
    Closed,
}

pub struct ChatUi {
    input_message: String,

    connection: Connection,
    messages: Vec<ServerMessage>,

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
            let scroll_height = ui.available_height() - 27.0;
            let text_height = ui.text_style_height(&TextStyle::Body);
            ui.vertical(|ui| {
                ScrollArea::vertical()
                    .stick_to_bottom()
                    .auto_shrink([false, false])
                    .max_height(scroll_height)
                    .show_rows(
                        ui,
                        text_height,
                        self.messages.len(),
                        |ui, row_range| {
                            for row in row_range {
                                match &self.messages[row] {
                                    ServerMessage::Message { user, text } => {
                                        ui.horizontal(|ui| {
                                            ui.label(format!("{}:", user));
                                            ui.label(text);
                                        });
                                    }
                                    ServerMessage::Joined { user } => {
                                        ui.label(RichText::new(&format!("{user} joined the room")).italics());
                                    }
                                    ServerMessage::Left { user } => {
                                        ui.label(RichText::new(&format!("{user} left the room")).italics());
                                    }
                                }
                            }
                        },
                    );
                ui.horizontal(|ui| {
                    let input = ui.text_edit_singleline(&mut self.input_message);
                    ui.add_enabled_ui(self.input_message.len() > 0, |ui| {
                        if ui.button("Send").clicked() || input.lost_focus() && ui.input().key_pressed(Key::Enter) && self.input_message.trim() != "" {
                            input.request_focus();
                            self.connection.send(ClientMessage::SendMessage {
                                text: std::mem::take(&mut self.input_message),
                            });
                        }
                    });
                });
            });
        });
    }
}