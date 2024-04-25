use std::fs;

use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Config {
    pub app: ApplicationConfig,
    pub search: SearchConfig,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct SearchConfig {
    pub address: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ApplicationConfig {
    pub port: i32,
    pub page_size: u32,
}

pub fn parse_config(file_name: &str) -> Config {
    let content = fs::read_to_string(file_name).expect("Failed to open TOML config file");
    toml::from_str(&content).expect("Failed to parse TOML config file")
}
