use std::error::Error;

use crate::application::repositories::user_repository::UserRepository;
use crate::domain::user::{User, UserId};
use crate::dto::user_dto::{CreateRequest, UpdateRequest};
use async_trait::async_trait;
use sqlx::postgres::PgQueryResult;
use sqlx::PgPool;
use uuid::Uuid;

pub struct PostgresUserRepository {
    pool: PgPool,
}

impl PostgresUserRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl UserRepository for PostgresUserRepository {
    async fn find_by_id(&self, user_id: Uuid) -> Result<Option<User>, Box<dyn Error>> {
        let result = sqlx::query_as!(
            User,
            "
            SELECT id, username, email, first_name, last_name, date_of_birth FROM users
            WHERE id = $1
            ",
            user_id
        )
        .fetch_one(&self.pool)
        .await;

        match result {
            Ok(user) => Ok(Some(user)),
            Err(sqlx::Error::RowNotFound) => Ok(None),
            Err(err) => Err(Box::new(err)),
        }
    }

    async fn find_all(&self) -> Result<Vec<User>, Box<dyn Error>> {
        let results = sqlx::query_as!(
            User,
            "
            SELECT id, username, email, first_name, last_name, date_of_birth FROM users
            ",
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(results)
    }

    async fn create(&self, user: &CreateRequest) -> Result<UserId, Box<dyn Error>> {
        let mut tx = self.pool.begin().await?;

        let result = sqlx::query_as!(
            UserId,
            "
            INSERT INTO users (username, email, password_hash, first_name, last_name, date_of_birth)
            VALUES ($1, $2, $3, $4, $5, $6)
            RETURNING id
            ",
            user.username,
            user.email,
            user.password,
            user.first_name,
            user.last_name,
            user.date_of_birth
        )
        .fetch_one(&mut *tx)
        .await?;

        tx.commit().await?;

        Ok(result)
    }

    async fn update(
        &self,
        user_id: Uuid,
        data: &UpdateRequest,
    ) -> Result<Option<User>, Box<dyn Error>> {
        let mut tx = self.pool.begin().await?;

        let result = sqlx::query_as!(
            User,
            "
            UPDATE users 
            SET username = $1,
                email = $2,
                first_name = $3,
                last_name = $4,
                date_of_birth = $5
            WHERE id = $6
            RETURNING id, username, email, first_name, last_name, date_of_birth
            ",
            data.username,
            data.email,
            data.first_name,
            data.last_name,
            data.date_of_birth,
            user_id
        )
        .fetch_one(&mut *tx)
        .await;

        tx.commit().await?;

        match result {
            Ok(user) => Ok(Some(user)),
            Err(sqlx::Error::RowNotFound) => Ok(None),
            Err(err) => Err(Box::new(err)),
        }
    }

    async fn delete(&self, user_id: Uuid) -> Result<PgQueryResult, Box<dyn Error>> {
        let mut tx = self.pool.begin().await?;

        let result = sqlx::query_as!(
            User,
            "
            DELETE FROM users
            WHERE id = $1
            ",
            user_id
        )
        .execute(&mut *tx)
        .await?;

        tx.commit().await?;

        Ok(result)
    }
}

#[cfg(test)]
mod tests {
    use std::env;

    use super::*;
    use crate::config::DatabaseConfig;
    use crate::infrastructure::postgres_database::PostgresDatabase;
    use tokio;

    async fn setup_database() -> PgPool {
        let database_url = env::var("TEST_DATABASE_URL").expect("TEST_DATABASE_URL must be set");
        let config = DatabaseConfig::new(database_url);
        let db = PostgresDatabase::new(config).await;

        let migrator = sqlx::migrate!("./migrations");
        migrator.run(&db.pool).await.unwrap();

        db.pool
    }

    async fn reset_test_db(pool: &PgPool) {
        sqlx::query("DELETE FROM users")
            .execute(pool)
            .await
            .unwrap();
    }

    #[tokio::test]
    async fn create_new_user() {
        let pool = setup_database().await;
        reset_test_db(&pool).await;
        let repo = PostgresUserRepository::new(pool.clone());

        let new_user = CreateRequest {
            username: "testuser".to_string(),
            email: "test@example.com".to_string(),
            password: "hashed_password".to_string(),
            first_name: None,
            last_name: None,
            date_of_birth: None,
        };

        let result = repo.create(&new_user).await;
        assert!(result.is_ok());

        reset_test_db(&pool).await;
    }

    #[tokio::test]
    async fn find_all() {
        let pool = setup_database().await;
        reset_test_db(&pool).await;

        let repo = PostgresUserRepository::new(pool.clone());

        let users = repo.find_all().await.unwrap();
        assert!(users.is_empty());

        let new_user = CreateRequest {
            username: "testuser".to_string(),
            email: "test@example.com".to_string(),
            password: "hashed_password".to_string(),
            first_name: None,
            last_name: None,
            date_of_birth: None,
        };

        repo.create(&new_user).await.unwrap();

        let users = repo.find_all().await.unwrap();
        assert_eq!(users.len(), 1);

        reset_test_db(&pool).await;
    }

    #[tokio::test]
    async fn find_one() {
        let pool = setup_database().await;
        reset_test_db(&pool).await;
        let repo = PostgresUserRepository::new(pool.clone());

        let new_user = CreateRequest {
            username: "testuser".to_string(),
            email: "test@example.com".to_string(),
            password: "hashed_password".to_string(),
            first_name: None,
            last_name: None,
            date_of_birth: None,
        };

        let created_user_id = repo.create(&new_user).await.unwrap();

        let result = repo.find_by_id(created_user_id.id).await;
        assert!(result.is_ok());

        reset_test_db(&pool).await;
    }

    #[tokio::test]
    async fn update() {
        let pool = setup_database().await;
        reset_test_db(&pool).await;
        let repo = PostgresUserRepository::new(pool.clone());

        let new_user = CreateRequest {
            username: "testuser".to_string(),
            email: "test@example.com".to_string(),
            password: "hashed_password".to_string(),
            first_name: None,
            last_name: None,
            date_of_birth: None,
        };

        let created_user_id = repo.create(&new_user).await.unwrap();

        let update_user_request = UpdateRequest {
            username: new_user.username,
            email: new_user.email,
            first_name: Some("Test".to_string()),
            last_name: Some("User".to_string()),
            date_of_birth: None,
        };

        let result = repo.update(created_user_id.id, &update_user_request).await;
        assert!(result.is_ok());

        let updated_user = result.unwrap().unwrap();
        assert_eq!(update_user_request.first_name, updated_user.first_name);
        assert_eq!(update_user_request.last_name, updated_user.last_name);

        reset_test_db(&pool).await;
    }

    #[tokio::test]
    async fn delete() {
        let pool = setup_database().await;
        reset_test_db(&pool).await;
        let repo = PostgresUserRepository::new(pool.clone());

        let new_user = CreateRequest {
            username: "testuser".to_string(),
            email: "test@example.com".to_string(),
            password: "hashed_password".to_string(),
            first_name: None,
            last_name: None,
            date_of_birth: None,
        };

        let created_user_id = repo.create(&new_user).await.unwrap();

        let result = repo.delete(created_user_id.id).await;
        assert!(result.is_ok());

        let affected_rows = result.unwrap().rows_affected();
        assert_eq!(1, affected_rows);

        reset_test_db(&pool).await;
    }
}
