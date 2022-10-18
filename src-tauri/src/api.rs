use std::{collections::HashMap, env};
use tokio::time::Duration;

use crate::config_handler::{
    load_app_config, load_config, load_private_config, save_config, save_private_config, MyConfig,
};
use rocket::{response::Redirect, serde::json::Json, uri};
use serde::Deserialize;

#[rocket::get("/config")]
pub fn api_config() -> Json<MyConfig> {
    Json(load_config().unwrap())
}

#[rocket::post("/config", data = "<updated_cfg>")]
pub fn api_update_config(updated_cfg: Json<MyConfig>) -> Json<MyConfig> {
    Json(save_config(updated_cfg.into_inner()).unwrap())
}

#[rocket::get("/spotify-login")]
pub fn spotify_login() -> Redirect {
    // let main = .to_owned();
    // let callback_url = uri!(spotify_callback).to_string().to_owned();
    // let url = main + &callback_url;
    // let test = &url as &str;
    Redirect::to(uri!("https://accounts.spotify.com/authorize?response_type=code&client_id=155229d5d7744f1eb57f6ca7e931debb&scope=user-read-currently-playing%20user-follow-read&redirect_uri=http://localhost:8000/api/spotify-callback"))
}

#[derive(Deserialize, Debug, Clone)]
struct SpotifyResponse {
    access_token: String,
    // token_type: String,
    expires_in: i32,
    refresh_token: Option<String>,
    // scope: String,
}

static mut SPOTIFY_TASK: Option<tokio::task::JoinHandle<()>> = None;

#[rocket::get("/spotify-callback?<code>&<error>")]
pub async fn spotify_callback(code: Option<&str>, error: Option<&str>) -> Redirect {
    let app_config = load_app_config().unwrap();
    if let None = error {
        let mut params = HashMap::new();
        params.insert("code", code.unwrap());
        params.insert("grant_type", "authorization_code");
        let base = env::var("SPOTIFY_BASE_URL")
            .unwrap_or_else(|_| "http://localhost/api/spotify-callback".into());
        println!("Callback: {}", base);
        params.insert("redirect_uri", &base);

        let client = reqwest::Client::new();
        let resp = client
            .post("https://accounts.spotify.com/api/token")
            .basic_auth(
                app_config.spotify_id.as_str(),
                Some(app_config.spotify_secret.as_str()),
            )
            .form(&params)
            .send()
            .await;

        match resp {
            Ok(resp) => {
                let mut cfg = load_config().unwrap();
                let spotify_data = resp.json::<SpotifyResponse>().await.unwrap();
                cfg.spotify_key = spotify_data.access_token.clone();

                match spotify_data.refresh_token.clone() {
                    Some(refresh_token) => {
                        let mut private_cfg = load_private_config().unwrap();
                        private_cfg.spotify_refresh = refresh_token.clone();
                        save_private_config(private_cfg).unwrap();
                        spotify_thread_kill();
                        unsafe {
                            SPOTIFY_TASK = Some(tokio::spawn(async move {
                                tokio::time::sleep(Duration::from_secs_f32(
                                    spotify_data.expires_in.clone() as f32,
                                ))
                                .await;
                                spotify_refresh(refresh_token).await;
                            }));
                        }
                    }
                    None => {}
                }
                save_config(cfg).unwrap();
            }
            Err(e) => {
                println!("Error: {:?}", e);
            }
        }

        return Redirect::to(uri!("/"));
    } else {
        println!("Error: {:?}", error);
        return Redirect::to(uri!("/"));
    }
}

async fn spotify_refresh(token: String) {
    let app_config = load_app_config().unwrap();

    let mut params = HashMap::new();
    params.insert("grant_type", "refresh_token");
    params.insert("refresh_token", token.as_str());
    let client = reqwest::Client::new();
    let resp = client
        .post("https://accounts.spotify.com/api/token")
        .basic_auth(
            app_config.spotify_id.as_str(),
            Some(app_config.spotify_secret.as_str()),
        )
        .form(&params)
        .send()
        .await;

    match resp {
        Ok(resp) => {
            let mut cfg = load_config().unwrap();
            let spotify_data = resp.json::<SpotifyResponse>().await.unwrap();
            cfg.spotify_key = spotify_data.access_token.clone();
            save_config(cfg).unwrap();

            match spotify_data.refresh_token.clone() {
                Some(refresh_token) => {
                    let mut private_cfg = load_private_config().unwrap();
                    private_cfg.spotify_refresh = refresh_token.clone();
                    save_private_config(private_cfg).unwrap();
                    tokio::time::sleep(Duration::from_secs_f32(
                        spotify_data.expires_in.clone() as f32
                    ))
                    .await;
                    let _ = spotify_refresh(refresh_token);
                }
                None => {}
            }
        }
        Err(e) => {
            println!("Error: {:?}", e);
        }
    }
}

pub async fn spotify_check() {
    let private_cfg = load_private_config().unwrap();
    if private_cfg.spotify_refresh != "" {
        spotify_thread_kill();
        unsafe {
            SPOTIFY_TASK = Some(tokio::spawn(async move {
                spotify_refresh(private_cfg.spotify_refresh).await;
            }));
        }
    }
}

fn spotify_thread_kill() {
    unsafe {
        match &SPOTIFY_TASK {
            Some(task) => {
                task.abort();
            }
            None => {}
        }
    }
}
