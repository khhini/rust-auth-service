use std::error::Error;

use uuid::Uuid;

use crate::{
    application::repositories::user_repository::UserRepository,
    config::PwdConfig,
    domain::user::{User, UserId},
    dto::user_dto::{CreateRequest, UpdateRequest},
    util::pwd::Pwd,
};

pub struct UserUseCase<R: UserRepository> {
    repository: R,
}

impl<R: UserRepository> UserUseCase<R> {
    pub fn new(repository: R) -> Self {
        Self { repository }
    }

    pub async fn create(
        &self,
        mut new_user: CreateRequest,
        pwd_cfg: PwdConfig,
    ) -> Result<UserId, Box<dyn Error>> {
        let pwd = Pwd::new(&pwd_cfg);
        new_user.password = pwd
            .generate_password_hash(new_user.password.as_str())
            .unwrap();

        self.repository.create(&new_user).await
    }

    pub async fn find_all(&self) -> Result<Vec<User>, Box<dyn Error>> {
        self.repository.find_all().await
    }

    pub async fn find_by_id(&self, user_id: Uuid) -> Result<Option<User>, Box<dyn Error>> {
        self.repository.find_by_id(user_id).await
    }

    pub async fn update(
        &self,
        user_id: Uuid,
        data: UpdateRequest,
    ) -> Result<Option<User>, Box<dyn Error>> {
        self.repository.update(user_id, &data).await
    }

    pub async fn delete(&self, user_id: Uuid) -> Result<u64, Box<dyn Error>> {
        match self.repository.delete(user_id).await {
            Ok(result) => Ok(result.rows_affected()),
            Err(err) => Err(err)
        }
    }
}
