use std::error::Error;

use crate::domain::user::{User, UserId};
use crate::dto::user_dto::{CreateRequest, UpdateRequest};
use async_trait::async_trait;
use sqlx::postgres::PgQueryResult;
use uuid::Uuid;

#[async_trait]
pub trait UserRepository {
    async fn find_by_id(&self, user_id: Uuid) -> Result<Option<User>, Box<dyn Error>>;
    async fn find_all(&self) -> Result<Vec<User>, Box<dyn Error>>;
    async fn create(&self, user: &CreateRequest) -> Result<UserId, Box<dyn Error>>;
    async fn update(
        &self,
        user_id: Uuid,
        data: &UpdateRequest,
    ) -> Result<Option<User>, Box<dyn Error>>;
    async fn delete(&self, user_id: Uuid) -> Result<PgQueryResult, Box<dyn Error>>;
}
