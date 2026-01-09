use argon2::{Argon2, Params, Algorithm, Version};
use rand::{rngs::OsRng, RngCore};

/// Length of the randomly generated salt in bytes.
///
/// A unique salt ensures that identical passwords derive
/// different keys and protects against rainbow table attacks.
pub const SALT_LEN: usize = 16;

/// Derives a fixed-size cryptographic key from a user-provided password.
///
/// This function uses **Argon2id**, which is currently the recommended
/// password-based key derivation algorithm due to its resistance to:
/// - GPU attacks
/// - ASIC attacks
/// - Side-channel leaks
///
/// Inputs:
/// - `password`: user secret (not stored, only used transiently)
/// - `salt`: per-file random value to guarantee uniqueness
///
/// Output:
/// - 32-byte key suitable for AES-256 or ChaCha20-Poly1305
pub fn derive_key(password: &str, salt: &[u8]) -> [u8; 32] {
    // Output buffer that will be filled by the KDF
    let mut key = [0u8; 32];

    // Argon2 configuration:
    // - memory cost: 19 MB (slows down brute-force attempts)
    // - iterations: 2 passes over memory
    // - parallelism: single-threaded (predictable and portable)
    // - output length: 32 bytes
    let params = Params::new(
        19456, // memory cost (in KiB)
        2,     // number of iterations
        1,     // degree of parallelism
        Some(32),
    ).expect("Invalid Argon2 params");

    // Explicitly select Argon2id variant and version
    let argon2 = Argon2::new(
        Algorithm::Argon2id,
        Version::V0x13,
        params,
    );

    // Derive the key directly into the output buffer
    argon2
        .hash_password_into(password.as_bytes(), salt, &mut key)
        .expect("KDF failed");

    key
}

/// Generates a cryptographically secure random salt.
///
/// Uses the operating system's CSPRNG to ensure unpredictability.
/// A new salt should be generated for every encrypted file or password.
pub fn generate_salt() -> [u8; SALT_LEN] {
    let mut salt = [0u8; SALT_LEN];

    // Fill the buffer with secure random bytes
    OsRng.fill_bytes(&mut salt);

    salt
}
