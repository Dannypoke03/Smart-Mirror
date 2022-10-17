#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use crate::commands::{get_config, update_config};
use api::{api_config, api_update_config, spotify_callback, spotify_check, spotify_login};
use config_handler::load_config;
use rocket::fs::FileServer;
mod api;
mod commands;
mod config_handler;

fn main() {
    load_config().unwrap();

    tauri::Builder::default()
        .setup(|_app| {
            tauri::async_runtime::spawn(
                rocket::build()
                    .mount(
                        "/api",
                        rocket::routes![
                            api_config,
                            api_update_config,
                            spotify_login,
                            spotify_callback
                        ],
                    )
                    .mount("/", FileServer::from("../dist"))
                    .launch(),
            );
            tauri::async_runtime::spawn(async {
                spotify_check().await;
            });
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![get_config, update_config])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
