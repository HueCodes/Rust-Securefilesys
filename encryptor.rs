use aes_gcm::{aead::{Aead, AeadCore, OsRng}, Aes256Gcm, KeyInit, Nonce};
use anyhow::Result;

/// Encrypt and decrypt data buffers.
pub struct Encryptor {
    cipher: Aes256Gcm,
}

impl Encryptor {
    pub fn new(cipher: Aes256Gcm) -> Self {
        Self { cipher }
    }

    pub fn encrypt(&self, plaintext: &[u8]) -> Result<Vec<u8>> {
        let nonce = Aes256Gcm::generate_nonce(&mut OsRng); // 96-bit
        let mut ct = self.cipher.encrypt(&nonce, plaintext)?;
        let mut out = nonce.to_vec();
        out.append(&mut ct);
        Ok(out)
    }

    pub fn decrypt(&self, ciphertext: &[u8]) -> Result<Vec<u8>> {
        let (nonce_bytes, data) = ciphertext.split_at(12);
        let nonce = Nonce::from_slice(nonce_bytes);
        Ok(self.cipher.decrypt(nonce, data)?)
    }
}
