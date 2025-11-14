use chacha20poly1305::{XChaCha20Poly1305, KeyInit};
use rand_core::OsRng;
use rand_core::RngCore;
use std::{fs, path::Path};
use anyhow::Result;
use zeroize::Zeroize;
use std::io::Write;
use std::fs::OpenOptions;

/// Handles key generation and persistence.
/// In production: prefer a hardware key store or OS keyring.
pub struct KeyManager {
    pub key_bytes: [u8; 32],
}

impl Drop for KeyManager {
    fn drop(&mut self) {
        self.key_bytes.zeroize();
    }
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
            let mut key = [0u8; 32];
            OsRng.fill_bytes(&mut key);

            // Write with restrictive permissions where possible
            #[cfg(unix)]
            {
                use std::os::unix::fs::OpenOptionsExt;
                let mut f = OpenOptions::new()
                    .write(true)
                    .create_new(true)
                    .mode(0o600)
                    .open(path)?;
                f.write_all(&key)?;
            }
            #[cfg(not(unix))]
            {
                fs::write(path, &key)?;
            }

            key
        };

        Ok(Self { key_bytes })
    }

    pub fn cipher(&self) -> XChaCha20Poly1305 {
        XChaCha20Poly1305::new_from_slice(&self.key_bytes).expect("valid key")
    }
}
