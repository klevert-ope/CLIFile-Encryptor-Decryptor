pub struct EncryptedFile {
    pub salt: [u8; 16],
    pub nonce: [u8; 12],
    pub ciphertext: Vec<u8>,
}

impl EncryptedFile {
    pub fn serialize(&self) -> Vec<u8> {
        let mut result = Vec::with_capacity(16 + 12 + self.ciphertext.len());
        result.extend_from_slice(&self.salt);
        result.extend_from_slice(&self.nonce);
        result.extend_from_slice(self.ciphertext.as_slice());
        result
    }

    pub fn deserialize(bytes: &[u8]) -> Result<Self, &'static str> {
        if bytes.len() < 28 {
            return Err("Input too short for salt and nonce");
        }
        let salt = bytes[0..16].try_into().map_err(|_| "Failed to parse salt")?;
        let nonce = bytes[16..28].try_into().map_err(|_| "Failed to parse nonce")?;
        let ciphertext = bytes[28..].to_vec();

        Ok(Self { salt, nonce, ciphertext })
    }
}
