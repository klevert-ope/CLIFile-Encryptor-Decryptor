/// Represents an encrypted file payload.
///
/// This struct contains all cryptographic material required to decrypt
/// the file later:
/// - `salt`: used during key derivation (e.g., Argon2 / PBKDF)
/// - `nonce`: required for AEAD ciphers (e.g., AES-GCM or ChaCha20-Poly1305)
/// - `ciphertext`: the encrypted file contents (including auth tag if applicable)
pub struct EncryptedFile {
    pub salt: [u8; 16],
    pub nonce: [u8; 12],
    pub ciphertext: Vec<u8>,
}

impl EncryptedFile {
    /// Serializes the encrypted file into a single contiguous byte buffer.
    ///
    /// Layout (fixed and deterministic):
    /// [ 0..16 )  -> salt
    /// [16..28 )  -> nonce
    /// [28.. ]    -> ciphertext
    ///
    /// This format allows easy storage to disk or transmission over the network.
    pub fn serialize(&self) -> Vec<u8> {
        // Pre-allocate exact capacity to avoid reallocations
        let mut result = Vec::with_capacity(16 + 12 + self.ciphertext.len());

        // Append cryptographic metadata first
        result.extend_from_slice(&self.salt);
        result.extend_from_slice(&self.nonce);

        // Append encrypted payload last
        result.extend_from_slice(self.ciphertext.as_slice());

        result
    }

    /// Deserializes a byte slice back into an `EncryptedFile`.
    ///
    /// Assumes the input bytes follow the same layout defined in `serialize`.
    /// Performs basic length validation to avoid out-of-bounds reads.
    pub fn deserialize(bytes: &[u8]) -> Result<Self, &'static str> {
        // Minimum length required to contain salt + nonce
        if bytes.len() < 28 {
            return Err("Input too short for salt and nonce");
        }

        // Extract fixed-size salt
        let salt = bytes[0..16]
            .try_into()
            .map_err(|_| "Failed to parse salt")?;

        // Extract fixed-size nonce
        let nonce = bytes[16..28]
            .try_into()
            .map_err(|_| "Failed to parse nonce")?;

        // Remaining bytes are treated as ciphertext
        let ciphertext = bytes[28..].to_vec();

        Ok(Self {
            salt,
            nonce,
            ciphertext,
        })
    }
}
