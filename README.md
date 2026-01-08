# CLI File Encryptor / Decryptor (`rcrypt`)

A secure command-line tool written in **Rust** for encrypting and
decrypting files using modern cryptography best practices.

This project serves both as a **learning reference** and a
**production-style CLI utility**.

------------------------------------------------------------------------

## Features

-   AES-256-GCM authenticated encryption
-   Password-based key derivation using Argon2
-   Secure random salt and nonce generation
-   Binary-safe file encryption
-   Clean CLI interface using `clap`
-   Explicit error handling
-   Cross-platform (Windows, macOS, Linux)

------------------------------------------------------------------------

## Security Design

Component          Implementation
  ------------------ ----------------------------------
Cipher             AES-256-GCM (AEAD)
Key Derivation     Argon2
Salt               16 bytes (random per file)
Nonce              12 bytes (random per encryption)
Integrity          Built-in via GCM authentication
Password Storage   Never stored or logged

Encrypted file layout:

    [salt (16 bytes)] [nonce (12 bytes)] [ciphertext (...)] 

------------------------------------------------------------------------

## Installation

### Prerequisites

-   Rust 1.70+
-   Cargo (included with Rust)

### Build from source

``` bash
git clone https://github.com/your-org/rcrypt.git
cd rcrypt
cargo build --release
```

Binary location:

    target/release/rcrypt

------------------------------------------------------------------------

## Usage

### General syntax

``` bash
rcrypt <COMMAND> [OPTIONS]
```

Commands:

-   `encrypt` --- Encrypt a file
-   `decrypt` --- Decrypt a file

------------------------------------------------------------------------

## Encrypting a File

``` bash
rcrypt encrypt \
  --input plaintext.txt \
  --output encrypted.bin \
  --password "MyStrongPassword123"
```

### Arguments

Flag           Description
  -------------- --------------------------
`--input`      Path to plaintext file
`--output`     Path to encrypted output
`--password`   Encryption password

------------------------------------------------------------------------

## Decrypting a File

``` bash
rcrypt decrypt \
  --input encrypted.bin \
  --output decrypted.txt \
  --password "MyStrongPassword123"
```

### Notes

-   Password must match exactly
-   Authentication failure indicates wrong password or file tampering
-   Corrupted files will not decrypt

------------------------------------------------------------------------

## Exit Codes

Code   Meaning
  ------ --------------------------------
`0`    Success
`1`    Runtime or cryptographic error
`2`    Invalid CLI usage

------------------------------------------------------------------------

## Example Workflow

``` bash
echo "Secret data" > secret.txt

rcrypt encrypt \
  --input secret.txt \
  --output secret.enc \
  --password strongPass!

rcrypt decrypt \
  --input secret.enc \
  --output recovered.txt \
  --password strongPass!
```

------------------------------------------------------------------------

## Development

Run in development mode:

``` bash
cargo run -- encrypt --input file.txt --output file.enc --password test
```

Quality checks:

``` bash
cargo fmt
cargo clippy
cargo test
```

------------------------------------------------------------------------

## Project Structure

    src/
    ├── main.rs        # Application entry point
    ├── cli.rs         # CLI argument parsing
    ├── error.rs       # Unified error handling
    ├── crypto/
    │   ├── cipher.rs  # AES-GCM encryption/decryption
    │   └── kdf.rs     # Argon2 key derivation
    ├── io/
    │   └── file.rs    # File I/O helpers
    └── metadata.rs    # Encrypted file format

------------------------------------------------------------------------

## Limitations

-   Entire file is loaded into memory
-   Not optimized for extremely large files
-   Password-based encryption only

------------------------------------------------------------------------

## Roadmap

-   Streaming encryption
-   ChaCha20-Poly1305 support
-   Public-key encryption
-   Secure memory locking
-   File format versioning

------------------------------------------------------------------------

## Disclaimer

This tool is provided for **educational and general-purpose use**.
Review cryptographic tools carefully before using them in sensitive
environments.
