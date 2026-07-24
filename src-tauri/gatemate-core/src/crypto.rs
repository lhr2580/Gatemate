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

fn get_machine_guid() -> String {
    #[cfg(windows)]
    {
        use std::process::Command;
        if let Ok(output) = Command::new("reg")
            .args(["query", "HKLM\\SOFTWARE\\Microsoft\\Cryptography", "/v", "MachineGuid"])
            .output()
        {
            if let Ok(output_str) = String::from_utf8(output.stdout) {
                for line in output_str.lines() {
                    if let Some(guid) = line.split_whitespace().last() {
                        return guid.to_string();
                    }
                }
            }
        }
    }
    "default_machine_guid".to_string()
}

fn encrypt_key_for_file(key: &[u8]) -> Result<Vec<u8>, CryptoError> {
    let machine_guid = get_machine_guid();
    let derived_key = derive_key_from_string(&machine_guid);
    encrypt_bytes(key, &derived_key)
}

fn decrypt_key_from_file(encrypted: &[u8]) -> Result<Vec<u8>, CryptoError> {
    let machine_guid = get_machine_guid();
    let derived_key = derive_key_from_string(&machine_guid);
    decrypt_bytes(encrypted, &derived_key)
}

fn derive_key_from_string(input: &str) -> Vec<u8> {
    use sha2::{Sha256, Digest};
    let mut hasher = Sha256::new();
    hasher.update(input.as_bytes());
    hasher.finalize().to_vec()
}

fn encrypt_bytes(data: &[u8], key: &[u8]) -> Result<Vec<u8>, CryptoError> {
    let key_arr: [u8; 32] = key.try_into().map_err(|_| CryptoError::InvalidKeyLength)?;
    let cipher = Aes256Gcm::new(Key::<aes_gcm::Aes256Gcm>::from_slice(&key_arr));
    let nonce = Aes256Gcm::generate_nonce(&mut rand::thread_rng());
    let ciphertext = cipher.encrypt(&nonce, data)
        .map_err(|e| CryptoError::EncryptionError(format!("{}", e)))?;
    
    let mut result = Vec::with_capacity(12 + ciphertext.len());
    result.extend_from_slice(&nonce);
    result.extend_from_slice(&ciphertext);
    Ok(result)
}

fn decrypt_bytes(encrypted: &[u8], key: &[u8]) -> Result<Vec<u8>, CryptoError> {
    if encrypted.len() < 12 {
        return Err(CryptoError::DataTooShort);
    }
    let key_arr: [u8; 32] = key.try_into().map_err(|_| CryptoError::InvalidKeyLength)?;
    let (nonce_bytes, ciphertext) = encrypted.split_at(12);
    let cipher = Aes256Gcm::new(Key::<aes_gcm::Aes256Gcm>::from_slice(&key_arr));
    let nonce = Nonce::from_slice(nonce_bytes);
    cipher.decrypt(nonce, ciphertext)
        .map_err(|e| CryptoError::DecryptionError(format!("{}", e)))
}

pub fn get_master_key() -> Vec<u8> {
    let app_data_dir = crate::get_app_data_dir();
    let secure_key_path = app_data_dir.join("master_key_secure.bin");
    let legacy_key_path = app_data_dir.join(MASTER_KEY_FILE);
    
    if let Ok(entry) = keyring::Entry::new(KEYRING_SERVICE, KEYRING_USERNAME) {
        if let Ok(key_b64) = entry.get_password() {
            if let Ok(key) = general_purpose::STANDARD.decode(&key_b64) {
                if key.len() == 32 {
                    return key;
                }
            }
        }
    }
    
    if secure_key_path.exists() {
        if let Ok(encrypted) = fs::read(&secure_key_path) {
            if let Ok(key) = decrypt_key_from_file(&encrypted) {
                if key.len() == 32 {
                    return key;
                }
            }
        }
    }
    
    let key = if legacy_key_path.exists() {
        match fs::read(&legacy_key_path) {
            Ok(mut key) if key.len() == 32 => {
                eprintln!("⚠️ 警告: 检测到旧版明文密钥文件，正在迁移到安全存储...");
                let _ = fs::remove_file(&legacy_key_path);
                let key_to_return = key.clone();
                key.zeroize();
                key_to_return
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
    
    let mut stored = false;
    
    if let Ok(entry) = keyring::Entry::new(KEYRING_SERVICE, KEYRING_USERNAME) {
        let key_b64 = general_purpose::STANDARD.encode(&key);
        if entry.set_password(&key_b64).is_ok() {
            stored = true;
        }
    }
    
    if !stored {
        if let Ok(encrypted) = encrypt_key_for_file(&key) {
            if fs::write(&secure_key_path, &encrypted).is_ok() {
                stored = true;
                eprintln!("⚠️ 警告: 系统密钥链不可用，主密钥已加密存储到文件");
            }
        }
    }
    
    if !stored {
        eprintln!("⚠️ 严重警告: 主密钥仅存储在内存中，重启后将重新生成，所有加密数据将无法解密");
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
    let key_arr: [u8; 32] = key.try_into().map_err(|_| CryptoError::InvalidKeyLength)?;
    
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
    
    let key_arr: [u8; 32] = key.try_into().map_err(|_| CryptoError::InvalidKeyLength)?;
    
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