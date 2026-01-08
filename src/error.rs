use thiserror::Error;

#[derive(Error, Debug)]
pub enum CryptoError {
    #[error("I/O error")]
    Io(#[from] std::io::Error),

    #[error("Encryption failed")]
    Encrypt,

    #[error("Decryption failed")]
    Decrypt,

    #[error("Invalid password or corrupted file")]
    AuthenticationFailed,
}
