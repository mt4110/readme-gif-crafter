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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_existing_preset_format() {
        let config: Config = toml::from_str(
            r#"
default_preset = "github"

[github]
width = 800
fps = 15
max_size_mb = 10.0

[mini]
width = 480
fps = 12
max_size_mb = 5.0
"#,
        )
        .unwrap();

        assert_eq!(config.default_preset.as_deref(), Some("github"));
        assert_eq!(config.presets["github"].width, Some(800));
        assert_eq!(config.presets["github"].fps, Some(15));
        assert_eq!(config.presets["github"].max_size_mb, Some(10.0));
        assert_eq!(config.presets["mini"].width, Some(480));
        assert_eq!(config.presets["mini"].fps, Some(12));
        assert_eq!(config.presets["mini"].max_size_mb, Some(5.0));
    }

    #[test]
    fn rejects_invalid_preset_type() {
        assert!(toml::from_str::<Config>("[github]\nwidth = 'wide'").is_err());
    }
}
