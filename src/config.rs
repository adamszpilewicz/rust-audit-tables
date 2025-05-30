use serde::Deserialize;
use std::fs;
use anyhow::Result;
use chrono::{DateTime, Utc};

#[derive(Debug, Deserialize)]
pub struct Config {
    pub database: DatabaseConfig,
    pub batch: BatchConfig,
    pub output: OutputConfig,
}

#[derive(Debug, Deserialize)]
pub struct DatabaseConfig {
    pub host: String,
    pub port: u16,
    pub user: String,
    pub password: String,
    pub dbname: String,
}

#[derive(Debug, Deserialize)]
pub struct BatchConfig {
    pub since: DateTime<Utc>,     // <- parse as chrono datetime
    pub max_rows: usize,
}

#[derive(Debug, Deserialize)]
pub struct OutputConfig {
    pub directory: String,
    pub partition: bool,
    pub format: String,
}


pub fn load_config(path: &str) -> Result<Config> {
    let content = fs::read_to_string(path)?;
    let cfg = serde_yaml::from_str(&content)?;
    Ok(cfg)
}
