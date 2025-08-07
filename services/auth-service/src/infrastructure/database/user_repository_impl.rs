use crate::domain::entities::user::User;
use crate::domain::repositories::user_repository::UserRepository;
use async_trait::async_trait;
use shared::features::errors::{SystemError, SystemResult};
use sqlx::{Pool, Postgres, Row};
use uuid::Uuid;

pub struct PostgresUserRepository {
    pool: Pool<Postgres>,
}

impl PostgresUserRepository {
    pub fn new(pool: Pool<Postgres>) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl UserRepository for PostgresUserRepository {
    async fn create(&self, user: &User) -> SystemResult<User> {
        log::info!("create() called with user: {:?}", user);

        // Insert the user into the database
        let row = match sqlx::query(
            "INSERT INTO users (
                id, email, phone_number, password_hash, role,
                is_verified, is_active, failed_login_attempts, locked_until,
                last_login, created_at, updated_at
            ) VALUES (
                $1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12
            ) RETURNING id"
        )
        .bind(&user.id)
        .bind(&user.email)
        .bind(&user.phone_number)
        .bind(&user.password_hash)
        .bind(user.role.to_string())
        .bind(&user.is_verified)
        .bind(&user.is_active)
        .bind(&user.failed_login_attempts)
        .bind(&user.locked_until)
        .bind(&user.last_login_at)
        .bind(&user.created_at)
        .bind(&user.updated_at)
        .fetch_one(&self.pool)
        .await
        {
            Ok(row) => row,
            Err(e) => {
                log::error!("Database query error: {}", e);
                return Err(SystemError::DatabaseError(format!("Query error: {}", e)));
            }
        };

        let id: Uuid = row.get("id");
        self.find_by_id(&id).await?.ok_or(SystemError::DatabaseError("User not found after creation".into()))
    }

    async fn find_by_id(&self, id: &Uuid) -> SystemResult<Option<User>> {
        log::info!("find_by_id() called with id: {}", id);

        // Query to fetch the user by id
        let row = match sqlx::query(
            "SELECT id, email, phone_number, password_hash, role,
                is_verified, is_active, failed_login_attempts, locked_until,
                last_login_at, created_at, updated_at
            FROM users 
            WHERE id = $1",
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await
        {
            Ok(row) => row,
            Err(e) => {
                log::error!("Database query error at fetch_optional: {}", e);
                return Err(SystemError::DatabaseError(format!("Query error: {}", e)));
            }
        };

        if let Some(row) = row {
            let user = User{
                id: row.get(0),
                email: row.get(1),
                phone_number: row.get(2),
                password_hash: row.get(3),
                role: row.get::<String, _>(4).parse().map_err(|_| SystemError::ValidationError("Invalid role value".into()))?,
                is_verified: row.get(5),
                is_active: row.get(6),
                failed_login_attempts: row.get(7),
                locked_until: row.get(8),
                last_login_at: row.get(9),
                created_at: row.get(10),
                updated_at: row.get(11),
            };
            return Ok(Some(user));
        }

        Ok(None)
    }

    async fn find_by_email(&self, email: &str) -> SystemResult<Option<User>> {
        log::info!("find_by_email() called with email: {}", email);

        // Query to fetch the user by email
        let row = match sqlx::query(
            "SELECT id, email, phone_number, password_hash, role,
                is_verified, is_active, failed_login_attempts, locked_until,
                last_login_at, created_at, updated_at
            FROM users
            WHERE email = $1",
        )
            .bind(email)
            .fetch_optional(&self.pool)
            .await
            {
                Ok(row) => row,
                Err(e) => {
                    log::error!("Database query error at fetch_optional: {}", e);
                    return Err(SystemError::DatabaseError(format!("Query error: {}", e)));
                }
            };

        if let Some(row) = row {
            let user = User{
                id: row.get(0),
                email: row.get(1),
                phone_number: row.get(2),
                password_hash: row.get(3),
                role: row.get::<String, _>(4).parse().map_err(|_| SystemError::ValidationError("Invalid role value".into()))?,
                is_verified: row.get(5),
                is_active: row.get(6),
                failed_login_attempts: row.get(7),
                locked_until: row.get(8),
                last_login_at: row.get(9),
                created_at: row.get(10),
                updated_at: row.get(11),
            };
            return Ok(Some(user));
        }

        Ok(None)
    }

    async fn find_by_phone(&self, phone: &str) -> SystemResult<Option<User>> {
        log::info!("find_by_phone() called with phone: {}", phone);

        // Query to fetch the user by id
        let row = sqlx::query(
            "SELECT id, email, phone_number, password_hash, role,
                is_verified, is_active, failed_login_attempts, locked_until,
                last_login_at, created_at, updated_at
            FROM users
            WHERE phone_number = $1",
        )
            .bind(phone)
            .fetch_optional(&self.pool)
            .await?;

        if let Some(row) = row {
            let user = User{
                id: row.get(0),
                email: row.get(1),
                phone_number: row.get(2),
                password_hash: row.get(3),
                role: row.get::<String, _>(4).parse().map_err(|_| SystemError::ValidationError("Invalid role value".into()))?,
                is_verified: row.get(5),
                is_active: row.get(6),
                failed_login_attempts: row.get(7),
                locked_until: row.get(8),
                last_login_at: row.get(9),
                created_at: row.get(10),
                updated_at: row.get(11),
            };
            return Ok(Some(user));
        }

        Ok(None)
    }

    async fn update(&self, user: &User) -> SystemResult<User> {
        log::info!("update() called with user: {:?}", user);

        // Update the user in the database
        let result = sqlx::query(
            "UPDATE users SET
                email = $1,
                phone_number = $2,
                is_verified = $3,
                is_active = $4,
                failed_login_attempts = $5,
                locked_until = $6,
                last_login_at = $7
            WHERE id = $8"
        )
        .bind(&user.email)
        .bind(&user.phone_number)
        .bind(&user.is_verified)
        .bind(&user.is_active)
        .bind(&user.failed_login_attempts)
        .bind(&user.locked_until)
        .bind(&user.last_login_at)
        .bind(&user.id)
        .execute(&self.pool)
        .await;

        match result {
            Ok(_) => (),
            Err(e) => {
                log::error!("Database update error: {}", e);
                return Err(SystemError::DatabaseError(format!("Update error: {}", e)));
            }
        }

        Ok(user.clone())
    }

    async fn delete(&self, id: &Uuid) -> SystemResult<()> {
        log::info!("delete() called with id: {}", id);

        let result = sqlx::query("DELETE FROM users WHERE id = $1")
            .bind(id)
            .execute(&self.pool)
            .await;

        match result {
            Ok(_) => (),
            Err(e) => {
                log::error!("Database delete error: {}", e);
                return Err(SystemError::DatabaseError(format!("Delete error: {}", e)));
            }
        }

        Ok(())
    }

    async fn exists_by_email(&self, email: &str) -> SystemResult<bool> {
        log::info!("exists_by_email() called with email: {}", email);
        let row = sqlx::query("SELECT EXISTS(SELECT 1 FROM users WHERE email = $1)").bind(email).fetch_one(&self.pool).await?;
        Ok(row.get::<bool, _>(0))
    }

    async fn exists_by_phone(&self, phone: &str) -> SystemResult<bool> {
        log::info!("exists_by_phone() called with phone: {}", phone);
        let row = sqlx::query("SELECT EXISTS(SELECT 1 FROM users WHERE phone_number = $1)").bind(phone).fetch_one(&self.pool).await?;
        Ok(row.get::<bool, _>(0))
    }
}
