use aes_gcm::{Aes256Gcm, KeyInit, Nonce, aead::Aead};
use anyhow::{Context, Result, bail};
use base64::{Engine as _, engine::general_purpose::STANDARD};
use rand::RngCore;

const KEY_ENV: &str = "BULLASTRATOR_ENCRYPTION_KEY";
const PREFIX: &str = "enc:v1:";
const NONCE_SIZE: usize = 12;

pub fn is_encrypted(value: &str) -> bool {
    value.starts_with(PREFIX)
}

fn cipher() -> Result<Aes256Gcm> {
    let encoded_key = std::env::var(KEY_ENV)
        .with_context(|| format!("{KEY_ENV} is required to encrypt connection passwords"))?;
    let key = STANDARD
        .decode(encoded_key)
        .with_context(|| format!("{KEY_ENV} must be a base64-encoded 32-byte key"))?;
    if key.len() != 32 {
        bail!("{KEY_ENV} must decode to exactly 32 bytes")
    }
    Aes256Gcm::new_from_slice(&key).context("failed to initialize encryption cipher")
}

pub fn encrypt(value: Option<String>) -> Result<Option<String>> {
    let Some(value) = value else { return Ok(None) };
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

pub fn decrypt(value: Option<String>) -> Result<Option<String>> {
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
    let cipher = cipher()?;
    let plaintext = cipher
        .decrypt(
            Nonce::from_slice(&payload[..NONCE_SIZE]),
            &payload[NONCE_SIZE..],
        )
        .map_err(|_| anyhow::anyhow!("failed to decrypt connection password"))?;
    Ok(Some(
        String::from_utf8(plaintext).context("decrypted connection password is not UTF-8")?,
    ))
}
