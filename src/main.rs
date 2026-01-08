mod cli;
mod crypto;
mod error;
mod io;
mod metadata;

use clap::Parser;
use cli::{Cli, Commands};
use crypto::{cipher, kdf};
use metadata::EncryptedFile;

fn main() -> Result<(), error::CryptoError> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Encrypt { input, output, password } => {
            let data = io::file::read_file(&input)?;
            let salt = kdf::generate_salt();
            let key = kdf::derive_key(password.as_str(), &salt);

            let (ciphertext, nonce) = cipher::encrypt(&data, &key).map_err(|_| error::CryptoError::Encrypt)?;

            let encrypted = EncryptedFile { salt, nonce, ciphertext };
            io::file::write_file(&output, &encrypted.serialize())?;
        }

        Commands::Decrypt { input, output, password } => {
            let bytes = io::file::read_file(&input)?;
            let encrypted = EncryptedFile::deserialize(bytes.as_slice())
                .map_err(|_| error::CryptoError::Decrypt)?;

            let key = kdf::derive_key(&password, &encrypted.salt);
            let plaintext = cipher::decrypt(
                encrypted.ciphertext.as_slice(),
                &key,
                &encrypted.nonce,
            )
                .map_err(|_| error::CryptoError::AuthenticationFailed)?;

            io::file::write_file(&output, &plaintext)?;
        }
    }

    Ok(())
}
