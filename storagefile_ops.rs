use crate::encryptor::Encryptor;
use crate::key_manager::KeyManager;
use crate::metadata::FileMetadata;
use anyhow::{Context, Result};
use tokio::fs;
use std::path::PathBuf;

pub struct SecureFileOps {
    encryptor: Encryptor,
    root: PathBuf,
}

impl SecureFileOps {
    pub fn new(km: KeyManager) -> Self {
        Self {
            encryptor: Encryptor::new(km.cipher()),
            root: PathBuf::from("storage"),
        }
    }

    pub async fn write_encrypted(&self, name: &str, data: &[u8]) -> Result<()> {
        fs::create_dir_all(&self.root).await?;
        let path = self.root.join(name);
    let enc = self.encryptor.encrypt(data, None)?;
        fs::write(&path, &enc).await?;
        FileMetadata::record(&path, data.len() as u64).await?;
        Ok(())
    }

    pub async fn read_encrypted(&self, name: &str) -> Result<Vec<u8>> {
        let path = self.root.join(name);
        let data = fs::read(&path).await
            .with_context(|| format!("reading {:?}", &path))?;
    self.encryptor.decrypt(&data, None)
    }
}
