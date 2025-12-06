use anyhow::{Context, Result};
use serde::Deserialize;
use std::collections::HashMap;
use std::fs;
use std::path::Path;

#[derive(Debug, Deserialize, Clone, Default)]
#[allow(dead_code)]
pub struct Config {
    pub default_preset: Option<String>,
    #[serde(flatten)]
    pub presets: HashMap<String, Preset>,
}

#[derive(Debug, Deserialize, Clone)]
#[allow(dead_code)]
pub struct Preset {
    pub width: Option<u32>,
    pub fps: Option<u32>,
    pub max_size_mb: Option<f64>,
}

pub fn load_config() -> Result<Config> {
    // Look for .readme-gif.toml in current directory
    let config_path = Path::new(".readme-gif.toml");
    if config_path.exists() {
        let content = fs::read_to_string(config_path).context("Failed to read config file")?;
        let config: Config = toml::from_str(&content).context("Failed to parse config file")?;
        Ok(config)
    } else {
        Ok(Config::default())
    }
}
