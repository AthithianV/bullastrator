use anyhow::Result;
use chrono::Utc;
use sqlx::SqlitePool;
use uuid::Uuid;

use crate::models::{Session, User};

#[derive(Clone)]
pub struct UserRepository {
    pool: SqlitePool,
}

impl UserRepository {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }

    pub async fn upsert(
        &self,
        id: &str,
        name: &str,
        email: &str,
        image: Option<&str>,
    ) -> Result<User> {
        let now = Utc::now().naive_utc();
        sqlx::query("INSERT INTO user (id, name, email, password_hash, image, created_at, updated_at) VALUES (?, ?, ?, NULL, ?, ?, ?) ON CONFLICT(id) DO UPDATE SET name = excluded.name, email = excluded.email, image = excluded.image, updated_at = excluded.updated_at")
            .bind(if id.is_empty() { Uuid::new_v4().to_string() } else { id.to_string() }).bind(name).bind(email).bind(image).bind(now).bind(now).execute(&self.pool).await?;
        Ok(sqlx::query_as::<_, User>(
            "SELECT id, name, email, password_hash, image, created_at, updated_at FROM user WHERE email = ?",
        )
        .bind(email)
        .fetch_one(&self.pool)
        .await?)
    }

    pub async fn get_current_user(&self) -> Result<Option<User>> {
        Ok(sqlx::query_as::<_, User>("SELECT id, name, email, password_hash, image, created_at, updated_at FROM user ORDER BY created_at LIMIT 1").fetch_optional(&self.pool).await?)
    }

    pub async fn delete_current_user(&self) -> Result<()> {
        sqlx::query("DELETE FROM user").execute(&self.pool).await?;
        Ok(())
    }

    pub async fn register(&self, name: &str, email: &str, password_hash: &str) -> Result<User> {
        let now = Utc::now().naive_utc();
        let id = Uuid::new_v4().to_string();
        sqlx::query("INSERT INTO user (id, name, email, password_hash, created_at, updated_at) VALUES (?, ?, ?, ?, ?, ?)")
            .bind(&id).bind(name).bind(email).bind(password_hash).bind(now).bind(now)
            .execute(&self.pool).await?;
        self.find_by_email(email)
            .await?
            .ok_or_else(|| anyhow::anyhow!("Registered user was not found"))
    }

    pub async fn find_by_email(&self, email: &str) -> Result<Option<User>> {
        Ok(sqlx::query_as::<_, User>("SELECT id, name, email, password_hash, image, created_at, updated_at FROM user WHERE email = ?")
            .bind(email).fetch_optional(&self.pool).await?)
    }

    pub async fn create_session(&self, session: &Session) -> Result<()> {
        sqlx::query("INSERT INTO session (id, user_id, token_hash, expires_at, revoked_at, created_at) VALUES (?, ?, ?, ?, ?, ?)")
            .bind(&session.id).bind(&session.user_id).bind(&session.token_hash)
            .bind(session.expires_at).bind(session.revoked_at).bind(session.created_at)
            .execute(&self.pool).await?;
        Ok(())
    }

    pub async fn revoke_session(&self, token_hash: &str) -> Result<bool> {
        let result = sqlx::query(
            "UPDATE session SET revoked_at = ? WHERE token_hash = ? AND revoked_at IS NULL",
        )
        .bind(Utc::now().naive_utc())
        .bind(token_hash)
        .execute(&self.pool)
        .await?;
        Ok(result.rows_affected() > 0)
    }
}
