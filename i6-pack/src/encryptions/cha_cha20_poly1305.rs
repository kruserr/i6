use chacha20poly1305::{
  aead::{Aead, AeadCore, KeyInit, OsRng}, Nonce,
};

use argon2::{self, password_hash::SaltString, Argon2, PasswordHasher};

use hmac::digest::{generic_array::GenericArray, typenum};
use rand::RngCore;
use std::fs::File;
use std::io::{self, Write};

use super::encryption::{self, Encryption};

const SALT_LEN: usize = 16;
const NONCE_LEN: usize = 12;

fn generate_salt() -> [u8; SALT_LEN] {
  let mut salt = [0u8; SALT_LEN];
  rand::thread_rng().fill_bytes(&mut salt);
  salt
}

fn derive_key_from_password_argon2(password: &str, salt: &[u8]) -> [u8; 32] {
  let argon2 = Argon2::default();
  let salt = SaltString::encode_b64(salt).unwrap();
  let password_hash = argon2.hash_password(password.as_bytes(), &salt).unwrap();
  let key = password_hash.hash.unwrap();
  let mut key_bytes = [0u8; 32];
  key_bytes.copy_from_slice(key.as_bytes());
  key_bytes
}

pub struct ChaCha20Poly1305;

impl Encryption for ChaCha20Poly1305 {
  fn encrypt_file(
    input_file: &str,
    output_file: &str,
    password: &str,
  ) -> io::Result<()> {
    let salt = generate_salt();
    let key = derive_key_from_password_argon2(password, &salt);
    let cipher = chacha20poly1305::ChaCha20Poly1305::new((&key).into());
    let nonce = chacha20poly1305::ChaCha20Poly1305::generate_nonce(&mut OsRng);

    let file_content = std::fs::read(input_file)?;
    let ciphertext = cipher
      .encrypt(&nonce, file_content.as_ref())
      .map_err(|_| io::Error::new(io::ErrorKind::Other, "Encryption failure"))?;

    let mut output = File::create(output_file)?;
    output.write_all(&salt)?; // Prepend salt
    output.write_all(nonce.as_slice())?; // Prepend nonce
    output.write_all(&ciphertext)?;

    Ok(())
  }

  fn decrypt_file(
    input_file: &str,
    output_file: &str,
    password: &str,
  ) -> io::Result<()> {
    let file_content = std::fs::read(input_file)?;
    let (salt_and_nonce, ciphertext) =
      file_content.split_at(SALT_LEN + NONCE_LEN); // Extract salt and nonce
    let (salt, nonce) = salt_and_nonce.split_at(SALT_LEN); // Extract salt

    let key = derive_key_from_password_argon2(password, salt);
    let cipher = chacha20poly1305::ChaCha20Poly1305::new((&key).into());

    let nonce = GenericArray::from_slice(nonce);
    let plaintext = match cipher.decrypt(nonce, ciphertext) {
      Ok(pt) => pt,
      Err(_) => {
        return Err(io::Error::new(io::ErrorKind::Other, "Decryption failure"))
      }
    };

    let mut output = File::create(output_file)?;
    output.write_all(&plaintext)?;
    Ok(())
  }
}
