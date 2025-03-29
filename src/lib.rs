use std::{
    io::{Read, Write},
    path::Path,
    sync::{Arc, LazyLock},
};

use pumpkin::plugin::{Context, EventPriority};
use pumpkin_api_macros::{plugin_impl, plugin_method};
use serde::{Deserialize, Serialize};
use tokio::sync::Mutex;

mod events;

static CONFIG_DIR: LazyLock<Arc<Mutex<String>>> =
    LazyLock::new(|| Arc::new(Mutex::new("config".to_string())));
pub static CONFIG: LazyLock<Arc<Mutex<CommandLimiter>>> =
    LazyLock::new(|| Arc::new(Mutex::new(CommandLimiter::default())));

#[plugin_method]
async fn on_load(&mut self, server: &Context) -> Result<(), String> {
    pumpkin::init_log!();
    *CONFIG_DIR.lock().await = server.get_data_folder();

    let data_dir = server.get_data_folder();
    let config_file = Path::new(&data_dir).join("config.json");

    if !config_file.exists() {
        let mut file = std::fs::File::create(&config_file).map_err(|e| e.to_string())?;
        let config = serde_json::to_string(&self).map_err(|e| e.to_string())?;
        file.write(config.as_bytes()).map_err(|e| e.to_string())?;
    } else {
        let mut file = std::fs::File::open(&config_file).map_err(|e| e.to_string())?;
        let mut config = String::new();
        file.read_to_string(&mut config)
            .map_err(|e| e.to_string())?;
        *self = serde_json::from_str(&config).map_err(|e| e.to_string())?;
    }

    log::info!("CommandLimiter config loaded!");

    server
        .register_event(
            Arc::new(events::CommandSendHandler),
            EventPriority::Highest,
            true,
        )
        .await;

    log::info!("CommandLimiter event handler registered!");

    *CONFIG.lock().await = self.clone();

    Ok(())
}

pub async fn save_config() -> Result<(), String> {
    let config = CONFIG.lock().await.clone();
    let data_dir = CONFIG_DIR.lock().await.clone();
    let config_file = Path::new(&data_dir).join("config.json");

    let mut file = std::fs::File::create(&config_file).map_err(|e| e.to_string())?;
    let config = serde_json::to_string(&config).map_err(|e| e.to_string())?;
    file.write(config.as_bytes()).map_err(|e| e.to_string())?;

    Ok(())
}

#[plugin_impl]
#[derive(Serialize, Deserialize, Clone)]
pub struct CommandLimiter {
    pub commands: Vec<CommandInfo>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct CommandInfo {
    pub name: String,
    pub blacklist: bool,
    pub allowed: Vec<String>,
}

impl CommandLimiter {
    pub fn new() -> Self {
        Self { commands: vec![] }
    }
}

impl Default for CommandLimiter {
    fn default() -> Self {
        Self::new()
    }
}
