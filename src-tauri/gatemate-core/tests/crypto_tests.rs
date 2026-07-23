use gatemate_core::crypto;

#[test]
fn test_encrypt_decrypt_roundtrip() {
    let key = crypto::generate_random_key();
    assert_eq!(key.len(), 32);
    
    let plaintext = "Hello, GateMate!";
    let encrypted = crypto::encrypt(plaintext, &key).unwrap();
    
    let decrypted = crypto::decrypt(&encrypted, &key).unwrap();
    assert_eq!(decrypted, plaintext);
}

#[test]
fn test_encrypt_with_short_key() {
    let short_key = vec![0u8; 16];
    let result = crypto::encrypt("test", &short_key);
    assert!(result.is_err());
    assert!(matches!(result.err(), Some(crypto::CryptoError::InvalidKeyLength)));
}

#[test]
fn test_decrypt_with_wrong_key() {
    let key1 = crypto::generate_random_key();
    let key2 = crypto::generate_random_key();
    
    let encrypted = crypto::encrypt("test", &key1).unwrap();
    let result = crypto::decrypt(&encrypted, &key2);
    assert!(result.is_err());
}

#[test]
fn test_decrypt_invalid_base64() {
    let key = crypto::generate_random_key();
    let result = crypto::decrypt("invalid!!!", &key);
    assert!(result.is_err());
    assert!(matches!(result.err(), Some(crypto::CryptoError::Base64DecodeError(_))));
}

#[test]
fn test_decrypt_short_data() {
    let key = crypto::generate_random_key();
    let short_data = "YWJjZGVm";
    let result = crypto::decrypt(short_data, &key);
    assert!(result.is_err());
    assert!(matches!(result.err(), Some(crypto::CryptoError::DataTooShort)));
}

#[test]
fn test_empty_string() {
    let key = crypto::generate_random_key();
    
    let encrypted = crypto::encrypt("", &key).unwrap();
    let decrypted = crypto::decrypt(&encrypted, &key).unwrap();
    assert_eq!(decrypted, "");
}

#[test]
fn test_special_characters() {
    let key = crypto::generate_random_key();
    let plaintext = "测试中文!@#$%^&*()_+-=[]{}|;':\",./<>?";
    
    let encrypted = crypto::encrypt(plaintext, &key).unwrap();
    let decrypted = crypto::decrypt(&encrypted, &key).unwrap();
    assert_eq!(decrypted, plaintext);
}