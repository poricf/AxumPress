use serde::Deserialize;
use std::{fs, io};
use toml::from_str;

/// Represents the configuration loaded from config.toml
#[derive(Debug, Deserialize)]
pub struct Config {
    pub theme: String,
    pub template: String,
    pub home_template: String,
    pub code_theme: String,
}

/// Reads and parses the config.toml file into a Config struct
pub fn read_config(config_path: &str) -> io::Result<Config> {
    // Read the entire config file as a string
    let config_contents = fs::read_to_string(config_path)?;

    // Parse the TOML string into a Config struct
    let config: Config = from_str(&config_contents)
        .map_err(|parse_error| io::Error::new(io::ErrorKind::InvalidData, parse_error.to_string()))?;

    Ok(config)
}