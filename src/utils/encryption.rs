use ring::aead::{Aad, BoundKey, Nonce, NonceSequence, OpeningKey, SealingKey, UnboundKey, AES_256_GCM, NONCE_LEN};
use ring::error::Unspecified;
use ring::rand::{SecureRandom, SystemRandom};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum EncryptionError {
    #[error("Encryption failed")]
    EncryptionFailed,
    #[error("Decryption failed")]
    DecryptionFailed,
    #[error("Invalid key length")]
    InvalidKeyLength,
    #[error("Ring error: {0}")]
    RingError(#[from] Unspecified),
}

pub struct EncryptionService {
    key: Vec<u8>,
    rng: SystemRandom,
}

impl EncryptionService {
    pub fn new() -> Result<Self, EncryptionError> {
        let key_str = std::env::var("ENCRYPTION_KEY")
            .expect("ENCRYPTION_KEY must be set in environment");
        
        let key = hex::decode(key_str)
            .map_err(|_| EncryptionError::InvalidKeyLength)?;
        
        if key.len() != 32 {
            return Err(EncryptionError::InvalidKeyLength);
        }

        Ok(Self {
            key,
            rng: SystemRandom::new(),
        })
    }

    pub fn encrypt(&self, plaintext: &str) -> Result<String, EncryptionError> {
        let mut nonce_bytes = [0u8; NONCE_LEN];
        self.rng.fill(&mut nonce_bytes)
            .map_err(EncryptionError::RingError)?;

        let nonce = Nonce::assume_unique_for_key(nonce_bytes);
        let unbound_key = UnboundKey::new(&AES_256_GCM, &self.key)
            .map_err(EncryptionError::RingError)?;
        
        let mut sealing_key = SealingKey::new(unbound_key, OneNonceSequence(Some(nonce)));
        
        let mut in_out = plaintext.as_bytes().to_vec();
        sealing_key.seal_in_place_append_tag(Aad::empty(), &mut in_out)
            .map_err(EncryptionError::RingError)?;

        // Prepend nonce to the encrypted data
        let mut result = nonce_bytes.to_vec();
        result.extend_from_slice(&in_out);
        
        Ok(hex::encode(result))
    }

    pub fn decrypt(&self, ciphertext_hex: &str) -> Result<String, EncryptionError> {
        let ciphertext = hex::decode(ciphertext_hex)
            .map_err(|_| EncryptionError::DecryptionFailed)?;

        if ciphertext.len() < NONCE_LEN {
            return Err(EncryptionError::DecryptionFailed);
        }

        let (nonce_bytes, encrypted_data) = ciphertext.split_at(NONCE_LEN);
        let nonce = Nonce::try_assume_unique_for_key(nonce_bytes)
            .map_err(EncryptionError::RingError)?;

        let unbound_key = UnboundKey::new(&AES_256_GCM, &self.key)
            .map_err(EncryptionError::RingError)?;
        
        let mut opening_key = OpeningKey::new(unbound_key, OneNonceSequence(Some(nonce)));
        
        let mut in_out = encrypted_data.to_vec();
        let plaintext = opening_key.open_in_place(Aad::empty(), &mut in_out)
            .map_err(EncryptionError::RingError)?;

        String::from_utf8(plaintext.to_vec())
            .map_err(|_| EncryptionError::DecryptionFailed)
    }
}

struct OneNonceSequence(Option<Nonce>);

impl NonceSequence for OneNonceSequence {
    fn advance(&mut self) -> Result<Nonce, Unspecified> {
        self.0.take().ok_or(Unspecified)
    }
}

// Global encryption service instance
use std::sync::OnceLock;
static ENCRYPTION_SERVICE: OnceLock<EncryptionService> = OnceLock::new();

pub fn get_encryption_service() -> &'static EncryptionService {
    ENCRYPTION_SERVICE.get_or_init(|| {
        EncryptionService::new().expect("Failed to initialize encryption service")
    })
}

pub fn encrypt_text(plaintext: &str) -> Result<String, EncryptionError> {
    get_encryption_service().encrypt(plaintext)
}

pub fn decrypt_text(ciphertext: &str) -> Result<String, EncryptionError> {
    get_encryption_service().decrypt(ciphertext)
} 