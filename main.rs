mod config;
mod crypto;
mod storage;
mod util;

use anyhow::Result;
use crypto::key_manager::KeyManager;
use storage::file_ops::SecureFileOps;

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
