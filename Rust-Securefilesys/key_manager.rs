use anyhow::{bail, Context, Result};
use chacha20poly1305::{KeyInit, XChaCha20Poly1305};
use rand_core::OsRng;
use rand_core::RngCore;
use std::fs::OpenOptions;
use std::io::Write;
use std::{fs, path::Path};
use zeroize::Zeroize;

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
            let data =
                fs::read(path).with_context(|| format!("reading key from {}", path.display()))?;
            if data.len() != 32 {
                bail!(
                    "expected 32-byte key at {} but found {} bytes",
                    path.display(),
                    data.len()
                );
            }
            let mut arr = [0u8; 32];
            arr.copy_from_slice(&data);
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
