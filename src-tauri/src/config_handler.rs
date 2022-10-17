use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
#[serde(crate = "rocket::serde")]
pub struct MyConfig {
    pub spotify_key: String,
    pub weather_location: String,
    pub weather_enabled: bool,
    pub time_enabled: bool,
    pub spotify_playing_enabled: bool,
    pub spotify_releases_enabled: bool,
    pub quotes_enabled: bool,
    pub quotes_interval: u64,
    pub quote_tags: Vec<String>,
}

impl ::std::default::Default for MyConfig {
    fn default() -> Self {
        Self {
            spotify_key: "".into(),
            weather_location: "".into(),
            weather_enabled: true,
            time_enabled: true,
            spotify_playing_enabled: true,
            spotify_releases_enabled: true,
            quotes_enabled: true,
            quotes_interval: 60,
            quote_tags: Vec::new(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(crate = "rocket::serde")]
pub struct PrivateConfig {
    pub spotify_refresh: String,
}

impl ::std::default::Default for PrivateConfig {
    fn default() -> Self {
        Self {
            spotify_refresh: "".into(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(crate = "rocket::serde")]
pub struct AppConfig {
    pub spotify_id: String,
    pub spotify_secret: String,
}

impl ::std::default::Default for AppConfig {
    fn default() -> Self {
        Self {
            spotify_id: "".into(),
            spotify_secret: "".into(),
        }
    }
}

pub fn load_config() -> Result<MyConfig, confy::ConfyError> {
    let cfg = confy::load::<MyConfig>("SmartMirror", None)?;
    Ok(cfg)
}

pub fn save_config(cfg: MyConfig) -> Result<MyConfig, confy::ConfyError> {
    confy::store("SmartMirror", None, &cfg)?;
    Ok(cfg)
}

pub fn load_private_config() -> Result<PrivateConfig, confy::ConfyError> {
    let cfg = confy::load::<PrivateConfig>("SmartMirror", Some("private"))?;
    Ok(cfg)
}

pub fn save_private_config(cfg: PrivateConfig) -> Result<PrivateConfig, confy::ConfyError> {
    confy::store("SmartMirror", Some("private"), &cfg)?;
    Ok(cfg)
}

pub fn load_app_config() -> Result<AppConfig, confy::ConfyError> {
    let cfg = confy::load_path::<AppConfig>("./config.toml")?;
    Ok(cfg)
}
