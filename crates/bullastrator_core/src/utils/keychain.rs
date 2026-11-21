use base64::{Engine as _, engine::general_purpose::STANDARD};
use keyring::Entry;
use rand::RngCore;

const ENCRYPTION_KEY_ENV: &str = "BULLASTRATOR_ENCRYPTION_KEY";
const KEYCHAIN_SERVICE: &str = "com.bullastrator.desktop";
const KEYCHAIN_ACCOUNT: &str = "connection-encryption-key";

pub fn initialize_encryption_key() -> Result<(), String> {
    if std::env::var_os(ENCRYPTION_KEY_ENV).is_some() {
        return Ok(());
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

    // The storage layer reads this only inside the current desktop process.
    std::env::set_var(ENCRYPTION_KEY_ENV, key);
    Ok(())
}
