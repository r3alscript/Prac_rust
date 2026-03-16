use chrono::Utc;
use rust_decimal::Decimal;
use sqlx::PgPool;
use uuid::Uuid;

use crate::domain::user::User;

#[derive(Clone)]
pub struct UserRepository {
    pool: PgPool,
}

impl UserRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn find_by_id(&self, user_id: Uuid) -> Result<Option<User>, sqlx::Error> {
        let user = sqlx::query_as::<_, User>(
            r#"
            SELECT
                id,
                email,
                name,
                surname,
                photo_url,
                balance,
                created_at_utc
            FROM users
            WHERE id = $1
            "#,
        )
            .bind(user_id)
            .fetch_optional(&self.pool)
            .await?;

        Ok(user)
    }

    pub async fn find_by_email(&self, email: &str) -> Result<Option<User>, sqlx::Error> {
        let user = sqlx::query_as::<_, User>(
            r#"
            SELECT
                id,
                email,
                name,
                surname,
                photo_url,
                balance,
                created_at_utc
            FROM users
            WHERE email = $1
            "#,
        )
            .bind(email)
            .fetch_optional(&self.pool)
            .await?;

        Ok(user)
    }

    pub async fn create_user(
        &self,
        email: &str,
        name: &str,
        surname: &str,
        photo_url: Option<&str>,
    ) -> Result<User, sqlx::Error> {
        let user = User {
            id: Uuid::new_v4(),
            email: email.to_string(),
            name: name.to_string(),
            surname: surname.to_string(),
            photo_url: photo_url.map(|s| s.to_string()),
            balance: Decimal::new(0, 0),
            created_at_utc: Utc::now().naive_utc(),
        };

        sqlx::query(
            r#"
            INSERT INTO users (id, email, name, surname, photo_url, balance, created_at_utc)
            VALUES ($1, $2, $3, $4, $5, $6, $7)
            "#,
        )
            .bind(user.id)
            .bind(&user.email)
            .bind(&user.name)
            .bind(&user.surname)
            .bind(&user.photo_url)
            .bind(user.balance)
            .bind(user.created_at_utc)
            .execute(&self.pool)
            .await?;

        Ok(user)
    }

    pub async fn find_or_create_google_user(
        &self,
        email: &str,
        full_name: &str,
        photo_url: Option<&str>,
    ) -> Result<User, sqlx::Error> {
        if let Some(existing) = self.find_by_email(email).await? {
            return Ok(existing);
        }

        let mut parts = full_name.split_whitespace();
        let name = parts.next().unwrap_or("GoogleUser");
        let surname = parts.collect::<Vec<_>>().join(" ");

        let surname_value = if surname.trim().is_empty() {
            "User"
        } else {
            surname.trim()
        };

        self.create_user(email, name, surname_value, photo_url).await
    }
}