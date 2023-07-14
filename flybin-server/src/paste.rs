use axum::http::StatusCode;
use sqlx::{FromRow, Pool, Sqlite};

use crate::error::AppError;

#[derive(FromRow, Debug)]
pub struct Paste {
    slug: String,
    pub content: String,
    created_at: chrono::NaiveDateTime,
    expires_at: chrono::NaiveDateTime,
    secret: String,
    ip_address: String,

    #[allow(dead_code)]
    password: Option<Vec<u8>>,
}

impl Paste {
    pub fn new(content: String, ip_address: String) -> Self {
        let slug = nanoid::nanoid!(6);
        let created_at = chrono::Local::now().naive_local();
        let expires_at = compute_expiry_date(&content);
        let secret = nanoid::nanoid!(16);
        let password = None;
        Self {
            slug,
            content,
            created_at,
            expires_at,
            secret,
            ip_address,
            password,
        }
    }

    pub fn get_response_str(&self) -> String {
        format!(
            r#"
Url: http://{}:8080/{}
Secret: {}
Expires at: {}
"#,
            dotenvy::var("HOST").unwrap(),
            self.slug,
            self.secret,
            self.expires_at
        )
    }

    pub async fn save(&self, pool: &Pool<Sqlite>) {
        sqlx::query!(
            r#"
                INSERT INTO pastes (slug, content, created_at, expires_at, secret, ip_address) 
                VALUES ($1, $2, $3, $4, $5, $6)
            "#,
            self.slug,
            self.content,
            self.created_at,
            self.expires_at,
            self.secret,
            self.ip_address,
        )
        .execute(pool)
        .await
        .unwrap();
    }

    pub async fn lock(
        slug: &str,
        secret: String,
        password: String,
        pool: &Pool<Sqlite>,
    ) -> Result<(), AppError> {
        let password = blake3::hash(password.as_bytes()).to_string();
        let rows_affected = sqlx::query!(
            r#"
                UPDATE pastes 
                SET password = $1 
                WHERE slug = $2 AND secret = $3
            "#,
            password,
            slug,
            secret,
        )
        .execute(pool)
        .await?
        .rows_affected();

        if rows_affected == 0 {
            Err(AppError(
                StatusCode::NOT_FOUND,
                "unable to lock paste".to_string(),
            ))
        } else {
            Ok(())
        }
    }

    pub async fn get(
        slug: String,
        password: Option<String>,
        pool: &Pool<Sqlite>,
    ) -> Result<Self, sqlx::Error> {
        let password = password.map(|password| blake3::hash(password.as_bytes()).to_string());
        sqlx::query_as!(
            Paste,
            r#"
                SELECT *
                FROM pastes
                WHERE slug = $1 AND ( password IS NULL  or password = $2 ) 
            "#,
            slug,
            password
        )
        .fetch_one(pool)
        .await
    }

    pub async fn delete(slug: &str, secret: String, pool: &Pool<Sqlite>) -> Result<(), AppError> {
        let rows_deleted = sqlx::query!(
            "DELETE FROM pastes WHERE slug = $1 AND secret = $2",
            slug,
            secret,
        )
        .execute(pool)
        .await?;

        if rows_deleted.rows_affected() == 0 {
            return Err(AppError(
                StatusCode::UNAUTHORIZED,
                "unable to delete paste".to_string(),
            ));
        }

        Ok(())
    }
}

/// Retention calculation based on [0x0.st](https://0x0.st)
fn compute_expiry_date(content: &str) -> chrono::NaiveDateTime {
    let min_age: i64 = 30;
    let max_age: i64 = 365;

    let retention =
        min_age + (-max_age + min_age) * (f64::powi((content.len() as f64) / 4096. - 1., 3)) as i64;
    chrono::Local::now().naive_local() + chrono::Duration::days(retention)
}
