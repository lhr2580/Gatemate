use aes_gcm::{Aes256Gcm, Key, KeyInit, Nonce};
use aes_gcm::aead::{Aead, AeadCore};
use base64::{Engine as _, engine::general_purpose};
use rand::RngCore;
use std::fs;
use zeroize::Zeroize;

const KEYRING_SERVICE: &str = "gatemate";
const KEYRING_USERNAME: &str = "master_key";
const MASTER_KEY_FILE: &str = "master_key.bin";

pub fn generate_random_key() -> Vec<u8> {
    let mut key = vec![0u8; 32];
    rand::thread_rng().fill_bytes(&mut key);
    key
}

pub fn get_master_key() -> Vec<u8> {
    if let Ok(entry) = keyring::Entry::new(KEYRING_SERVICE, KEYRING_USERNAME) {
        match entry.get_password() {
            Ok(key_b64) => {
                if let Ok(key) = general_purpose::STANDARD.decode(&key_b64) {
                    if key.len() == 32 {
                        return key;
                    }
                }
            }
            Err(_) => {}
        }
    }
    
    let app_data_dir = crate::get_app_data_dir();
    let key_path = app_data_dir.join(MASTER_KEY_FILE);
    
    let key = if key_path.exists() {
        match fs::read(&key_path) {
            Ok(key) if key.len() == 32 => {
                let _ = fs::remove_file(&key_path);
                key
            }
            Ok(mut key) => {
                key.zeroize();
                generate_random_key()
            }
            Err(_) => generate_random_key(),
        }
    } else {
        generate_random_key()
    };
    
    if let Ok(entry) = keyring::Entry::new(KEYRING_SERVICE, KEYRING_USERNAME) {
        let key_b64 = general_purpose::STANDARD.encode(&key);
        let _ = entry.set_password(&key_b64);
    }
    
    key
}

#[derive(Debug)]
pub enum CryptoError {
    InvalidKeyLength,
    EncryptionError(String),
    DecryptionError(String),
    Base64DecodeError(String),
    Utf8DecodeError(String),
    DataTooShort,
}

impl std::fmt::Display for CryptoError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CryptoError::InvalidKeyLength => write!(f, "key length must be 32 bytes"),
            CryptoError::EncryptionError(e) => write!(f, "encryption failed: {}", e),
            CryptoError::DecryptionError(e) => write!(f, "decryption failed: {}", e),
            CryptoError::Base64DecodeError(e) => write!(f, "base64 decode failed: {}", e),
            CryptoError::Utf8DecodeError(e) => write!(f, "UTF-8 decode failed: {}", e),
            CryptoError::DataTooShort => write!(f, "data too short"),
        }
    }
}

impl std::error::Error for CryptoError {}

pub fn encrypt(data: &str, key: &[u8]) -> Result<String, CryptoError> {
    if key.len() != 32 {
        return Err(CryptoError::InvalidKeyLength);
    }
    
    let key_arr: [u8; 32] = key.try_into().unwrap();
    
    let cipher = Aes256Gcm::new(Key::<aes_gcm::Aes256Gcm>::from_slice(&key_arr));
    let nonce = Aes256Gcm::generate_nonce(&mut rand::thread_rng());
    
    let ciphertext = cipher.encrypt(&nonce, data.as_bytes())
        .map_err(|e| CryptoError::EncryptionError(format!("{}", e)))?;
    
    let mut result = Vec::with_capacity(12 + ciphertext.len());
    result.extend_from_slice(&nonce);
    result.extend_from_slice(&ciphertext);
    
    Ok(general_purpose::STANDARD.encode(&result))
}

pub fn decrypt(encoded: &str, key: &[u8]) -> Result<String, CryptoError> {
    let data = general_purpose::STANDARD.decode(encoded)
        .map_err(|e| CryptoError::Base64DecodeError(format!("{}", e)))?;
    
    if data.len() < 12 {
        return Err(CryptoError::DataTooShort);
    }
    
    if key.len() != 32 {
        return Err(CryptoError::InvalidKeyLength);
    }
    
    let key_arr: [u8; 32] = key.try_into().unwrap();
    
    let (nonce_bytes, ciphertext) = data.split_at(12);
    let cipher = Aes256Gcm::new(Key::<aes_gcm::Aes256Gcm>::from_slice(&key_arr));
    
    let nonce = Nonce::from_slice(nonce_bytes);
    let mut plaintext = cipher.decrypt(nonce, ciphertext)
        .map_err(|e| CryptoError::DecryptionError(format!("{}", e)))?;
    
    let result = String::from_utf8(plaintext.clone())
        .map_err(|e| CryptoError::Utf8DecodeError(format!("{}", e)));
    
    plaintext.zeroize();
    
    result
}