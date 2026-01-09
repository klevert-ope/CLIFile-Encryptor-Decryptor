mod crypto;
mod error;
mod io;
mod metadata;

use crypto::{cipher, kdf};
use dialoguer::{Input, Select};
use metadata::EncryptedFile;


use rpassword::read_password;
use std::path::Path;

/// Entry point for the interactive encryption/decryption tool
fn main() -> Result<(), error::CryptoError> {
    // Present menu to the user
    let action = Select::new()
        .with_prompt("Select an action")
        .items(&["Encrypt a file", "Decrypt a file"])
        .default(0)
        .interact()
        .unwrap();

    match action {
        0 => encrypt_flow()?,
        1 => decrypt_flow()?,
        _ => unreachable!(),
    }

    Ok(())
}

/// Interactive encryption workflow
fn encrypt_flow() -> Result<(), error::CryptoError> {
    // Prompt for input file path
    let input_str: String = Input::new()
        .with_prompt("Enter path to the file to encrypt")
        .interact_text()
        .unwrap();
    let input = Path::new(&input_str);

    // Prompt for output file path
    let output_str: String = Input::new()
        .with_prompt("Enter output file path")
        .interact_text()
        .unwrap();
    let output = Path::new(&output_str);

    // Read password securely
    println!("Enter encryption password:");
    let password = read_password().unwrap();

    println!("Confirm password:");
    let confirm = read_password().unwrap();

    if password != confirm {
        return Err(error::CryptoError::PasswordMismatch);
    }

    // Read plaintext file
    let data = io::file::read_file(&input)?;

    // Generate salt and derive key
    let salt = kdf::generate_salt();
    let key = kdf::derive_key(password.as_str(), &salt);

    // Encrypt
    let (ciphertext, nonce) =
        cipher::encrypt(&data, &key).map_err(|_| error::CryptoError::Encrypt)?;

    // Bundle encrypted data and metadata
    let encrypted = EncryptedFile { salt, nonce, ciphertext };

    // Write to disk
    io::file::write_file(&output, &encrypted.serialize())?;

    println!("Encryption completed successfully.");
    Ok(())
}

/// Interactive decryption workflow
fn decrypt_flow() -> Result<(), error::CryptoError> {
    // Prompt for encrypted input file
    let input_str: String = Input::new()
        .with_prompt("Enter path to the encrypted file")
        .interact_text()
        .unwrap();
    let input = Path::new(&input_str);

    // Prompt for output file path
    let output_str: String = Input::new()
        .with_prompt("Enter output file path")
        .interact_text()
        .unwrap();
    let output = Path::new(&output_str);

    // Read decryption password
    println!("Enter decryption password:");
    let password = read_password().unwrap();

    // Read and parse encrypted file
    let bytes = io::file::read_file(&input)?;
    let encrypted = EncryptedFile::deserialize(&bytes)
        .map_err(|_| error::CryptoError::Decrypt)?;

    // Re-derive key
    let key = kdf::derive_key(password.as_str(), &encrypted.salt);

    // Decrypt
    let plaintext = cipher::decrypt(
        &encrypted.ciphertext,
        &key,
        &encrypted.nonce,
    )
        .map_err(|_| error::CryptoError::AuthenticationFailed)?;

    // Write decrypted file
    io::file::write_file(&output, &plaintext)?;

    println!("Decryption completed successfully.");
    Ok(())
}