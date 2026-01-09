use std::fs;
use std::path::Path;
use crate::error::CryptoError;

/// Reads the entire contents of a file into memory.
pub fn read_file(path: &Path) -> Result<Vec<u8>, CryptoError> {
    let data = fs::read(path)?;
    Ok(data)
}

/// Writes a byte buffer to a file, replacing it if it already exists.
pub fn write_file(path: &Path, data: &[u8]) -> Result<(), CryptoError> {
    fs::write(path, data)?;
    Ok(())
}
