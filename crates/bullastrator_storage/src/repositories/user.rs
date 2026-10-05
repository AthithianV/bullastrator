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

    pub async fn register(&self, name: &str, email: &str, password_hash: &str) -> Result<User> {
        let now = Utc::now().naive_utc();
        let id = Uuid::new_v4().to_string();

        sqlx::query(
            r#"
            INSERT INTO
                user (
                    id,
                    name,
                    email,
                    password_hash,
                    created_at,
                    updated_at
                ) VALUES (
                    ?,?,?,?,?,?
                )"#,
        )
        .bind(&id)
        .bind(name)
        .bind(email)
        .bind(password_hash)
        .bind(now)
        .bind(now)
        .execute(&self.pool)
        .await?;
        self.find_by_email(email)
            .await?
            .ok_or_else(|| anyhow::anyhow!("Registered user was not found"))
    }

    pub async fn find_by_email(&self, email: &str) -> Result<Option<User>> {
        Ok(sqlx::query_as::<_, User>(
            r#"
                    SELECT
                        id, name, email, password_hash, image, created_at, updated_at
                    FROM
                        user
                    WHERE
                        email = ?
                "#,
        )
        .bind(email)
        .fetch_optional(&self.pool)
        .await?)
    }

    pub async fn create_session(&self, session: &Session) -> Result<()> {
        sqlx::query(
            r#"
                INSERT INTO
                session (id, user_id, token_hash, expires_at, revoked_at, created_at)
                VALUES (?, ?, ?, ?, ?, ?)
            "#,
        )
        .bind(&session.id)
        .bind(&session.user_id)
        .bind(&session.token_hash)
        .bind(session.expires_at)
        .bind(session.revoked_at)
        .bind(session.created_at)
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    pub async fn revoke_session(&self, token_hash: &str) -> Result<bool> {
        let result = sqlx::query(
            r#"
                UPDATE
                    session
                SET
                    revoked_at = ?
                WHERE
                    token_hash = ? AND revoked_at IS NULL
            "#,
        )
        .bind(Utc::now().naive_utc())
        .bind(token_hash)
        .execute(&self.pool)
        .await?;
        Ok(result.rows_affected() > 0)
    }

    pub async fn get_user_id(&self, token_hash: &str) -> Result<Option<String>> {
        Ok(sqlx::query_scalar(
            r#"
            SELECT user_id
            FROM sessions
            WHERE token_hash = ?
              AND revoked_at IS NULL
              AND expires_at > CURRENT_TIMESTAMP
            "#,
        )
        .bind(token_hash)
        .fetch_optional(&self.pool)
        .await?)
    }
}
