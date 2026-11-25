use base64::{Engine as _, engine::general_purpose::STANDARD};
use keyring::Entry;
use rand::RngCore;
use std::sync::OnceLock;

const ENCRYPTION_KEY_ENV: &str = "BULLASTRATOR_ENCRYPTION_KEY";
const KEYCHAIN_SERVICE: &str = "com.bullastrator.desktop";
const KEYCHAIN_ACCOUNT: &str = "connection-encryption-key";
static ENCRYPTION_KEY: OnceLock<String> = OnceLock::new();

pub fn initialize_encryption_key() -> Result<(), String> {
    encryption_key().map(|_| ())
}

pub fn encryption_key() -> Result<String, String> {
    if let Some(key) = ENCRYPTION_KEY.get() {
        return Ok(key.clone());
    }

    if let Ok(key) = std::env::var(ENCRYPTION_KEY_ENV) {
        return cache_key(key);
    }

    let entry = Entry::new(KEYCHAIN_SERVICE, KEYCHAIN_ACCOUNT)
        .map_err(|error| format!("Failed to access OS keychain: {error}"))?;
    let key = match entry.get_password() {
        Ok(key) => key,
        Err(keyring::Error::NoEntry) => {
            let mut bytes = [0_u8; 32];
            rand::rngs::OsRng.fill_bytes(&mut bytes);
            let key = STANDARD.encode(bytes);
            entry.set_password(&key).map_err(|error| {
                format!("Failed to store encryption key in OS keychain: {error}")
            })?;
            key
        }
        Err(error) => {
            return Err(format!(
                "Failed to read encryption key from OS keychain: {error}"
            ));
        }
    };

    cache_key(key)
}

fn cache_key(key: String) -> Result<String, String> {
    ENCRYPTION_KEY
        .set(key.clone())
        .map_err(|_| "Encryption key was initialized concurrently".to_string())?;
    Ok(key)
}
