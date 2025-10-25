use aes_gcm::aead::KeyInit;
use aes_gcm::Aes256Gcm;
use rand_chacha::ChaCha20Rng;
use rand_core::{RngCore, SeedableRng};
use std::{fs, path::Path};
use anyhow::Result;

/// Handles key generation and persistence.
/// In production: use a hardware key store or OS keyring.
pub struct KeyManager {
    pub key_bytes: [u8; 32],
}

impl KeyManager {
    pub fn new(cfg: &crate::config::Config) -> Result<Self> {
        let path = Path::new(&cfg.key_path);
        let key_bytes = if path.exists() {
            let data = fs::read(path)?;
            let mut arr = [0u8; 32];
            arr.copy_from_slice(&data[..32]);
            arr
        } else {
            let mut rng = ChaCha20Rng::from_entropy();
            let mut key = [0u8; 32];
            rng.fill_bytes(&mut key);
            fs::write(path, &key)?;
            key
        };

        Ok(Self { key_bytes })
    }

    pub fn cipher(&self) -> Aes256Gcm {
        Aes256Gcm::new_from_slice(&self.key_bytes).expect("valid key")
    }
}
