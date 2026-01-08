use argon2::{Argon2, Params, Algorithm, Version};
use rand::{rngs::OsRng, RngCore};

pub const SALT_LEN: usize = 16;

pub fn derive_key(password: &str, salt: &[u8]) -> [u8; 32] {
    let mut key = [0u8; 32];

    let params = Params::new(
        19456, // memory cost (19 MB)
        2,     // iterations
        1,     // parallelism
        Some(32),
    ).expect("Invalid Argon2 params");

    let argon2 = Argon2::new(
        Algorithm::Argon2id,
        Version::V0x13,
        params,
    );

    argon2
        .hash_password_into(password.as_bytes(), salt, &mut key)
        .expect("KDF failed");

    key
}

pub fn generate_salt() -> [u8; SALT_LEN] {
    let mut salt = [0u8; SALT_LEN];
    OsRng.fill_bytes(&mut salt);
    salt
}
