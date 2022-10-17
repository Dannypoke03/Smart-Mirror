use crate::config_handler::{load_config, save_config, MyConfig};

#[tauri::command]
pub fn get_config() -> Result<MyConfig, String> {
    let cfg = load_config();

    match cfg {
        Ok(conf) => Ok(conf),
        Err(e) => Err(e.to_string()),
    }
}

#[tauri::command]
pub fn update_config(updated_cfg: crate::config_handler::MyConfig) -> Result<String, String> {
    let cfg = save_config(updated_cfg);
    match cfg {
        Ok(cfg) => Ok(format!("{:?}", cfg)),
        Err(e) => Err(e.to_string()),
    }
}
