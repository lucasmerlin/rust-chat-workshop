use eframe::egui;

use crate::chat_ui::ChatUi;
use crate::connection::Connection;
use crate::egui::{CentralPanel, Grid};

mod connection;
mod models;
mod chat_ui;

#[tokio::main]
async fn main() {
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "My egui App",
        options,
        Box::new(|_cc| Box::new(MyApp::default())),
    );
}

enum MyApp {
    Home {
        server: String,
        room: String,
    },
    Chat(ChatUi),
}

impl Default for MyApp {
    fn default() -> Self {
        Self::Home {
            room: "test".to_string(),
            server: "ws://localhost:6789".to_string(),
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let mut new_state: Option<MyApp> = None;
        match self {
            MyApp::Home { server, room } => {
                CentralPanel::default().show(ctx, |ui| {
                    Grid::new("home")
                        .num_columns(2)
                        .show(ui, |ui| {
                            {
                                ui.label("Chat App");
                                ui.end_row();

                                ui.label("Server");
                                ui.text_edit_singleline(server);
                                ui.end_row();

                                ui.label("Room");
                                ui.text_edit_singleline(room);
                                ui.end_row();
                            }

                            if ui.button("Connect").clicked() {
                                new_state = Some(MyApp::Chat(ChatUi::new(Connection::new())));
                            };
                        });
                });
            }
            MyApp::Chat(chat_ui) => {
                chat_ui.ui(ctx);
            }
        };

        if let Some(state) = new_state {
            *self = state;
        };
    }
}