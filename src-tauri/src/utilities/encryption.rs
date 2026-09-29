use aes_gcm::aead::Aead;
use aes_gcm::{Aes256Gcm, Key, KeyInit, Nonce};
use machine_uid;
use rand::RngCore;
use rand::rngs::OsRng;
use sha2::{Digest, Sha256};

pub fn get_master_key() -> [u8; 32] {
    // Get the unique ID of this computer (e.g., UUID of the motherboard)
    let uid = machine_uid::get().unwrap_or_else(|_| "fallback-unique-string".to_string());

    // Hash it so it's exactly 32 bytes and looks like random data
    let mut hasher = Sha256::new();
    hasher.update(uid.as_bytes());
    hasher.update(b"bullastrator_salt");

    let result = hasher.finalize();
    let mut key = [0u8; 32];
    key.copy_from_slice(&result);
    key
}

pub fn encrypt_password(password: &str) -> Result<String, String> {
    let master_key = get_master_key();
    let key = Key::<Aes256Gcm>::from_slice(&master_key);
    let cipher = Aes256Gcm::new(key);

    let mut nonce_bytes = [0u8; 12];
    OsRng.fill_bytes(&mut nonce_bytes);
    let nonce = Nonce::from_slice(&nonce_bytes);

    let ciphertext = cipher
        .encrypt(nonce, password.as_bytes())
        .map_err(|e| e.to_string())?;

    // Combine Nonce + Ciphertext and encode to Base64 (or Hex) for DB storage
    let mut combined = nonce_bytes.to_vec();
    combined.extend(ciphertext);
    Ok(hex::encode(combined))
}

pub fn decrypt_password(hex_data: &str) -> Result<String, String> {
    let combined = hex::decode(hex_data).map_err(|e| e.to_string())?;
    let (nonce_bytes, ciphertext) = combined.split_at(12);

    let master_key = get_master_key();
    let key = Key::<Aes256Gcm>::from_slice(&master_key);
    let cipher = Aes256Gcm::new(key);
    let nonce = Nonce::from_slice(nonce_bytes);

    let plaintext = cipher
        .decrypt(nonce, ciphertext)
        .map_err(|e| e.to_string())?;

    String::from_utf8(plaintext).map_err(|e| e.to_string())
}
