use anyhow::{Context, Result, bail};
use argon2::{
    Argon2,
    password_hash::{PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
};
use bullastrator_storage::{
    models::{Session, User},
    repositories::UserRepository,
};
use chrono::{Duration, Utc};
use rand::rngs::OsRng;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use uuid::Uuid;

const SESSION_TTL_DAYS: i64 = 30;

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RegisterRequest {
    pub name: String,
    pub email: String,
    pub password: String,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AuthResponse {
    pub user: User,
    pub token: String,
    pub expires_at: chrono::NaiveDateTime,
}

#[derive(Clone)]
pub struct UserService {
    repository: UserRepository,
}

impl UserService {
    pub fn new(repository: UserRepository) -> Self {
        Self { repository }
    }

    #[tracing::instrument(skip(self, request), err)]
    pub async fn register(&self, request: RegisterRequest) -> Result<AuthResponse> {
        tracing::info!("registering user");
        let name = request.name.trim();
        let email = normalize_email(&request.email)?;
        validate_password(&request.password)?;
        if name.is_empty() {
            bail!("Name cannot be empty");
        }
        if self.repository.find_by_email(&email).await?.is_some() {
            bail!("An account with this email already exists");
        }

        let password_hash = hash_password(&request.password)?;
        let user = self
            .repository
            .register(name, &email, &password_hash)
            .await?;
        self.create_auth_response(user).await
    }

    #[tracing::instrument(skip(self, request), err)]
    pub async fn login(&self, request: LoginRequest) -> Result<AuthResponse> {
        tracing::info!("authenticating user");
        let email = normalize_email(&request.email)?;
        let user = self
            .repository
            .find_by_email(&email)
            .await?
            .context("Invalid email or password")?;
        let password_hash = user
            .password_hash
            .as_deref()
            .context("Invalid email or password")?;
        let parsed_hash = PasswordHash::new(password_hash)
            .map_err(|_| anyhow::anyhow!("Invalid stored password hash"))?;
        Argon2::default()
            .verify_password(request.password.as_bytes(), &parsed_hash)
            .map_err(|_| anyhow::anyhow!("Invalid email or password"))?;
        self.create_auth_response(user).await
    }

    #[tracing::instrument(skip(self, token), err)]
    pub async fn logout(&self, token: &str) -> Result<bool> {
        tracing::info!("logging out user");
        if token.trim().is_empty() {
            return Ok(false);
        }
        Ok(self.repository.revoke_session(&hash_token(token)).await?)
    }

    async fn create_auth_response(&self, user: User) -> Result<AuthResponse> {
        let token = Uuid::new_v4().to_string();
        let now = Utc::now().naive_utc();
        let expires_at = now + Duration::days(SESSION_TTL_DAYS);
        self.repository
            .create_session(&Session {
                id: Uuid::new_v4().to_string(),
                user_id: user.id.clone(),
                token_hash: hash_token(&token),
                expires_at,
                revoked_at: None,
                created_at: now,
            })
            .await?;
        Ok(AuthResponse {
            user,
            token,
            expires_at,
        })
    }
}

fn normalize_email(email: &str) -> Result<String> {
    let email = email.trim().to_lowercase();
    if email.is_empty() || !email.contains('@') {
        bail!("A valid email is required");
    }
    Ok(email)
}

fn validate_password(password: &str) -> Result<()> {
    if password.chars().count() < 8 {
        bail!("Password must contain at least 8 characters");
    }
    Ok(())
}

fn hash_password(password: &str) -> Result<String> {
    let salt = SaltString::generate(&mut OsRng);
    Ok(Argon2::default()
        .hash_password(password.as_bytes(), &salt)
        .map_err(|error| anyhow::anyhow!("Failed to hash password: {error}"))?
        .to_string())
}

fn hash_token(token: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(token.as_bytes());
    hex::encode(hasher.finalize())
}
