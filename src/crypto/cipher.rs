use aes_gcm::{Aes256Gcm, Key, Nonce};
use aes_gcm::aead::{Aead, KeyInit};
use rand::{rngs::OsRng, RngCore};

/// Length of the nonce in bytes for AES-GCM.
///
/// AES-GCM requires a 96-bit (12-byte) nonce for optimal security
/// and performance. Nonces must be **unique per key**.
pub const NONCE_LEN: usize = 12;

/// Encrypts plaintext data using AES-256-GCM.
///
/// AES-GCM is an authenticated encryption algorithm, meaning:
/// - Confidentiality: data is encrypted
/// - Integrity: tampering is detected
/// - Authenticity: decryption fails if the key/nonce is incorrect
///
/// Returns:
/// - `ciphertext`: encrypted output (includes authentication tag)
/// - `nonce`: randomly generated per-encryption value
pub fn encrypt(
    data: &[u8],
    key_bytes: &[u8; 32],
) -> Result<(Vec<u8>, [u8; NONCE_LEN]), ()> {
    // Initialize AES-256-GCM with the derived 256-bit key
    let key = Key::<Aes256Gcm>::from_slice(key_bytes);
    let cipher = Aes256Gcm::new(key);

    // Generate a fresh, cryptographically secure nonce
    // A nonce must never be reused with the same key
    let mut nonce_bytes = [0u8; NONCE_LEN];
    OsRng.fill_bytes(&mut nonce_bytes);

    // Convert raw nonce bytes into the AEAD nonce type
    let nonce = Nonce::from_slice(&nonce_bytes);

    // Perform authenticated encryption
    // The authentication tag is appended to the ciphertext internally
    let ciphertext = cipher
        .encrypt(nonce, data)
        .map_err(|_| ())?;

    Ok((ciphertext, nonce_bytes))
}

/// Decrypts AES-256-GCM encrypted data.
///
/// Decryption will fail if:
/// - The key is incorrect
/// - The nonce does not match the encryption nonce
/// - The ciphertext has been modified (authentication failure)
///
/// Returns the original plaintext on success.
pub fn decrypt(
    ciphertext: &[u8],
    key_bytes: &[u8; 32],
    nonce_bytes: &[u8; NONCE_LEN],
) -> Result<Vec<u8>, ()> {
    // Reconstruct AES-256-GCM cipher using the same key
    let key = Key::<Aes256Gcm>::from_slice(key_bytes);
    let cipher = Aes256Gcm::new(key);

    // Reconstruct the nonce used during encryption
    let nonce = Nonce::from_slice(nonce_bytes);

    // Perform authenticated decryption
    cipher
        .decrypt(nonce, ciphertext)
        .map_err(|_| ())
}
