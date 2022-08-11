use eframe::egui;
use eframe::egui::Frame;

use crate::chat_ui::ChatUi;
use crate::connection::Connection;
use crate::egui::{Button, CentralPanel, Grid, Key, TextEdit, Window};

mod connection;
mod models;
mod chat_ui;

#[tokio::main]
async fn main() {
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "Chat App",
        options,
        Box::new(|cc| Box::new(MyApp::new(cc))),
    );
}

#[derive(serde::Deserialize, serde::Serialize)]
struct AppValues {
    server: String,
    room: String,
    user: String,
}

#[derive(Default)]
enum AppState {
    #[default] Home,
    Chat(ChatUi),
}

#[derive(serde::Deserialize, serde::Serialize)]
struct MyApp {
    values: AppValues,

    #[serde(skip)]
    state: AppState,
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            values: AppValues {
                room: "test".to_string(),
                server: "ws://localhost:6789".to_string(),
                user: "user".to_string(),
            },
            state: AppState::Home,
        }
    }
}

impl MyApp {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        if let Some(storage) = cc.storage {
            return  eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
        }
        Default::default()
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        let mut save_state = false;
        match &mut self.state {
            AppState::Home => {
                let AppValues {
                    server,
                    room,
                    user,
                } = &mut self.values;

                Window::new("Connect")
                    .default_pos((200.0,200.0))
                    .show(ctx, |ui| {
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

                                ui.label("User Name");
                                ui.text_edit_singleline(user);
                                ui.end_row();
                            }

                            ui.add_enabled_ui(user.len() > 0, |ui| {
                                if ui.button("Connect").clicked() || ui.input().key_pressed(Key::Enter) {
                                    save_state = true;

                                    self.state = AppState::Chat(ChatUi::new(Connection::new(
                                        server.to_string(),
                                        room.to_string(),
                                        user.to_string(),
                                    )));
                                };
                            });
                        });
                });
                CentralPanel::default().show(ctx, |ui|{});
            }
            AppState::Chat(chat_ui) => {
                chat_ui.ui(ctx);
            }
        };

        // currently app.save is not called when quitting on macos so we do it here manually
        if save_state {
            if let Some(storage) = frame.storage_mut() {
                eframe::set_value(storage, eframe::APP_KEY, self);
                storage.flush();
            }
        }
    }

    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }
}