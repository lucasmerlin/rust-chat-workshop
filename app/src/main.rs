use eframe::egui;

use crate::chat_ui::ChatUi;
use crate::connection::Connection;
use crate::egui::{CentralPanel, Grid, Window};

mod connection;
mod chat_ui;

#[cfg(not(target_arch = "wasm32"))]
#[tokio::main]
async fn main() {
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "Chat App",
        options,
        Box::new(|cc| Box::new(MyApp::new(cc))),
    );
}

#[cfg(target_arch = "wasm32")]
fn main() {
    use eframe::wasm_bindgen::{self, prelude::*};

    eframe::start_web("canvas", Box::new(|cc| Box::new(MyApp::new(cc))));
}

#[derive(serde::Deserialize, serde::Serialize)]
struct AppValues {
    server: String,
    room: String,
    user: String,
    selected: SelectedServer,
}

#[derive(serde::Deserialize, serde::Serialize, Eq, PartialEq)]
enum SelectedServer {
    Localhost,
    MerlinsMedia,
    Custom,
}

#[derive(serde::Deserialize, serde::Serialize)]
struct MyApp {
    values: AppValues,

    #[serde(skip)]
    chats: Vec<ChatUi>,
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            values: AppValues {
                room: "test".to_string(),
                server: "ws://localhost:6789".to_string(),
                user: "".to_string(),
                selected: SelectedServer::Localhost,
            },
            chats: vec![],
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

static MERLINS_MEDIA: &str = "wss://chat-workshop.merlins.media";
static LOCALHOST: &str = "ws://localhost:6789";

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        let mut save_state = false;
        let AppValues {
            server,
            room,
            user,
            selected,
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

                        ui.end_row();
                        ui.radio_value(selected, SelectedServer::MerlinsMedia, MERLINS_MEDIA);
                        ui.end_row();
                        ui.radio_value(selected, SelectedServer::Localhost, LOCALHOST);
                        ui.end_row();
                        ui.radio_value(selected, SelectedServer::Custom, "Custom:");
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
                        if ui.button("Connect").clicked() {
                            save_state = true;

                            let server = match selected {
                                SelectedServer::Localhost => LOCALHOST.to_string(),
                                SelectedServer::MerlinsMedia => MERLINS_MEDIA.to_string(),
                                SelectedServer::Custom => server.clone(),
                            };

                            self.chats.push(ChatUi::new(Connection::new(
                                server,
                                room.to_string(),
                                user.to_string(),
                                ctx.clone(),
                            )));
                        };
                    });
                });
        });
        CentralPanel::default().show(ctx, |_ui|{});

        for chat in &mut self.chats {
            chat.ui(ctx);
        }

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