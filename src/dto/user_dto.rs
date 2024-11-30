use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;

use crate::domain::user::{User, UserId};

#[derive(FromRow, Deserialize, Serialize)]
pub struct CreateRequest {
    pub username: String,
    pub email: String,
    pub password: String,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub date_of_birth: Option<NaiveDate>,
}

#[derive(FromRow, Deserialize, Serialize)]
pub struct UpdateRequest {
    pub username: String,
    pub email: String,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub date_of_birth: Option<NaiveDate>,
}

#[derive(Deserialize, Serialize)]
pub struct FindAllResponse {
    pub data: Vec<User>,
}

#[derive(Deserialize, Serialize)]
pub struct CreateResponse {
    pub data: UserId,
}

#[derive(Deserialize, Serialize)]
pub struct FindByIdResponse {
    pub data: User,
}

#[derive(Deserialize, Serialize)]
pub struct DeleteResponse {
    pub message: String,
}
