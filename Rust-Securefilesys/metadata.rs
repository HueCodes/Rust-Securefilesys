use serde::{Serialize, Deserialize};
use std::path::Path;
use tokio::fs;
use anyhow::Result;

#[derive(Debug, Serialize, Deserialize)]
pub struct FileMetadata {
    pub filename: String,
    pub size: u64,
}

impl FileMetadata {
    pub async fn record(path: &Path, size: u64) -> Result<()> {
        let meta = Self {
            filename: path.file_name().unwrap().to_string_lossy().into_owned(),
            size,
        };
        let json = serde_json::to_string_pretty(&meta)?;
        let meta_path = path.with_extension("meta.json");
        fs::write(meta_path, json).await?;
        Ok(())
    }
}
