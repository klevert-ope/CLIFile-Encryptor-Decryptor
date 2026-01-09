use thiserror::Error;

/// Represents all errors that can occur in the encryption/decryption process.
#[derive(Error, Debug)]
pub enum CryptoError {
    /// Wrapper around standard I/O errors (file read/write failures)
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),

    /// Encryption operation failed
    #[error("Encryption failed")]
    Encrypt,

    /// Decryption operation failed
    #[error("Decryption failed")]
    Decrypt,

    /// Authentication failed (wrong password or tampered file)
    #[error("Invalid password or corrupted file")]
    AuthenticationFailed,

    /// User-provided passwords did not match during interactive encryption
    #[error("Passwords do not match")]
    PasswordMismatch,
}
