use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::fs;

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub key_path: String,
    pub storage_dir: String,
}

impl Config {
    pub fn load(path: &str) -> Result<Self> {
        let s =
            fs::read_to_string(path).with_context(|| format!("reading config file {}", path))?;
        Ok(serde_json::from_str(&s)?)
    }
}
