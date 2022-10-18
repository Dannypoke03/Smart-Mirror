// #![cfg_attr(
//     all(not(debug_assertions), target_os = "windows"),
//     windows_subsystem = "windows"
// )]

use std::thread;

// use crate::commands::{get_config, update_config};
use api::{api_config, api_update_config, spotify_callback, spotify_check, spotify_login};
use rocket::fs::FileServer;
mod api;
// mod commands;
mod config_handler;

#[macro_use]
extern crate rocket;

#[launch]
fn rocket() -> _ {
    thread::spawn(|| async {
        spotify_check().await;
    });
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
}
