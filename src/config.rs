use serde::{Deserialize, Serialize};
use std::fs;

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    pub gallery: Option<GalleryConfig>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GalleryConfig {
    pub title: Option<String>,
    pub description: Option<String>,
    pub author: Option<String>,
}

pub fn get_config(file: &str) -> Config {
    let config_str = fs::read_to_string(file).unwrap();
    toml::from_str(&config_str).unwrap()
}
