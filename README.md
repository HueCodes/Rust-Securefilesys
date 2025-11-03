# Rust-Securefilesys

A secure file system implementation in Rust, providing encrypted storage, access controls, and integrity checks for sensitive data management.

## Features

- **Encryption**: AES-256-GCM for file contents and metadata.
- **Access Control**: Role-based permissions with fine-grained policies.
- **Integrity Verification**: HMAC-SHA256 for tamper detection.
- **Performance**: Optimized for concurrent reads/writes using Tokio.
- **Cross-Platform**: Supports Linux, macOS, and Windows.

## Installation

1. Ensure Rust is installed: `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`
2. Clone the repository: `git clone https://github.com/HueCodes/Rust-Securefilesys.git`
3. Build the project: `cd Rust-Securefilesys && cargo build --release`

## Usage

### Quick Start

Initialize a secure file system:

```rust
use securefilesys::SecureFS;

let mut fs = SecureFS::new("path/to/mount", "your-master-key").expect("Initialization failed");
fs.create_file("secret.txt", b"Confidential data").expect("File creation failed");
```

### API Examples

- **Read File**:
  ```rust
  let content = fs.read_file("secret.txt").expect("Read failed");
  println!("{:?}", content);
  ```

- **List Directory**:
  ```rust
  let entries = fs.list_dir("/").expect("List failed");
  for entry in entries {
      println!("{}", entry.name);
  }
  ```

Refer to `src/lib.rs` for full API documentation.

## Building and Testing

- Run tests: `cargo test`
- Run benchmarks: `cargo bench`
- Generate docs: `cargo doc --open`

## Contributing

1. Fork the repository.
2. Create a feature branch: `git checkout -b feature/YourFeature`.
3. Commit changes: `git commit -m "Add YourFeature"`.
4. Push to branch: `git push origin feature/YourFeature`.
5. Open a Pull Request.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.
