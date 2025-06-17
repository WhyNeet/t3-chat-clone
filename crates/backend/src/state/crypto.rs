use std::env;

use aes_gcm::{AeadCore, Aes256Gcm, Key, aead::Aead};
use anyhow::Context;
use argon2::{
    Argon2, PasswordHash, PasswordVerifier,
    password_hash::{PasswordHasher, SaltString, rand_core::OsRng},
};
use hmac::{Hmac, Mac, digest::generic_array::GenericArray};
use sha2::Sha256;

pub type HmacSha256 = Hmac<Sha256>;

pub struct CryptoState {
    key_encryption_secret: Box<[u8]>,
    session_signing_secret: Box<[u8]>,
}

impl CryptoState {
    pub fn new() -> anyhow::Result<Self> {
        Ok(Self {
            key_encryption_secret: hex::decode(
                env::var("KEY_ENCRYPTION_SECRET")
                    .context("Missing key encryption secret")?
                    .as_bytes()
                    .to_vec()
                    .into_boxed_slice(),
            )?
            .into_boxed_slice(),
            session_signing_secret: env::var("SESSION_SECRET_KEY")
                .context("Missing session secret key")?
                .as_bytes()
                .to_vec()
                .into_boxed_slice(),
        })
    }
}

impl CryptoState {
    pub fn hash_password(&self, password: &[u8]) -> anyhow::Result<String> {
        let salt = SaltString::generate(&mut OsRng);
        let argon2 = Argon2::default();
        let password = argon2
            .hash_password(password, &salt)
            .map(|h| h.to_string())
            .map_err(|e| anyhow::anyhow!(e))?;

        Ok(password)
    }

    pub fn verify_password(&self, hash: &str, password: &[u8]) -> anyhow::Result<bool> {
        let hash = PasswordHash::new(hash).unwrap();
        Ok(Argon2::default().verify_password(password, &hash).is_ok())
    }

    pub fn encrypt_key(&self, plaintext: &[u8]) -> anyhow::Result<String> {
        use aes_gcm::KeyInit;

        let key = Key::<Aes256Gcm>::from_slice(&self.key_encryption_secret);
        let cipher = Aes256Gcm::new(&key);
        let nonce = Aes256Gcm::generate_nonce(&mut OsRng);
        let ciphertext = cipher.encrypt(&nonce, plaintext).unwrap();
        let result = format!("{}.{}", hex::encode(nonce), hex::encode(ciphertext));

        Ok(result)
    }

    pub fn decrypt_key(&self, key: &str) -> anyhow::Result<String> {
        use aes_gcm::KeyInit;

        let (nonce, ciphertext) = key.split_once('.').unwrap();
        let nonce = hex::decode(nonce).unwrap();
        let ciphertext = hex::decode(ciphertext).unwrap();

        let key = Key::<Aes256Gcm>::from_slice(&self.key_encryption_secret);
        let cipher = Aes256Gcm::new(&key);
        let plaintext = cipher
            .decrypt(GenericArray::from_slice(&nonce), ciphertext.as_ref())
            .unwrap();

        Ok(String::from_utf8(plaintext)?)
    }

    pub fn sign_session(&self, input: &[u8]) -> anyhow::Result<String> {
        let mut mac = HmacSha256::new_from_slice(&self.session_signing_secret)?;
        mac.update(input);

        let signature = mac.finalize().into_bytes();
        let signature = hex::encode(signature);

        Ok(signature)
    }

    pub fn verify_session_signature(&self, input: &[u8], signature: &[u8]) -> anyhow::Result<bool> {
        let mut mac = HmacSha256::new_from_slice(&self.session_signing_secret).unwrap();

        mac.update(input);
        let signature = hex::decode(signature)?;

        Ok(mac.verify_slice(&signature).is_ok())
    }
}
