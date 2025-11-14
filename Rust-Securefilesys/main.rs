mod config;
mod encryptor;
mod key_manager;
mod storagefile_ops;
mod metadata;
mod util;

use anyhow::Result;
use key_manager::KeyManager;
use storagefile_ops::SecureFileOps;

#[tokio::main]
async fn main() -> Result<()> {
    let cfg = config::Config::load("config.json")?;
    let km = KeyManager::new(&cfg)?;
    let fs = SecureFileOps::new(km);

    fs.write_encrypted("example.txt", b"Top secret data").await?;
    let data = fs.read_encrypted("example.txt").await?;

    println!("Decrypted content: {}", String::from_utf8_lossy(&data));
    Ok(())
}
