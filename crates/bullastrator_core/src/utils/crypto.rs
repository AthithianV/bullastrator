use aes_gcm::{Aes256Gcm, KeyInit, Nonce, aead::Aead};
use anyhow::{Context, Result, bail};
use base64::{Engine as _, engine::general_purpose::STANDARD};
use rand::RngCore;

const PREFIX: &str = "enc:v1:";
const NONCE_SIZE: usize = 12;

pub fn is_encrypted(value: &str) -> bool {
    value.starts_with(PREFIX)
}

pub fn ensure_encryption(value: Option<String>) -> Result<Option<String>> {
    let Some(value) = value else { return Ok(None) };
    if value.is_empty() || is_encrypted(&value) {
        return Ok((!value.is_empty()).then_some(value));
    }
    encrypt_password(Some(value))
}

pub fn encrypt_password(value: Option<String>) -> Result<Option<String>> {
    let Some(value) = value else { return Ok(None) };
    if value.is_empty() {
        return Ok(Some(value));
    }
    let cipher = cipher()?;
    let mut nonce_bytes = [0_u8; NONCE_SIZE];
    rand::rngs::OsRng.fill_bytes(&mut nonce_bytes);
    let ciphertext = cipher
        .encrypt(Nonce::from_slice(&nonce_bytes), value.as_bytes())
        .map_err(|_| anyhow::anyhow!("failed to encrypt connection password"))?;
    let mut payload = nonce_bytes.to_vec();
    payload.extend(ciphertext);
    Ok(Some(format!("{PREFIX}{}", STANDARD.encode(payload))))
}

pub fn decrypt_password(value: Option<String>) -> Result<Option<String>> {
    let Some(value) = value else { return Ok(None) };
    let Some(encoded) = value.strip_prefix(PREFIX) else {
        return Ok(Some(value));
    };
    let payload = STANDARD
        .decode(encoded)
        .context("invalid encrypted connection password")?;
    if payload.len() <= NONCE_SIZE {
        bail!("invalid encrypted connection password")
    }
    let plaintext = cipher()?
        .decrypt(
            Nonce::from_slice(&payload[..NONCE_SIZE]),
            &payload[NONCE_SIZE..],
        )
        .map_err(|_| anyhow::anyhow!("failed to decrypt connection password"))?;
    Ok(Some(
        String::from_utf8(plaintext).context("decrypted connection password is not UTF-8")?,
    ))
}

fn cipher() -> Result<Aes256Gcm> {
    let encoded_key = crate::utils::keychain::encryption_key().map_err(anyhow::Error::msg)?;
    let key = STANDARD
        .decode(encoded_key)
        .context("encryption key must be base64 encoded")?;
    if key.len() != 32 {
        bail!("encryption key must decode to exactly 32 bytes")
    }
    Aes256Gcm::new_from_slice(&key).context("failed to initialize encryption cipher")
}
